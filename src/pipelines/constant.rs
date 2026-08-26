use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};

use crate::pipelines::Pipelines;
use crate::{IntoShape, Tensor, TensorEncoder, TensorError};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct ConstantParameters {
    length: u32,
    value: f32
}

impl<'scope> TensorEncoder<'scope> {
    pub fn zeros(
        &mut self,
        shape: impl IntoShape
    ) -> Result<Tensor<'scope>, TensorError> {
        self.constant(0.0, shape)
    }

    pub fn ones(
        &mut self,
        shape: impl IntoShape
    ) -> Result<Tensor<'scope>, TensorError> {
        self.constant(1.0, shape) 
    }

    fn constant(
        &mut self,
        value: f32,
        shape: impl IntoShape
    ) -> Result<Tensor<'scope>, TensorError> {

        let mut params = ConstantParameters {
            length: 1,
            value
        };

        let mut zero = false;
        let shape = shape.shape();
        for dimension in shape {
            params.length = params.length.checked_mul(dimension)
                .ok_or(TensorError::OversizedTensor)?;
            zero |= dimension == 0;
        }

        let output = self.temp(shape)?;
        if zero { return Ok(output) }

        let compute_pass = self.encoder.compute(
            Pipelines::constant,
            &params
        );

        output.bind(compute_pass, 1, false);
        let num_workgroups = params.length.div_ceil(256);

        if num_workgroups != 0 && num_workgroups <= u16::MAX as u32 {
            compute_pass.dispatch_workgroups(num_workgroups, 1, 1); 
        } else if num_workgroups != 0 {
            let floor = num_workgroups.isqrt();
            let x = floor + (floor * floor != num_workgroups) as u32;
            let y = num_workgroups.div_ceil(x);
            compute_pass.dispatch_workgroups(x, y, 1);
        }

        Ok(output)
    }
}

impl Pipelines {
    fn constant(&mut self) -> &ComputePipeline {
        self.constant.get_or_insert_with(|| {
            let module = self.device.create_shader_module(
                include_wgsl!(concat!(env!("OUT_DIR"), "/constant.wgsl"))
            );

            let param_layout = self.param_layouts
                .get::<ConstantParameters>(&self.device);

            let layout = self.device.create_pipeline_layout(
                &PipelineLayoutDescriptor {
                    label: Some("Constant"),
                    immediate_size: 0,
                    bind_group_layouts: &[
                        Some(param_layout),
                        Some(&self.tensor_output_layout)
                    ]
                }
            );

            self.device.create_compute_pipeline(
                &ComputePipelineDescriptor {
                    label: Some("Constant"),
                    compilation_options: Default::default(),
                    cache: None,
                    layout: Some(&layout),
                    entry_point: Some("main"),
                    module: &module
                }
            )
        })
    }
}