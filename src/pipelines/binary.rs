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
        operand_one: &Tensor<'scope>,
        operand_two: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::SUBTRACT,
            operand_one,
            operand_two,
        )
    }

    pub fn multiply(
        &mut self,
        operand_one: &Tensor<'scope>,
        operand_two: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::MULTIPLY,
            operand_one,
            operand_two,
        )
    }

    pub fn divide(
        &mut self,
        operand_one: &Tensor<'scope>,
        operand_two: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::DIVIDE,
            operand_one,
            operand_two,
        )
    }

    pub fn power(
        &mut self,
        operand_one: &Tensor<'scope>,
        operand_two: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::POWER,
            operand_one,
            operand_two,
        )
    }

    pub fn minimum(
        &mut self,
        operand_one: &Tensor<'scope>,
        operand_two: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::MINIMUM,
            operand_one,
            operand_two,
        )
    }

    pub fn maximum(
        &mut self,
        operand_one: &Tensor<'scope>,
        operand_two: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::MAXIMUM,
            operand_one,
            operand_two,
        )
    }

    pub fn remainder(
        &mut self,
        operand_one: &Tensor<'scope>,
        operand_two: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.binary(
            BinaryOperation::REMAINDER,
            operand_one,
            operand_two,
        )
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

                if lhs_required {
                    let gradient = lhs_gradient(encoder, &output)?;
                    let diff = ShapeDiff::new(lhs_shape, res_shape);
                    let gradient = encoder.sum(&gradient, diff)?;
                    gradients.insert(encoder, lhs_weak, gradient)?;
                }

                if rhs_required {
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