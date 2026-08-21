
pub use tensor::{Tensor, IntoShape};
pub use staging::{TensorReader, TensorWriter, PrintTensorReader};
use wgpu::{BufferView, BufferViewMut, CommandEncoder, Device};

use crate::pipelines::{Pipelines};
use crate::staging::{Encoder, EncoderPool, StagingAllocator, StagingAllocatorPool};
use crate::tensor::TensorPool;

pub use pipelines::{
    ALL_FEATURES,
    BASELINE_DOWNLEVEL_FLAGS,
    BASELINE_FEATURES,
    BASELINE_LIMITS
};

mod tensor;
mod staging;
mod pipelines;

pub struct TensorContext {
    write_pool: StagingAllocatorPool<BufferViewMut>,
    read_pool: StagingAllocatorPool<BufferView>,
    encoder_pool: EncoderPool,
    tensors: TensorPool,
    pipelines: Pipelines,
    device: Device
}

pub struct TensorEncoder<'scope> {
    pipelines: &'scope mut Pipelines,
    read_allocator: StagingAllocator<'scope, BufferView>,
    write_allocator: StagingAllocator<'scope, BufferViewMut>,
    encoder: Encoder<'scope>,
    tensors: &'scope TensorPool,
    device: &'scope Device
}

#[derive(Clone, Copy, Debug)]
pub enum TensorError {
    ShapeMismatch,
    OversizedTensor,
    OversizedDispatch
}

impl TensorContext {
    pub fn new(device: Device) -> TensorContext {
        let tensors = TensorPool::new();
        let read_pool = StagingAllocatorPool::new(device.clone());
        let write_pool = StagingAllocatorPool::new(device.clone());
        let encoder_pool = EncoderPool::new(&device);

        TensorContext {
            tensors,
            pipelines: Pipelines::default(),
            encoder_pool,
            read_pool,
            write_pool,
            device
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
            pipelines: &mut self.pipelines,
            tensors: &mut self.tensors,
            read_allocator,
            write_allocator,
            device: &self.device,
            encoder 
        })
    }
}