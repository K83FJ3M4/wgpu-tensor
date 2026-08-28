use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, Weak};

use bytemuck::Contiguous;
use crossbeam_channel::Sender;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferAddress, BufferDescriptor, BufferSize, BufferUsages, COPY_BUFFER_ALIGNMENT, ComputePass, Device};
pub use shape::{Shape, IntoShape};
pub use indices::IntoIndices;
pub(super) use pool::TensorPool;
pub(super) use indices::{AllDimensions, ShapeDiff};

use crate::optimizers::{Optimizer, OptimizerConfig};
use crate::pipelines::Pipelines;
use crate::{TensorContext, TensorEncoder, TensorError};

mod indices;
mod shape;
mod pool;

#[derive(Clone)]
pub struct Tensor<'scope>(Arc<TensorInner<'scope>>);

#[derive(Clone)]
pub(crate) struct WeakTensor<'scope>(Weak<TensorInner<'scope>>);

struct TensorInner<'scope> {
    domain: TensorDomain<'scope>,
    read_bind_group: BindGroup,
    write_bind_group: BindGroup,
    buffer: Buffer,
    shape: Arc<Shape>,
}

enum TensorDomain<'scope> {
    Trainable(Box<Mutex<dyn Optimizer>>),
    Temporary(&'scope Sender<TensorInner<'static>>),
    Generic
}

impl Tensor<'static> {
    pub fn new(
        context: &mut TensorContext,
        shape: impl IntoShape
    ) -> Result<Tensor<'static>, TensorError> {
        let shape = shape.shape();
        Ok(Tensor(Arc::new(Self::create(
            &mut context.encoder_pool.pipelines(),
            shape, Self::buffer_size(shape)?,
            TensorDomain::Generic
        ))))
    }

    pub fn new_trainable(
        context: &mut TensorContext,
        config: impl OptimizerConfig,
        shape: impl IntoShape,
    ) -> Result<Tensor<'static>, TensorError> {
        let optimizer = config.build(context);
        let shape = shape.shape();
        Ok(Tensor(Arc::new(Self::create(
            &mut context.encoder_pool.pipelines(),
            shape, Self::buffer_size(shape)?,
            TensorDomain::Trainable(
                Box::new(Mutex::new(optimizer))
            )
        ))))
    } 
}

impl<'scope> Tensor<'scope> {
    const MIN_TEMPORARY_CAPACITY: BufferAddress = 256;
    const BUCKETS_PER_OCTAVE: BufferAddress = 4;

    pub(crate) fn optimize(
        self,
        encoder: &mut TensorEncoder<'scope>,
        gradients: &Tensor<'scope>
    ) -> Result<(), TensorError> {
        let TensorDomain::Trainable(optimizer) = &self.0.domain else {
            panic!("Attempted to optimize a non trainable tensor")
        };

        let Ok(mut optimizer) = optimizer.lock() else {
            panic!("Failed to acuqire optimizer lock")
        };

        optimizer.optimize(encoder, gradients, &self)
    }

    pub(crate) fn downgrade(&self) -> WeakTensor<'scope> {
        WeakTensor(Arc::downgrade(&self.0))
    }

    pub(crate) fn data_buffer(&self) -> &Buffer {
        &self.0.buffer
    }

    pub fn trainable(&self) -> bool {
        match self.0.domain {
            TensorDomain::Trainable(..) => true,
            _ => false
        }
    }

    pub fn shape(&self) -> Shape {
        *self.0.shape
    }
    
    fn create(
        pipelines: &Pipelines,
        shape: Shape,
        size: BufferSize,
        domain: TensorDomain<'scope>
    ) -> TensorInner<'scope> {
        let buffer = pipelines.device.create_buffer(
            &BufferDescriptor {
                label: None,
                size: size.get(),
                mapped_at_creation: false,
                usage: BufferUsages::COPY_DST
                    | BufferUsages::COPY_SRC
                    | BufferUsages::STORAGE
            }
        );

        let read_bind_group = Self::create_bind_group(
            &pipelines.device,
            &pipelines.tensor_input_layout,
            &buffer
        );

        let write_bind_group = Self::create_bind_group(
            &pipelines.device,
            &pipelines.tensor_output_layout, 
            &buffer
        );

        TensorInner {
            read_bind_group,
            write_bind_group,
            shape: Arc::new(shape),
            domain,
            buffer,
        }
    }  

    pub(crate) fn bind(
        &self,
        compute_pass: &mut ComputePass,
        index: u32,
        read_only: bool
    ) {
        if read_only {
            compute_pass.set_bind_group(
                index,
                &self.0.read_bind_group,
                &[]
            );
        } else {
            compute_pass.set_bind_group(
                index,
                &self.0.write_bind_group,
                &[]
            );
        }
    }

    fn create_bind_group(
        device: &Device,
        layout: &BindGroupLayout,
        buffer: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout,
            entries: &[BindGroupEntry {
                resource: buffer.as_entire_binding(),
                binding: 0,
            }]
        })
    } 

    pub(crate) fn data_size(shape: Shape) -> Result<BufferAddress, TensorError> {
        shape.into_iter()
            .map(|x| x as u64)
            .try_fold(1, u64::checked_mul)
            .ok_or(TensorError::OversizedTensor)?
            .checked_mul(size_of::<f32>() as u64)
            .ok_or(TensorError::OversizedTensor)
    }

    pub(crate) fn buffer_size(shape: Shape) -> Result<BufferSize, TensorError> {
        let alignment = BufferSize::new(COPY_BUFFER_ALIGNMENT).unwrap();
        Ok(BufferSize::new(
            Self::data_size(shape)?
                .checked_next_multiple_of(alignment.get())
                .unwrap_or(BufferSize::MAX_VALUE)
        ).unwrap_or(alignment))
    }

    fn bucket_size(shape: Shape) -> Result<BufferSize, TensorError> {
        let required = Self::buffer_size(shape)?.get()
            .max(Self::MIN_TEMPORARY_CAPACITY);

        let octave = 1u64 << ((u64::BITS - 1) - required.leading_zeros());
        let bucket_width = (octave / Self::BUCKETS_PER_OCTAVE)
            .max(COPY_BUFFER_ALIGNMENT);

        Ok(required.checked_next_multiple_of(bucket_width)
            .map(BufferSize::new).flatten()
            .unwrap_or(BufferSize::MAX))
    }
}

impl<'scope> Drop for TensorInner<'scope> {
    fn drop(&mut self) {
        if let TensorDomain::Temporary(sender) = self.domain {
            sender.send(TensorInner {
                write_bind_group: self.write_bind_group.clone(),
                read_bind_group: self.read_bind_group.clone(),
                buffer: self.buffer.clone(),
                shape: self.shape.clone(),
                domain: TensorDomain::Generic
            }).ok();
        }
    }
}

impl<'scope> Eq for WeakTensor<'scope> {}
impl<'scope> PartialEq for WeakTensor<'scope> {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.0, &other.0) 
    }
}

impl<'scope> Hash for WeakTensor<'scope> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Weak::as_ptr(&self.0).hash(state);
    }
}

impl<'scope> Eq for Tensor<'scope> {}
impl<'scope> PartialEq for Tensor<'scope> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) 
    }
}

impl<'scope> Hash for Tensor<'scope> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Arc::as_ptr(&self.0).hash(state);
    }
}
