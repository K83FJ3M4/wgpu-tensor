use std::ops::Mul;

use bytemuck::cast_slice;
use shaders::Shape;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, COPY_BUFFER_ALIGNMENT, ComputePass, Device};
pub use shape::IntoShape;

use crate::TensorContext;
use crate::pipelines::{BindingShape, BindingSize};

mod shape;

pub struct Tensor {
    pub(crate) read_bind_group: BindGroup,
    pub(crate) write_bind_group: BindGroup,
    pub(crate) buffer: Buffer,
    pub(crate) shape: Shape,
}

impl Tensor {
    pub fn new(context: &mut TensorContext, shape: impl IntoShape) -> Tensor {
        let shape = shape.shape();
        let size = shape.into_iter()
            .map(|x| x as u64)
            .fold(1, Mul::mul)
            .mul(size_of::<f32>() as u64)
            .next_multiple_of(COPY_BUFFER_ALIGNMENT)
            .max(COPY_BUFFER_ALIGNMENT);

        let usage =  BufferUsages::COPY_DST
                | BufferUsages::COPY_SRC
                | BufferUsages::STORAGE;

        let buffer = context.device.create_buffer(
            &BufferDescriptor {
                mapped_at_creation: false,
                label: None,
                usage,
                size
            }
        );

        let shape_buffer = context.device.create_buffer_init(
            &BufferInitDescriptor {
                contents: cast_slice(shape.as_slice()),
                usage: BufferUsages::UNIFORM,
                label: None,
            }
        );

        let read_bind_group = Self::create_bind_group(
            &context.device,
            context.bind_group_layouts.get(&[
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
            &context.device,
            context.bind_group_layouts.get(&[
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
            read_bind_group,
            write_bind_group,
            buffer,
            shape
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

    pub(crate) fn data_size(&self) -> usize {
        self.shape.into_iter()
            .map(|x| x as usize)
            .fold(1, Mul::mul)
            .mul(size_of::<f32>())
    }
}