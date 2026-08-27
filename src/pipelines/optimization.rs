use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};
use crate::pipelines::Pipelines;
use crate::{Tensor, TensorEncoder, TensorError};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct OptimizationParameters {
    learning_rate: f32,
    length: u32,
}

impl<'scope> TensorEncoder<'scope> {
    pub(crate) fn optimize(
        &mut self,
        gradients: &Tensor<'scope>,
        weights: &Tensor<'scope>,
        learning_rate: f32,
    ) -> Result<(), TensorError> {

        let mut params = OptimizationParameters {
            learning_rate,
            length: 1,
        };

        if gradients.shape() != weights.shape() {
            return Err(TensorError::ShapeMismatch)
        }

        let mut zero = false;
        for dimension in gradients.shape() {
            params.length = params.length.checked_mul(dimension)
                .ok_or(TensorError::OversizedTensor)?;
            zero |= dimension == 0;
        }

        if zero { return Ok(()) }

        let compute_pass = self.encoder.compute(
            Pipelines::optimization,
            &params
        );

        gradients.bind(compute_pass, 1, true);
        weights.bind(compute_pass, 2, false);
        let num_workgroups = params.length.div_ceil(256);

        if num_workgroups != 0 && num_workgroups <= u16::MAX as u32 {
            compute_pass.dispatch_workgroups(num_workgroups, 1, 1); 
        } else if num_workgroups != 0 {
            let floor = num_workgroups.isqrt();
            let x = floor + (floor * floor != num_workgroups) as u32;
            let y = num_workgroups.div_ceil(x);
            compute_pass.dispatch_workgroups(x, y, 1);
        }

        Ok(())
    }
}

impl Pipelines {
    fn optimization(&mut self) -> &ComputePipeline {
        self.optimization.get_or_insert_with(|| {
            let module = self.device.create_shader_module(
                include_wgsl!(concat!(env!("OUT_DIR"), "/optimization.wgsl"))
            );

            let param_layout = self.param_layouts
                .get::<OptimizationParameters>(&self.device);

            let layout = self.device.create_pipeline_layout(
                &PipelineLayoutDescriptor {
                    label: Some("Optimization"),
                    immediate_size: 0,
                    bind_group_layouts: &[
                        Some(param_layout),
                        Some(&self.tensor_input_layout),
                        Some(&self.tensor_output_layout)
                    ]
                }
            );

            self.device.create_compute_pipeline(
                &ComputePipelineDescriptor {
                    label: Some("Optimization"),
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
