pub use tensor::{Tensor, IntoShape, IntoIndices, Shape};
pub use staging::{TensorReader, TensorWriter, PrintTensorReader};
use wgpu::{BufferView, BufferViewMut, CommandEncoder, Device, DownlevelFlags, Features, Limits};

use crate::optimizers::AutogradEncoder;
use crate::pipelines::RngState;
use crate::staging::{Encoder, EncoderPool, StagingAllocator, StagingAllocatorPool};
use crate::tensor::TensorPool;

pub const OPTIONAL_FEATURES: Features = Features::empty();
pub const REQUIRED_LIMITS: Limits = Limits::defaults();
pub const REQUIRED_DOWNLEVEL_FLAGS: DownlevelFlags = {
    DownlevelFlags::BUFFER_BINDINGS_NOT_16_BYTE_ALIGNED
        .union(DownlevelFlags::COMPUTE_SHADERS)
};

mod tensor;
mod staging;
mod pipelines;

pub mod optimizers;
pub mod layers;

pub struct TensorContext {
    write_pool: StagingAllocatorPool<BufferViewMut>,
    read_pool: StagingAllocatorPool<BufferView>,
    encoder_pool: EncoderPool,
    tensors: TensorPool,
    rng: RngState
}

pub struct TensorEncoder<'scope> {
    read_allocator: StagingAllocator<'scope, BufferView>,
    write_allocator: StagingAllocator<'scope, BufferViewMut>,
    autograd: Option<AutogradEncoder<'scope>>,
    tensors: &'scope TensorPool,
    encoder: Encoder<'scope>,
    rng: &'scope mut RngState
}

#[derive(Clone, Copy, Debug)]
pub enum TensorError {
    ShapeMismatch,
    OversizedTensor,
    OversizedDispatch,
    IndexOutOfBounds,
    IncompatibleMatrices,
    InvalidLossShape,
    InvalidRange,
    TrainableWriteDuringLearning,
}

impl<'scope> TensorEncoder<'scope> {
    pub(crate) fn validate_write(
        &self,
        tensor: &Tensor<'static>,
    ) -> Result<(), TensorError> {
        if self.autograd.is_some() && tensor.trainable() {
            Err(TensorError::TrainableWriteDuringLearning)
        } else {
            Ok(())
        }
    }
}

impl TensorContext {
    pub fn new(device: Device) -> TensorContext {
        let tensors = TensorPool::new();
        let read_pool = StagingAllocatorPool::new(device.clone());
        let write_pool = StagingAllocatorPool::new(device.clone());
        let encoder_pool = EncoderPool::new(device);
        let rng = RngState::new();

        TensorContext {
            encoder_pool,
            read_pool,
            write_pool,
            tensors,
            rng
        }
    }

    pub fn infer(
        &mut self,
        encoder: &mut CommandEncoder,
        callback: impl for<'scope> FnOnce(&mut TensorEncoder<'scope>)
            -> Result<(), TensorError>
    ) -> Result<(), TensorError> {
        let read_allocator = StagingAllocator::new(&self.read_pool);
        let write_allocator = StagingAllocator::new(&self.write_pool);
        let encoder = Encoder::new(&mut self.encoder_pool, encoder);

        callback(&mut TensorEncoder {
            tensors: &mut self.tensors,
            rng: &mut self.rng,
            autograd: None,
            read_allocator,
            write_allocator,
            encoder 
        })
    }

    pub fn learn(
        &mut self,
        encoder: &mut CommandEncoder,
        callback: impl for<'scope> FnOnce(&mut TensorEncoder<'scope>)
            -> Result<Tensor<'scope>, TensorError>
    ) -> Result<(), TensorError> {
        let read_allocator = StagingAllocator::new(&self.read_pool);
        let write_allocator = StagingAllocator::new(&self.write_pool);
        let encoder = Encoder::new(&mut self.encoder_pool, encoder);
        let autograd = AutogradEncoder::new();
        let mut encoder = TensorEncoder {
            tensors: &mut self.tensors,
            rng: &mut self.rng,
            autograd: Some(autograd),
            read_allocator,
            write_allocator,
            encoder 
        };

        let loss = callback(&mut encoder)?;
        let autograd = encoder.autograd.take().unwrap();
        autograd.encode(&mut encoder, loss)
    }
}
