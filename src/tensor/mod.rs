use std::sync::mpsc::Sender;

use bytemuck::{Contiguous, cast_slice};
use shaders::Shape;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferAddress, BufferBindingType, BufferDescriptor, BufferSize, BufferUsages, COPY_BUFFER_ALIGNMENT, ComputePass, Device};
pub use shape::IntoShape;
pub(super) use pool::TensorPool;

use crate::TensorContext;
use crate::pipelines::{BindGroupLayoutPool, BindingShape, BindingSize};

mod shape;
mod pool;

pub struct Tensor<'scope> {
    sender: Option<&'scope Sender<Tensor<'static>>>,
    read_bind_group: BindGroup,
    write_bind_group: BindGroup,
    shape_buffer: Buffer,
    buffer: Buffer,
    shape: Shape
}

impl Tensor<'static> {
    pub fn new(
        context: &mut TensorContext,
        shape: impl IntoShape
    ) -> Tensor<'static> {
        let shape = shape.shape();
        Self::create(
            &mut context.bind_group_layouts,
            &context.device, shape,
            Self::buffer_size(shape)
        )
    }
}

impl<'scope> Tensor<'scope> {
    const MIN_TEMPORARY_CAPACITY: BufferAddress = 256;
    const BUCKETS_PER_OCTAVE: BufferAddress = 4; 

    pub(crate) fn shape_buffer(&self) -> &Buffer {
        &self.shape_buffer
    }

    pub(crate) fn data_buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }
    
    fn create(
        bind_group_layouts: &mut BindGroupLayoutPool,
        device: &Device,
        shape: Shape,
        size: BufferSize,
    ) -> Tensor<'static> {
        let buffer = device.create_buffer(
            &BufferDescriptor {
                label: None,
                size: size.get(),
                mapped_at_creation: false,
                usage: BufferUsages::COPY_DST
                    | BufferUsages::COPY_SRC
                    | BufferUsages::STORAGE
            }
        );

        let shape_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                contents: cast_slice(shape.as_slice()),
                usage: BufferUsages::UNIFORM
                    | BufferUsages::COPY_DST,
                label: None,
            }
        );

        let read_bind_group = Self::create_bind_group(
            device, bind_group_layouts.get(&[
                BindingShape::Buffer {
                    has_dynamic_offset: false,
                    size: BindingSize::of::<f32>(),
                    ty: BufferBindingType::Storage {
                        read_only: true
                    }
                },
                BindingShape::Buffer {
                    has_dynamic_offset: false,
                    size: BindingSize::of::<Shape>(),
                    ty: BufferBindingType::Uniform
                }
            ]),
            &shape_buffer,
            &buffer
        );

        let write_bind_group = Self::create_bind_group(
            device, bind_group_layouts.get(&[
                BindingShape::Buffer {
                    has_dynamic_offset: false,
                    size: BindingSize::of::<f32>(),
                    ty: BufferBindingType::Storage {
                        read_only: false
                    }
                },
                BindingShape::Buffer {
                    has_dynamic_offset: false,
                    size: BindingSize::of::<Shape>(),
                    ty: BufferBindingType::Uniform
                }
            ]),
            &shape_buffer,
            &buffer
        );

        Tensor {
            sender: None,
            read_bind_group,
            write_bind_group,
            shape_buffer,
            buffer,
            shape,
        }
    } 

    pub(crate) fn broadcast(&self, other: &Tensor) -> Option<Shape> {
        let mut shape = self.shape.clone();
        for (dst, src) in shape.iter_mut().zip(other.shape) {
            if (*dst == 1 || src == 1) || (*dst == src) {
                *dst = (*dst).max(src);
            } else { return None }
        }

        Some(shape)
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
                &self.read_bind_group,
                &[]
            );
        } else {
            compute_pass.set_bind_group(
                index,
                &self.write_bind_group,
                &[]
            );
        }
    }

    fn create_bind_group(
        device: &Device,
        layout: &BindGroupLayout,
        shape_buffer: &Buffer,
        buffer: &Buffer,
    ) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout,
            entries: &[BindGroupEntry {
                resource: buffer.as_entire_binding(),
                binding: 0,
            }, BindGroupEntry {
                resource: shape_buffer.as_entire_binding(),
                binding: 1,
            }]
        })
    }

    pub(crate) fn data_size(shape: Shape) -> BufferAddress {
        shape.into_iter()
            .map(|x| x as u64)
            .fold(1, u64::saturating_mul)
            .saturating_mul(size_of::<f32>() as u64)
    }

    pub(crate) fn buffer_size(shape: Shape) -> BufferSize {
        BufferSize::new(
            Self::data_size(shape)
                .checked_next_multiple_of(COPY_BUFFER_ALIGNMENT)
                .unwrap_or(BufferSize::MAX_VALUE)
        ).unwrap_or(BufferSize::new(COPY_BUFFER_ALIGNMENT).unwrap())
    }

    fn bucket_size(shape: Shape) -> BufferSize {
        let required = Self::buffer_size(shape).get()
            .max(Self::MIN_TEMPORARY_CAPACITY);

        let octave = 1u64 << ((u64::BITS - 1) - required.leading_zeros());
        let bucket_width = (octave / Self::BUCKETS_PER_OCTAVE)
            .max(COPY_BUFFER_ALIGNMENT);

        required.checked_next_multiple_of(bucket_width)
            .map(BufferSize::new).flatten()
            .unwrap_or(BufferSize::MAX)
    }
}

impl<'scope> Drop for Tensor<'scope> {
    fn drop(&mut self) {
        if let Some(sender) = self.sender.take() {
            sender.send(Tensor {
                write_bind_group: self.write_bind_group.clone(),
                read_bind_group: self.read_bind_group.clone(),
                shape_buffer: self.shape_buffer.clone(),
                buffer: self.buffer.clone(),
                shape: self.shape.clone(),
                sender: None
            }).ok();
        }
    }
}