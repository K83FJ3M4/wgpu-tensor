use std::num::NonZeroU64;
use bytemuck::{Pod, bytes_of};
use crossbeam_channel::{Receiver, Sender, unbounded};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Buffer, BufferAddress, BufferBinding, BufferDescriptor, BufferUsages, BufferViewMut, CommandEncoder, ComputePass, ComputePassDescriptor, ComputePipeline, Device, MapMode};

use crate::cache::CacheMap;
use crate::pipelines::Pipelines;

const CPU_BLOCK_SIZE: u64 = 4 << 20;
const GPU_BLOCK_SIZE: u64 = 256 << 10;

pub(crate) struct EncoderPool {
    parameters: Buffer,
    pipelines: Pipelines,
    bind_groups: CacheMap<usize, BindGroup>,
    receiver: Receiver<Buffer>,
    sender: Sender<Buffer>,
}

pub(crate) struct Encoder<'a> {
    command_encoder: &'a mut CommandEncoder,
    compute_pass: Option<ComputePass<'static>>,
    pool: &'a EncoderPool,
    view: Option<EncoderStagingBuffer>,
    cpu_offset: u64,
    gpu_offset: u64
}

struct EncoderStagingBuffer {
    view: BufferViewMut,
    buffer: Buffer
}

impl<'a> Encoder<'a> {
    pub(crate) fn new(
        pool: &'a EncoderPool,
        command_encoder: &'a mut CommandEncoder
    ) -> Encoder<'a> {
        Encoder {
            pool,
            command_encoder,
            compute_pass: None,
            view: None,
            cpu_offset: 0,
            gpu_offset: 0
        }
    } 

    pub(crate) fn compute<T: Pod>(
        &mut self,
        pipeline: fn(&Pipelines) -> &ComputePipeline,
        params: &T
    ) -> &mut ComputePass<'static> {
        let bytes = bytes_of(params);
        let length = bytes.len() as BufferAddress;
        let align = self.pool.pipelines.device.limits()
            .min_uniform_buffer_offset_alignment as u64;

        assert!((size_of::<T>() as u64) > 0);
        assert!((size_of::<T>() as u64) < 256);
        assert!((size_of::<T>() as u64) < GPU_BLOCK_SIZE);

        self.cpu_offset = self.cpu_offset.next_multiple_of(align);
        self.gpu_offset = self.gpu_offset.next_multiple_of(align);

        let replace_view = self.cpu_offset + length > CPU_BLOCK_SIZE;
        let write_params = self.gpu_offset + length > GPU_BLOCK_SIZE;

        if let Some(view) = self.view.as_ref() && replace_view {
            let buffer = view.buffer.clone();
            drop(self.view.take());
            buffer.unmap();
        }

        if let Some(view) = self.view.as_ref() && write_params {
            self.compute_pass = None;
            self.gpu_offset = 0;
            self.cpu_offset = self.cpu_offset
                .next_multiple_of(GPU_BLOCK_SIZE);

            self.command_encoder.copy_buffer_to_buffer(
                &view.buffer, self.cpu_offset,
                &self.pool.parameters, 0,
                self.pool.parameters.size()
            );
        }

        let view = self.view.get_or_insert_with(|| {
            let view = self.pool.buffer();
            self.compute_pass = None;
            self.cpu_offset = 0;
            self.gpu_offset = 0;

            let sender = self.pool.sender.clone();
            let buffer = view.buffer.clone();

            self.command_encoder.copy_buffer_to_buffer(
                &view.buffer, 0,
                &self.pool.parameters, 0,
                self.pool.parameters.size()
            );

            self.command_encoder.map_buffer_on_submit(
                &view.buffer,
                MapMode::Write, ..,
                move |result| if result.is_ok() {
                    sender.send(buffer).ok();
                }
            );

            view
        });

        let offset = self.cpu_offset as usize;
        view.view.slice(offset..offset + bytes.len())
            .copy_from_slice(bytes); 

        let compute_pass = self.compute_pass.get_or_insert_with(|| {
            self.command_encoder.begin_compute_pass(
                &ComputePassDescriptor {
                    label: None,
                    timestamp_writes: None
                }
            ).forget_lifetime()
        });

        compute_pass.set_bind_group(
            0, &self.pool.bind_group::<T>(),
            &[self.gpu_offset as u32]
        );

        let pipeline = pipeline(&self.pool.pipelines);
        compute_pass.set_pipeline(pipeline);

        self.cpu_offset += length;
        self.gpu_offset += length;
        compute_pass
    }

    pub(super) fn command(&mut self) -> &mut CommandEncoder {
        self.compute_pass = None;
        &mut self.command_encoder
    } 

    pub(crate) fn pipelines(&mut self) -> &Pipelines {
        &self.pool.pipelines
    }
}

impl EncoderPool {
    pub(crate) fn new(device: Device) -> EncoderPool {

        let (sender, receiver) = unbounded();
        let parameters = device.create_buffer(&BufferDescriptor {
            label: None,
            mapped_at_creation: false,
            size: GPU_BLOCK_SIZE,
            usage: BufferUsages::UNIFORM
                | BufferUsages::COPY_DST
        });

        EncoderPool {
            pipelines: Pipelines::new(device),
            bind_groups: CacheMap::new(),
            parameters,
            receiver,
            sender
        }
    }

    fn bind_group<T: Pod>(&self) -> BindGroup {
        let layout = self.pipelines.param_layout::<T>(&self.pipelines.device);

        self.bind_groups.get(size_of::<T>(), || {
            self.pipelines.device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &self.parameters,
                        size: NonZeroU64::new(size_of::<T>() as u64),
                        offset: 0
                    })
                }]
            }) 
        })
    }

    fn buffer(&self) -> EncoderStagingBuffer {
        while let Ok(buffer) = self.receiver.try_recv() {
            if let Ok(view) = buffer.get_mapped_range_mut(..) {
                return EncoderStagingBuffer {
                    buffer,
                    view
                } 
            }
        }

        let buffer = self.pipelines.device.create_buffer(&BufferDescriptor {
            usage: BufferUsages::MAP_WRITE | BufferUsages::COPY_SRC,
            mapped_at_creation: true,
            size: CPU_BLOCK_SIZE,
            label: None
        });

        let view = buffer.get_mapped_range_mut(..).unwrap();
        EncoderStagingBuffer {
            buffer,
            view
        } 
    }

    pub(crate) fn pipelines(&mut self) -> &mut Pipelines {
        &mut self.pipelines
    }
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        if let Some(view) = self.view.as_ref() {
            let buffer = view.buffer.clone();
            drop(self.view.take());
            buffer.unmap();
        }
    }
}