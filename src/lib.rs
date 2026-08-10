
pub use tensor::{Tensor, IntoShape};
pub use staging::{TensorReader, TensorWriter};
use wgpu::{BufferView, BufferViewMut, CommandEncoder, ComputePass, ComputePassDescriptor, Device};

use crate::pipelines::{BindGroupLayoutPool, Pipelines};
use crate::staging::{StagingAllocator, StagingAllocatorPool};

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
    bind_group_layouts: BindGroupLayoutPool,
    write_pool: StagingAllocatorPool<BufferViewMut>,
    read_pool: StagingAllocatorPool<BufferView>,
    pipelines: Pipelines,
    device: Device
}

pub struct TensorEncoder<'a> {
    encoders: Encoders<'a>,
    pipelines: &'a mut Pipelines,
    bind_group_layouts: &'a mut BindGroupLayoutPool,
    read_allocator: StagingAllocator<'a, BufferView>,
    write_allocator: StagingAllocator<'a, BufferViewMut>
}

struct Encoders<'a> {
    command_encoder: &'a mut CommandEncoder,
    compute_pass: Option<ComputePass<'static>>
}

impl TensorContext {
    pub fn new(device: Device) -> TensorContext {
        let read_pool = StagingAllocatorPool::new(device.clone());
        let write_pool = StagingAllocatorPool::new(device.clone());
        let bind_group_layouts = BindGroupLayoutPool::new(&device)
            .expect("Failed to create bind group layout pool");

        TensorContext {
            pipelines: Pipelines::default(),
            bind_group_layouts,
            read_pool,
            write_pool,
            device
        }
    }

    pub fn encode(
        &mut self,
        encoder: &mut CommandEncoder,
        callback: impl FnOnce(&mut TensorEncoder)
    ) {
        let read_allocator = StagingAllocator::new(&self.read_pool);
        let write_allocator = StagingAllocator::new(&self.write_pool);

        callback(&mut TensorEncoder {
            encoders: Encoders::new(encoder),
            pipelines: &mut self.pipelines,
            bind_group_layouts: &mut self.bind_group_layouts,
            read_allocator,
            write_allocator
        });
    }
}

impl<'a> Encoders<'a> {
    fn new(command_encoder: &'a mut CommandEncoder) -> Encoders<'a> {
        Encoders {
            command_encoder,
            compute_pass: None
        }
    }

    fn command(&mut self) -> &mut CommandEncoder {
        self.compute_pass = None;
        self.command_encoder
    }

    fn compute(&mut self) -> &mut ComputePass<'static> {
        self.compute_pass.get_or_insert_with(|| {
            self.command_encoder.begin_compute_pass(
                &ComputePassDescriptor {
                    label: None,
                    timestamp_writes: None
                }
            ).forget_lifetime()
        })
    }
}