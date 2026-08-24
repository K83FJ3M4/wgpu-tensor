use std::collections::HashMap;

use bytemuck::Pod;
use wgpu::{BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BufferBindingType, BufferSize, ComputePipeline, Device, ShaderStages};

mod reduction;
mod binary;
mod unary;

pub(crate) struct Pipelines {
    pub(crate) device: Device,
    pub(crate) tensor_input_layout: BindGroupLayout,
    pub(crate) tensor_output_layout: BindGroupLayout,
    pub(crate) param_layouts: ParamLayouts,

    reduction: Option<ComputePipeline>,
    binary: Option<ComputePipeline>,
    unary: Option<ComputePipeline>
}

pub(crate) struct ParamLayouts {
    cache: HashMap<usize, BindGroupLayout>
}

impl Pipelines {
    pub(crate) fn new(device: Device) -> Pipelines {

        let tensor_input_layout_ty = BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: true },
            has_dynamic_offset: false,
            min_binding_size: BufferSize::new(size_of::<u32>() as u64)
        };

        let tensor_input_layout = device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("Tensor Input"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: ShaderStages::COMPUTE,
                    ty: tensor_input_layout_ty
                }]
            }
        );

        let tensor_output_layout_ty = BindingType::Buffer {
            ty: BufferBindingType::Storage { read_only: false },
            has_dynamic_offset: false,
            min_binding_size: BufferSize::new(size_of::<u32>() as u64)
        };

        let tensor_output_layout = device.create_bind_group_layout(
            &BindGroupLayoutDescriptor {
                label: Some("Tensor Output"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    count: None,
                    visibility: ShaderStages::COMPUTE,
                    ty: tensor_output_layout_ty
                }]
            }
        );

        Pipelines {
            param_layouts: ParamLayouts::new(),
            tensor_input_layout,
            tensor_output_layout,
            device,

            reduction: None, 
            binary: None,
            unary: None
        }
    } 
}

impl ParamLayouts {
    fn new() -> ParamLayouts {
        ParamLayouts {
            cache: HashMap::new()
        }
    }

    pub(crate) fn get<T: Pod>(&mut self, device: &Device) -> &BindGroupLayout {
        let key = size_of::<T>();
        self.cache.entry(key).or_insert_with(|| {
            let param_ty = BindingType::Buffer {
                ty: BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: BufferSize::new(key as u64)
            };

            device.create_bind_group_layout(
                &BindGroupLayoutDescriptor {
                    label: Some(&format!("Parameters ({key})")),
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        count: None,
                        visibility: ShaderStages::COMPUTE,
                        ty: param_ty
                    }]
                }
            )
        })
    }
}