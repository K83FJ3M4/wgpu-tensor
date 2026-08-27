use std::cell::OnceCell;
use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};
use crate::optimizers::{AutogradEncoder};
use crate::pipelines::{BroadcastInfo, Pipelines};
use crate::tensor::ShapeDiff;
use crate::{Tensor, TensorEncoder, TensorError};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct BinaryParameters {
    pub info: BroadcastInfo,
    pub length: u32,
    pub operation: BinaryOperation,
    pub pad0: u32,
    pub pad1: u32
}


#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq)]
pub struct BinaryOperation(u32);

impl BinaryOperation {
    pub const ADD: Self = Self(0);
    pub const SUBTRACT: Self = Self(1);
    pub const MULTIPLY: Self = Self(2);
    pub const DIVIDE: Self = Self(3);
    pub const POWER: Self = Self(4);
    pub const MINIMUM: Self = Self(5);
    pub const MAXIMUM: Self = Self(6);
    pub const REMAINDER: Self = Self(7);
    pub const EQUAL: Self = Self(8);
}

impl<'scope> TensorEncoder<'scope> {
    pub fn add(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.binary(
            BinaryOperation::ADD,
            lhs,
            rhs,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            autograd.backwards_binary(
                lhs, rhs, &res,
                |_, output| Ok(output.clone()),
                |_, output| Ok(output.clone())
            ); 
        }

        Ok(res)
    }

    pub fn subtract(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.binary(
            BinaryOperation::SUBTRACT,
            lhs,
            rhs,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            autograd.backwards_binary(
                lhs, rhs, &res,
                |_, output| Ok(output.clone()),
                |encoder, output| encoder.negate(output)
            );
        }

        Ok(res)
    }

    pub fn multiply(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.binary(
            BinaryOperation::MULTIPLY,
            lhs,
            rhs
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let lhs_clone = lhs.clone();
            let rhs_clone = rhs.clone();
            autograd.backwards_binary(
                &lhs, &rhs, &res,
                move |encoder, output| {
                    encoder.multiply(output, &rhs_clone)
                },
                move |encoder, output| {
                    encoder.multiply(output, &lhs_clone)
                }
            );
        }

        Ok(res)
    }

    pub fn divide(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {

        let res = self.binary(
            BinaryOperation::DIVIDE,
            lhs,
            rhs
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let lhs_clone = lhs.clone();
            let rhs_clone = rhs.clone();

            let cell = Rc::new(OnceCell::new());
            let cell_clone = cell.clone();
            let lhs_grad = move |
                encoder: &mut TensorEncoder<'scope>,
                output: &Tensor<'scope>| {
                cell_clone.get_or_init(|| {
                    encoder.divide(output, &rhs_clone)
                }).clone()
            };

            let rhs_clone = rhs.clone();
            let rhs_grad = move |
                encoder: &mut TensorEncoder<'scope>,
                output: &Tensor<'scope>| {
                let q = cell.get_or_init(|| {
                    encoder.divide(output, &rhs_clone)
                }).clone()?;
                let drhs = encoder.divide(&lhs_clone, &rhs_clone)?;
                let drhs = encoder.multiply(&q, &drhs)?;
                encoder.negate(&drhs)
            };

            autograd.backwards_binary(
                &lhs, &rhs, &res,
                lhs_grad,
                rhs_grad 
            );
        }

