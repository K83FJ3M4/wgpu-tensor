pub const BASELINE_FEATURES: wgpu::Features = wgpu::Features::empty();
pub const ALL_FEATURES: wgpu::Features = wgpu::Features::empty();
pub const BASELINE_LIMITS: wgpu::Limits = wgpu::Limits {
    max_texture_dimension_1d: 8192,
    max_texture_dimension_2d: 8192,
    max_texture_dimension_3d: 2048,
    max_texture_array_layers: 256,
    max_bind_groups: 4,
    max_bind_groups_plus_vertex_buffers: 24,
    max_bindings_per_bind_group: 1000,
    max_dynamic_uniform_buffers_per_pipeline_layout: 8,
    max_dynamic_storage_buffers_per_pipeline_layout: 4,
    max_sampled_textures_per_shader_stage: 16,
    max_samplers_per_shader_stage: 16,
    max_storage_buffers_per_shader_stage: 8,
    max_storage_textures_per_shader_stage: 4,
    max_uniform_buffers_per_shader_stage: 12,
    max_binding_array_elements_per_shader_stage: 0,
    max_binding_array_acceleration_structure_elements_per_shader_stage: 0,
    max_binding_array_sampler_elements_per_shader_stage: 0,
    max_uniform_buffer_binding_size: 65536,
    max_storage_buffer_binding_size: 134217728,
    max_vertex_buffers: 8,
    max_buffer_size: 268435456,
    max_vertex_attributes: 16,
    max_vertex_buffer_array_stride: 2048,
    max_inter_stage_shader_variables: 16,
    min_uniform_buffer_offset_alignment: 256,
    min_storage_buffer_offset_alignment: 256,
    max_color_attachments: 8,
    max_color_attachment_bytes_per_sample: 32,
    max_compute_workgroup_storage_size: 16384,
    max_compute_invocations_per_workgroup: 256,
    max_compute_workgroup_size_x: 256,
    max_compute_workgroup_size_y: 256,
    max_compute_workgroup_size_z: 64,
    max_compute_workgroups_per_dimension: 65535,
    max_immediate_size: 0,
    max_non_sampler_bindings: 1000000,
    max_task_workgroup_total_count: 0,
    max_task_workgroups_per_dimension: 0,
    max_mesh_workgroup_total_count: 0,
    max_mesh_workgroups_per_dimension: 0,
    max_task_invocations_per_workgroup: 0,
    max_task_invocations_per_dimension: 0,
    max_mesh_invocations_per_workgroup: 0,
    max_mesh_invocations_per_dimension: 0,
    max_task_payload_size: 0,
    max_mesh_output_vertices: 0,
    max_mesh_output_primitives: 0,
    max_mesh_output_layers: 0,
    max_mesh_multiview_view_count: 0,
    max_blas_primitive_count: 0,
    max_blas_geometry_count: 0,
    max_tlas_instance_count: 0,
    max_acceleration_structures_per_shader_stage: 0,
    max_buffers_and_acceleration_structures_per_shader_stage: 28,
    max_multiview_view_count: 0,
    max_ray_dispatch_count: 0,
    max_ray_recursion_depth: 0,
};
pub const BASELINE_DOWNLEVEL_FLAGS: wgpu::DownlevelFlags = wgpu::DownlevelFlags::COMPUTE_SHADERS
    .union(wgpu::DownlevelFlags::BUFFER_BINDINGS_NOT_16_BYTE_ALIGNED);
