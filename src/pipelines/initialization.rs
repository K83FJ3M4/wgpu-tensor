use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};

use crate::pipelines::Pipelines;
use crate::{IntoShape, Tensor, TensorEncoder, TensorError};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct InitializationParameters {
    length: u32,
    operation: u32,
    constant_value: f32,
    random_lower: f32,
    random_upper: f32,
    seed_low: u32,
    seed_high: u32,
    stream: u32,
}

const CONSTANT: u32 = 0;
const RANDOM_UNIFORM: u32 = 1;

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

    pub fn constant(
        &mut self,
        value: f32,
        shape: impl IntoShape
    ) -> Result<Tensor<'scope>, TensorError> {
        let shape = shape.shape();
        let output = self.temp(shape)?;

        self.initialize_inner(&output, InitializationParameters {
            length: 0,
            operation: CONSTANT,
            constant_value: value,
            random_lower: 0.0,
            random_upper: 0.0,
            seed_low: 0,
            seed_high: 0,
            stream: 0,
        })?;

        Ok(output)
    }

    pub fn fill(
        &mut self,
        output: &Tensor<'static>,
        value: f32,
    ) -> Result<(), TensorError> {
        self.validate_write(output)?;

        self.initialize_inner(output, InitializationParameters {
            length: 0,
            operation: CONSTANT,
            constant_value: value,
            random_lower: 0.0,
            random_upper: 0.0,
            seed_low: 0,
            seed_high: 0,
            stream: 0,
        })
    }

    pub fn random_uniform(
        &mut self,
        output: &Tensor<'static>,
        lower: f32,
        upper: f32,
        seed: u64,
        stream: u32,
    ) -> Result<(), TensorError> {
        self.validate_write(output)?;

        if !lower.is_finite() || !upper.is_finite() || lower > upper {
            return Err(TensorError::InvalidRange)
        }

        self.initialize_inner(output, InitializationParameters {
            length: 0,
            operation: RANDOM_UNIFORM,
            constant_value: 0.0,
            random_lower: lower,
            random_upper: upper,
            seed_low: seed as u32,
            seed_high: (seed >> 32) as u32,
            stream,
        })
    }

    pub fn xavier_uniform(
        &mut self,
        output: &Tensor<'static>,
        fan_in: u32,
        fan_out: u32,
        seed: u64,
        stream: u32,
    ) -> Result<(), TensorError> {
        let bound = (6.0 / (fan_in as f32 + fan_out as f32)).sqrt();
        self.random_uniform(output, -bound, bound, seed, stream)
    }

    pub fn he_uniform(
        &mut self,
        output: &Tensor<'static>,
        fan_in: u32,
        seed: u64,
        stream: u32,
    ) -> Result<(), TensorError> {
        let bound = (6.0 / fan_in as f32).sqrt();
        self.random_uniform(output, -bound, bound, seed, stream)
    }

    fn initialize_inner(
        &mut self,
        output: &Tensor,
        mut params: InitializationParameters,
    ) -> Result<(), TensorError> {
        params.length = 1;

        let mut zero = false;
        for dimension in output.shape() {
            params.length = params.length.checked_mul(dimension)
                .ok_or(TensorError::OversizedTensor)?;
            zero |= dimension == 0;
        }

        if zero { return Ok(()) }

        let compute_pass = self.encoder.compute(
            Pipelines::initialization,
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

        Ok(())
    }
}

impl Pipelines {
    fn initialization(&mut self) -> &ComputePipeline {
        self.initialization.get_or_insert_with(|| {
            let module = self.device.create_shader_module(
                include_wgsl!(concat!(env!("OUT_DIR"), "/initialization.wgsl"))
            );

            let param_layout = self.param_layouts
                .get::<InitializationParameters>(&self.device);

            let layout = self.device.create_pipeline_layout(
                &PipelineLayoutDescriptor {
                    label: Some("Initialization"),
                    immediate_size: 0,
                    bind_group_layouts: &[
                        Some(param_layout),
                        Some(&self.tensor_output_layout)
                    ]
                }
            );

            self.device.create_compute_pipeline(
                &ComputePipelineDescriptor {
                    label: Some("Initialization"),
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