        Ok(res)
    }

    pub fn power(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.binary(
            BinaryOperation::POWER,
            lhs,
            rhs,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let lhs_clone = lhs.clone();
            let rhs_clone = rhs.clone();
            let lhs_grad = move |
                encoder: &mut TensorEncoder<'scope>,
                output: &Tensor<'scope>| {
                let one = encoder.ones(1)?;
                let exponent = encoder.subtract(&rhs_clone, &one)?;
                let power = encoder.power(&lhs_clone, &exponent)?;
                let derivative = encoder.multiply(&rhs_clone, &power)?;
                encoder.multiply(output, &derivative)
            };

            let lhs_clone = lhs.clone();
            let res_clone = res.clone();
            let rhs_grad = move |
                encoder: &mut TensorEncoder<'scope>,
                output: &Tensor<'scope>| {
                let logarithm = encoder.logarithm(&lhs_clone)?;
                let derivative = encoder.multiply(&res_clone, &logarithm)?;
                encoder.multiply(output, &derivative)
            };

            autograd.backwards_binary(
                lhs, rhs, &res,
                lhs_grad,
                rhs_grad
            );
        }

        Ok(res)
    }

    pub fn minimum(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.extremum(
            BinaryOperation::MINIMUM,
            lhs,
            rhs,
        )
    }

    pub fn maximum(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.extremum(
            BinaryOperation::MAXIMUM,
            lhs,
            rhs,
        )
    }

    pub fn remainder(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.binary(
            BinaryOperation::REMAINDER,
            lhs,
            rhs,
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let lhs_clone = lhs.clone();
            let rhs_clone = rhs.clone();
            let res_clone = res.clone();
            autograd.backwards_binary(
                lhs, rhs, &res,
                |_, output| Ok(output.clone()),
                move |encoder, output| {
                    let difference = encoder.subtract(&lhs_clone, &res_clone)?;
                    let quotient = encoder.divide(&difference, &rhs_clone)?;
                    let gradient = encoder.multiply(output, &quotient)?;
                    encoder.negate(&gradient)
                }
            );
        }

        Ok(res)
    }

    //TODO implement boolean ops so this is no longer required
    pub(crate) fn equal(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(BinaryOperation::EQUAL, lhs, rhs)
    }

    //TODO masks with "select" and "greater_equal"
    fn extremum(
        &mut self,
        operation: BinaryOperation,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.binary(operation, lhs, rhs)?;

        if let Some(autograd) = self.autograd.as_mut() {
            let lhs_clone = lhs.clone();
            let rhs_clone = rhs.clone();
            let res_clone = res.clone();

            let cell = Rc::new(OnceCell::new());
            let cell_clone = cell.clone();
            let lhs_grad = move |
                encoder: &mut TensorEncoder<'scope>,
                output: &Tensor<'scope>| {
                let scaled = cell_clone.get_or_init(|| {
                    let difference = encoder.subtract(&lhs_clone, &rhs_clone)?;
                    encoder.divide(output, &difference)
                }).clone()?;
                let numerator = encoder.subtract(&res_clone, &rhs_clone)?;
                encoder.multiply(&scaled, &numerator)
            };

            let lhs_clone = lhs.clone();
            let rhs_clone = rhs.clone();
            let res_clone = res.clone();
            let rhs_grad = move |
                encoder: &mut TensorEncoder<'scope>,
                output: &Tensor<'scope>| {
                let scaled = cell.get_or_init(|| {
                    let difference = encoder.subtract(&lhs_clone, &rhs_clone)?;
                    encoder.divide(output, &difference)
                }).clone()?;
                let numerator = encoder.subtract(&lhs_clone, &res_clone)?;
                encoder.multiply(&scaled, &numerator)
            };

            autograd.backwards_binary(
                lhs, rhs, &res,
                lhs_grad,
                rhs_grad
            );
        }

        Ok(res)
    }

    fn binary(
        &mut self,
        operation: BinaryOperation,
        operand_one: &Tensor<'scope>,
        operand_two: &Tensor<'scope>,
    ) -> Result<Tensor<'scope>, TensorError> {

        let mut shape = operand_one.shape();
        let mut params = BinaryParameters::zeroed(); 
        params.operation = operation; 
        params.length = 1;

        let mut zero = false;
        for (dst, src) in shape.iter_mut().zip(operand_two.shape()) {
            *dst = BroadcastInfo::boradcast_dimension(*dst, src)?;
            params.length = params.length.checked_mul(*dst)
                .ok_or(TensorError::OversizedDispatch)?;
            zero |= *dst == 0;
        }

        let output = self.temp(shape)?;
        if zero { return Ok(output) }
        params.info = BroadcastInfo::new(
            operand_one.shape(),
            operand_two.shape()
        )?;

        let compute_pass = self.encoder.compute(
            Pipelines::binary,
            &params
        );

        operand_one.bind(compute_pass, 1, true);
        operand_two.bind(compute_pass, 2, true);
        output.bind(compute_pass, 3, false);
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
    fn backwards_binary(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>,
        res: &Tensor<'scope>,
        lhs_gradient: impl FnOnce(
            &mut TensorEncoder<'scope>,
            &Tensor<'scope>,
        ) -> Result<Tensor<'scope>, TensorError> + 'scope,
        rhs_gradient: impl FnOnce(
            &mut TensorEncoder<'scope>,
            &Tensor<'scope>,
        ) -> Result<Tensor<'scope>, TensorError> + 'scope,
    ) {
        let lhs_required = self.require([res], lhs);
        let rhs_required = self.require([res], rhs);
        
        let lhs_gradient = lhs_required.then_some(lhs_gradient);
        let rhs_gradient = rhs_required.then_some(rhs_gradient);

        let lhs_shape = lhs.shape();
        let rhs_shape = rhs.shape();
        let res_shape = res.shape();

        if lhs_required || rhs_required {
            let res_weak = res.downgrade();
            let lhs_weak = lhs.downgrade();
            let rhs_weak = rhs.downgrade();

            self.backwards(move |encoder, gradients| {
                let output = match gradients.remove(res_weak) {
                    Some(output) => output,
                    None => return Ok(())
                };

                if let Some(lhs_gradient) = lhs_gradient {
                    let gradient = lhs_gradient(encoder, &output)?;
                    let diff = ShapeDiff::new(lhs_shape, res_shape);
                    let gradient = encoder.sum(&gradient, diff)?;
                    gradients.insert(encoder, lhs_weak, gradient)?;
                }

                if let Some(rhs_gradient) = rhs_gradient {
                    let gradient = rhs_gradient(encoder, &output)?;
                    let diff = ShapeDiff::new(rhs_shape, res_shape);
                    let gradient = encoder.sum(&gradient, diff)?;
                    gradients.insert(encoder, rhs_weak, gradient)?;
                }
                
                Ok(())
            });
        } 
    }
}

impl Pipelines {
    fn binary(&mut self) -> &ComputePipeline {
        self.binary.get_or_insert_with(|| {
            let module = self.device.create_shader_module(
                include_wgsl!(concat!(env!("OUT_DIR"), "/binary.wgsl"))
            );

            let param_layout = self.param_layouts
                .get::<BinaryParameters>(&self.device);

            let layout = self.device.create_pipeline_layout(
                &PipelineLayoutDescriptor {
                    label: Some("Binary"),
                    immediate_size: 0,
                    bind_group_layouts: &[
                        Some(param_layout),
                        Some(&self.tensor_input_layout),
                        Some(&self.tensor_input_layout),
                        Some(&self.tensor_output_layout)
                    ]
                }
            );

            self.device.create_compute_pipeline(
                &ComputePipelineDescriptor {
                    label: Some("Binary"),
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
