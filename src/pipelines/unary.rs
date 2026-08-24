use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};

use crate::pipelines::Pipelines;
use crate::{Tensor, TensorEncoder, TensorError};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct UnaryParameters {
    pub length: u32,
    pub operation: UnaryOperation
}

#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq)]
pub struct UnaryOperation(u32);

impl UnaryOperation {
    pub const NEGATE: Self = Self(0);
    pub const ABSOLUTE: Self = Self(1);
    pub const RECIPROCAL: Self = Self(2);
    pub const SQUARE_ROOT: Self = Self(3);
    pub const RECIPROCAL_SQUARE_ROOT: Self = Self(4);
    pub const EXPONENTIAL: Self = Self(5);
    pub const LOGARITHM: Self = Self(6);
    pub const COPY: Self = Self(7);
}

impl<'scope> TensorEncoder<'scope> {
    pub fn negate(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::NEGATE,
            operand,
        )
    } 

    pub fn absolute(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::ABSOLUTE,
            operand,
        )
    }

    pub fn reciprocal(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::RECIPROCAL,
            operand,
        )
    }

    pub fn square_root(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::SQUARE_ROOT,
            operand,
        )
    }

    pub fn reciprocal_square_root(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::RECIPROCAL_SQUARE_ROOT,
            operand,
        )
    }

    pub fn exponential(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::EXPONENTIAL,
            operand,
        )
    }

    pub fn logarithm(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::LOGARITHM,
            operand,
        )
    }

    pub fn copy(
        &mut self,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {
        self.unary(
            UnaryOperation::COPY,
            operand,
        )
    }

    fn unary(
        &mut self,
        operation: UnaryOperation,
        operand: &Tensor,
    ) -> Result<Tensor<'scope>, TensorError> {

        let mut params = UnaryParameters {
            length: 1,
            operation
        };

        let mut zero = false;
        for dimension in operand.shape() {
            params.length = params.length.checked_mul(dimension)
                .ok_or(TensorError::OversizedTensor)?;
            zero |= dimension == 0;
        }

        let output = self.temp(operand.shape())?;
        if zero { return Ok(output) }

        let compute_pass = self.encoder.compute(
            Pipelines::unary,
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
    fn unary(&mut self) -> &ComputePipeline {
        self.unary.get_or_insert_with(|| {
            let module = self.device.create_shader_module(
                include_wgsl!(concat!(env!("OUT_DIR"), "/unary.wgsl"))
            );

            let param_layout = self.param_layouts
                .get::<UnaryParameters>(&self.device);

            let layout = self.device.create_pipeline_layout(
                &PipelineLayoutDescriptor {
                    label: Some("Unary"),
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
                    label: Some("Unary"),
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