use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};

use crate::optimizers::AutogradEncoder;
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
    pub const SIGN: Self = Self(8);
}

impl<'scope> TensorEncoder<'scope> {
    pub fn negate(
        &mut self,
        operand: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.unary(
            UnaryOperation::NEGATE,
            operand,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            autograd.backwards_unary(
                operand,
                &res,
                |encoder, output_grad| encoder.negate(output_grad)
            );
        }

        Ok(res)
    }

    pub fn absolute(
        &mut self,
        operand: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.unary(
            UnaryOperation::ABSOLUTE,
            operand,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let operand_value = operand.clone();
            autograd.backwards_unary(
                operand,
                &res,
                move |encoder, output_grad| {
                    let sign = encoder.sign(&operand_value)?;
                    encoder.multiply(output_grad, &sign)
                }
            );
        }

        Ok(res)
    }

    pub fn reciprocal(
        &mut self,
        operand: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.unary(
            UnaryOperation::RECIPROCAL,
            operand,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let result = res.clone();
            autograd.backwards_unary(
                operand,
                &res,
                move |encoder, output_grad| {
                    let square = encoder.multiply(&result, &result)?;
                    let gradient = encoder.multiply(output_grad, &square)?;
                    encoder.negate(&gradient)
                }
            );
        }

        Ok(res)
    }

    pub fn square_root(
        &mut self,
        operand: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.unary(
            UnaryOperation::SQUARE_ROOT,
            operand,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let result = res.clone();
            autograd.backwards_unary(
                operand,
                &res,
                move |encoder, output_grad| {
                    let denominator = encoder.add(&result, &result)?;
                    encoder.divide(output_grad, &denominator)
                }
            );
        }

        Ok(res)
    }

    pub fn reciprocal_square_root(
        &mut self,
        operand: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.unary(
            UnaryOperation::RECIPROCAL_SQUARE_ROOT,
            operand,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let result = res.clone();
            autograd.backwards_unary(
                operand,
                &res,
                move |encoder, output_grad| {
                    let square = encoder.multiply(&result, &result)?;
                    let cube = encoder.multiply(&square, &result)?;
                    let gradient = encoder.multiply(output_grad, &cube)?;
                    let one = encoder.ones(1)?;
                    let two = encoder.add(&one, &one)?;
                    let gradient = encoder.divide(&gradient, &two)?;
                    encoder.negate(&gradient)
                }
            );
        }

        Ok(res)
    }

    pub fn exponential(
        &mut self,
        operand: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.unary(
            UnaryOperation::EXPONENTIAL,
            operand,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let result = res.clone();
            autograd.backwards_unary(
                operand,
                &res,
                move |encoder, output_grad| {
                    encoder.multiply(output_grad, &result)
                }
            );
        }

        Ok(res)
    }

    pub fn logarithm(
        &mut self,
        operand: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.unary(
            UnaryOperation::LOGARITHM,
            operand,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let operand_value = operand.clone();
            autograd.backwards_unary(
                operand,
                &res,
                move |encoder, output_grad| {
                    encoder.divide(output_grad, &operand_value)
                }
            );
        }

        Ok(res)
    }

    pub fn copy(
        &mut self,
        operand: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.unary(
            UnaryOperation::COPY,
            operand,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            autograd.backwards_unary(
                operand,
                &res,
                |_, output_grad| Ok(output_grad.clone())
            );
        }

        Ok(res)
    }

    pub fn sign(
        &mut self,
        operand: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.unary(UnaryOperation::SIGN, operand)?;

        if let Some(autograd) = self.autograd.as_mut() {
            let operand_shape = operand.shape();
            autograd.backwards_unary(
                operand,
                &res,
                move |encoder, _| encoder.zeros(operand_shape)
            );
        }

        Ok(res)
    }

    fn unary(
        &mut self,
        operation: UnaryOperation,
        operand: &Tensor<'scope>,
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

impl<'scope> AutogradEncoder<'scope> {
    fn backwards_unary(
        &mut self,
        operand: &Tensor<'scope>,
        res: &Tensor<'scope>,
        gradient: impl FnOnce(
            &mut TensorEncoder<'scope>,
            &Tensor<'scope>,
        ) -> Result<Tensor<'scope>, TensorError> + 'scope,
    ) {
        if self.require([res], operand) {
            let res_weak = res.downgrade();
            let operand_weak = operand.downgrade();

            self.backwards(move |encoder, gradients| {
                let Some(output_grad) = gradients.remove(res_weak) else {
                    return Ok(())
                };

                let input_grad = gradient(encoder, &output_grad)?;
                gradients.insert(
                    encoder,
                    operand_weak,
                    input_grad
                )
            });
        }
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
