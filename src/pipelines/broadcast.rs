
use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};
use crate::pipelines::{BroadcastInfo, Pipelines};
use crate::tensor::ShapeDiff;
use crate::{IntoShape, Tensor, TensorEncoder, TensorError};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct BroadcastParameters {
    pub info: BroadcastInfo,
    pub length: u32,
    pub pad0: u32,
    pub pad1: u32,
    pub pad2: u32
}


impl<'scope> TensorEncoder<'scope> {
    pub fn broadcast(
        &mut self,
        operand: &Tensor<'scope>,
        shape: impl IntoShape
    ) -> Result<Tensor<'scope>, TensorError> {
        let source_shape = operand.shape();
        let result = self.broadcast_inner(operand, shape)?;

        if let Some(autograd) = self.autograd.as_mut() {
            let required = autograd.require([&result], operand);

            if required {
                let result_shape = result.shape();
                let result_weak = result.downgrade();
                let operand_weak = operand.downgrade();

                autograd.backwards(move |encoder, gradients| {
                    let Some(output_grad) = gradients.remove(result_weak) else {
                        return Ok(())
                    };

                    let dimensions = ShapeDiff::new(
                        source_shape,
                        result_shape
                    );
                    let input_grad = encoder.sum(
                        &output_grad,
                        dimensions
                    )?;

                    gradients.insert(
                        encoder,
                        operand_weak,
                        input_grad
                    )
                });
            }
        }

        Ok(result)
    }

    fn broadcast_inner(
        &mut self,
        operand: &Tensor<'scope>,
        shape: impl IntoShape
    ) -> Result<Tensor<'scope>, TensorError> {

        let target = shape.shape();
        let source = operand.shape();
        let mut params = BroadcastParameters::zeroed(); 
        params.length = 1;

        let mut zero = false;
        for (source_dimension, target_dimension) in source.into_iter().zip(target) {
            if source_dimension != 1 && source_dimension != target_dimension {
                return Err(TensorError::ShapeMismatch)
            }

            params.length = params.length.checked_mul(target_dimension)
                .ok_or(TensorError::OversizedDispatch)?;
            zero |= target_dimension == 0;
        }

        let output = self.temp(target)?;
        if zero { return Ok(output) }
        params.info = BroadcastInfo::new(
            source,
            target
        )?;

        let compute_pass = self.encoder.compute(
            Pipelines::broadcast,
            &params
        );

        operand.bind(compute_pass, 1, true);
        output.bind(compute_pass, 2, false);
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
    fn broadcast(&self) -> &ComputePipeline {
        self.broadcast.get_or_init(|| {
            let module = self.device.create_shader_module(
                include_wgsl!(concat!(env!("OUT_DIR"), "/broadcast.wgsl"))
            );

            let param_layout = self.param_layout::<BroadcastParameters>(&self.device);
            let layout = self.device.create_pipeline_layout(
                &PipelineLayoutDescriptor {
                    label: Some("Broadcast"),
                    immediate_size: 0,
                    bind_group_layouts: &[
                        Some(&param_layout),
                        Some(&self.tensor_input_layout),
                        Some(&self.tensor_output_layout)
                    ]
                }
            );

            self.device.create_compute_pipeline(
                &ComputePipelineDescriptor {
                    label: Some("Broadcast"),
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