#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BindingSize(wgpu::BufferSize);
#[allow(unused)]
impl BindingSize {
    pub const fn of<T>() -> Self {
        let size = core::mem::size_of::<T>() as u64;
        let Some(size) = wgpu::BufferSize::new(size) else {
            panic!("buffer binding types must not have a size of zero");
        };
        Self(size)
    }
}
#[allow(unused)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BindingShape {
    Buffer { ty: wgpu::BufferBindingType, has_dynamic_offset: bool, size: BindingSize },
    Sampler(wgpu::SamplerBindingType),
    Texture {
        sample_type: wgpu::TextureSampleType,
        view_dimension: wgpu::TextureViewDimension,
        multisampled: bool,
    },
    StorageTexture {
        access: wgpu::StorageTextureAccess,
        format: wgpu::TextureFormat,
        view_dimension: wgpu::TextureViewDimension,
    },
    ExternalTexture,
}
#[allow(unused)]
impl BindingShape {
    fn binding_type(self) -> wgpu::BindingType {
        match self {
            Self::Buffer { ty, has_dynamic_offset, size } => {
                wgpu::BindingType::Buffer {
                    ty,
                    has_dynamic_offset,
                    min_binding_size: Some(size.0),
                }
            }
            Self::Sampler(ty) => wgpu::BindingType::Sampler(ty),
            Self::Texture { sample_type, view_dimension, multisampled } => {
                wgpu::BindingType::Texture {
                    sample_type,
                    view_dimension,
                    multisampled,
                }
            }
            Self::StorageTexture { access, format, view_dimension } => {
                wgpu::BindingType::StorageTexture {
                    access,
                    format,
                    view_dimension,
                }
            }
            Self::ExternalTexture => wgpu::BindingType::ExternalTexture,
        }
    }
}
#[allow(unused)]
pub struct BindGroupLayoutPool {
    device: wgpu::Device,
    layouts: std::collections::HashMap<Vec<BindingShape>, wgpu::BindGroupLayout>,
}
#[allow(unused)]
impl BindGroupLayoutPool {
    pub fn new(device: &wgpu::Device) -> Option<Self> {
        if !device.features().contains(BASELINE_FEATURES)
            || !BASELINE_LIMITS.check_limits(&device.limits())
        {
            return None;
        }
        Some(Self {
            device: device.clone(),
            layouts: std::collections::HashMap::new(),
        })
    }
    pub fn get(&mut self, shape: &[BindingShape]) -> &wgpu::BindGroupLayout {
        let device = &self.device;
        self.layouts
            .entry(shape.to_vec())
            .or_insert_with(|| {
                let entries = shape
                    .iter()
                    .enumerate()
                    .map(|(binding, shape)| wgpu::BindGroupLayoutEntry {
                        binding: binding as u32,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: shape.binding_type(),
                        count: None,
                    })
                    .collect::<Vec<_>>();
                device
                    .create_bind_group_layout(
                        &wgpu::BindGroupLayoutDescriptor {
                            label: None,
                            entries: &entries,
                        },
                    )
            })
    }
}
pub mod shaders {
    #[allow(unused_imports)]
    use super::{BindGroupLayoutPool, BindingShape, BindingSize};
    pub mod binary {
        #[allow(unused_imports)]
        use super::{BindGroupLayoutPool, BindingShape, BindingSize};
        pub fn main(pool: &mut BindGroupLayoutPool) -> wgpu::ComputePipeline {
            let bind_group_layout_0 = pool
                .get(
                    &[
                        BindingShape::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: true,
                            size: BindingSize(wgpu::BufferSize::new(128u64).unwrap()),
                        },
                    ],
                )
                .clone();
            let bind_group_layout_1 = pool
                .get(
                    &[
                        BindingShape::Buffer {
                            ty: wgpu::BufferBindingType::Storage {
                                read_only: true,
                            },
                            has_dynamic_offset: false,
                            size: BindingSize(wgpu::BufferSize::new(4u64).unwrap()),
                        },
                    ],
                )
                .clone();
            let bind_group_layout_2 = pool
                .get(
                    &[
                        BindingShape::Buffer {
                            ty: wgpu::BufferBindingType::Storage {
                                read_only: true,
                            },
                            has_dynamic_offset: false,
                            size: BindingSize(wgpu::BufferSize::new(4u64).unwrap()),
                        },
                    ],
                )
                .clone();
            let bind_group_layout_3 = pool
                .get(
                    &[
                        BindingShape::Buffer {
                            ty: wgpu::BufferBindingType::Storage {
                                read_only: false,
                            },
                            has_dynamic_offset: false,
                            size: BindingSize(wgpu::BufferSize::new(4u64).unwrap()),
                        },
                    ],
                )
                .clone();
            let bind_group_layouts: &[Option<&wgpu::BindGroupLayout>] = &[
                Some(&bind_group_layout_0),
                Some(&bind_group_layout_1),
                Some(&bind_group_layout_2),
                Some(&bind_group_layout_3),
            ];
            let pipeline_layout = pool
                .device
                .create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: Some("binary::main"),
                        bind_group_layouts,
                        immediate_size: 0u32,
                    },
                );
            let shader = pool
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("binary::main"),
                    source: wgpu::ShaderSource::Wgsl(
                        include_str!("binary_main.wgsl").into(),
                    ),
                });
            pool.device
                .create_compute_pipeline(
                    &wgpu::ComputePipelineDescriptor {
                        label: Some("binary::main"),
                        layout: Some(&pipeline_layout),
                        module: &shader,
                        entry_point: Some("main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        cache: None,
                    },
                )
        }
    }
}
