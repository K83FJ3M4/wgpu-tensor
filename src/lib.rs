
pub use tensor::{Tensor, IntoShape, IntoIndices, Shape};
pub use staging::{TensorReader, TensorWriter, PrintTensorReader};
use wgpu::{BufferView, BufferViewMut, CommandEncoder, Device, DownlevelFlags, Features, Limits};

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

pub struct TensorContext {
    write_pool: StagingAllocatorPool<BufferViewMut>,
    read_pool: StagingAllocatorPool<BufferView>,
    encoder_pool: EncoderPool,
    tensors: TensorPool
}

pub struct TensorEncoder<'scope> {
    read_allocator: StagingAllocator<'scope, BufferView>,
    write_allocator: StagingAllocator<'scope, BufferViewMut>,
    tensors: &'scope TensorPool,
    encoder: Encoder<'scope>
}

#[derive(Clone, Copy, Debug)]
pub enum TensorError {
    ShapeMismatch,
    OversizedTensor,
    OversizedDispatch,
    IndexOutOfBounds
}

impl TensorContext {
    pub fn new(device: Device) -> TensorContext {
        let tensors = TensorPool::new();
        let read_pool = StagingAllocatorPool::new(device.clone());
        let write_pool = StagingAllocatorPool::new(device.clone());
        let encoder_pool = EncoderPool::new(device);

        TensorContext {
            encoder_pool,
            read_pool,
            write_pool,
            tensors,
        }
    }

    pub fn encode<T>(
        &mut self,
        encoder: &mut CommandEncoder,
        callback: impl for<'scope> FnOnce(&mut TensorEncoder<'scope>) -> T
    ) -> T {
        let read_allocator = StagingAllocator::new(&self.read_pool);
        let write_allocator = StagingAllocator::new(&self.write_pool);
        let encoder = Encoder::new(&mut self.encoder_pool, encoder);

        callback(&mut TensorEncoder {
            tensors: &mut self.tensors,
            read_allocator,
            write_allocator,
            encoder 
        })
    }
}