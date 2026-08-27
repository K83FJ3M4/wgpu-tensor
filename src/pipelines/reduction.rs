use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};

use crate::optimizers::AutogradEncoder;
use crate::pipelines::Pipelines;
use crate::tensor::ShapeDiff;
use crate::{IntoIndices, Tensor, TensorEncoder, TensorError};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct ReductionParameters {
    pub operation: ReductionOperation,
    pub cluster_shift: u32,

    pub inner_size: u32,
    pub reduction_size: u32,
    pub outer_size: u32
}

#[repr(transparent)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq)]
pub struct ReductionOperation(u32);

impl ReductionOperation {
    pub const SUM: Self = Self(0);
    pub const PRODUCT: Self = Self(1);
    pub const MINIMUM: Self = Self(2);
    pub const MAXIMUM: Self = Self(3);
}

impl<'scope> TensorEncoder<'scope> {
    pub fn sum(
        &mut self,
        operand: &Tensor<'scope>,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.reduction(
            ReductionOperation::SUM,
            operand,
            dimensions
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let operand_shape = operand.shape();
            autograd.backwards_reduction(
                operand,
                &res,
                move |encoder, output_grad| {
                    encoder.broadcast(output_grad, operand_shape)
                }
            );
        }

        Ok(res)
    }

    pub fn mean(
        &mut self,
        operand: &Tensor<'scope>,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        let operand_shape = operand.shape();
        let mut output_shape = operand_shape;
        let mut count = 1u32;

        for index in dimensions.indices() {
            let Some(dimension) = output_shape.get_mut(index) else {
                return Err(TensorError::IndexOutOfBounds)
            };

            count = count.checked_mul(*dimension)
                .ok_or(TensorError::OversizedDispatch)?;
            *dimension = 1;
        }

        let sum = self.sum(
            operand,
            ShapeDiff::new(operand_shape, output_shape)
        )?;
        let count = self.constant(count as f32, 1)?;
        self.divide(&sum, &count)
    }

    pub fn prod(
        &mut self,
        operand: &Tensor<'scope>,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.reduction(
            ReductionOperation::PRODUCT,
            operand,
            dimensions
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let operand_value = operand.clone();
            let result = res.clone();
            let operand_shape = operand_value.shape();
            let result_shape = result.shape();

            autograd.backwards_reduction(
                operand,
                &res,
                move |encoder, output_grad| {
                    let zero = encoder.zeros(1)?;
                    let one = encoder.ones(1)?;
                    let zero_mask = encoder.equal(&operand_value, &zero)?;
                    let safe_operand = encoder.add(
                        &operand_value,
                        &zero_mask
                    )?;

                    let zero_count = encoder.sum(
                        &zero_mask,
                        ShapeDiff::new(result_shape, operand_shape)
                    )?;
                    let nonzero_product = encoder.prod(
                        &safe_operand,
                        ShapeDiff::new(result_shape, operand_shape)
                    )?;

                    let no_zeros = encoder.equal(&zero_count, &zero)?;
                    let one_zero = encoder.equal(&zero_count, &one)?;

                    let normal = encoder.divide(&result, &safe_operand)?;
                    let normal = encoder.multiply(&normal, &no_zeros)?;

                    let single_zero = encoder.multiply(
                        &zero_mask,
                        &nonzero_product
                    )?;
                    let single_zero = encoder.multiply(
                        &single_zero,
                        &one_zero
                    )?;

                    let derivative = encoder.add(&normal, &single_zero)?;
                    let output_grad = encoder.broadcast(
                        output_grad,
                        operand_shape
                    )?;
                    encoder.multiply(&output_grad, &derivative)
                }
            );
        }

        Ok(res)
    }

    pub fn min(
        &mut self,
        operand: &Tensor<'scope>,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        self.extremum_reduction(
            ReductionOperation::MINIMUM,
            operand,
            dimensions
        )
    }

    pub fn max(
        &mut self,
        operand: &Tensor<'scope>,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        self.extremum_reduction(
            ReductionOperation::MAXIMUM,
            operand,
            dimensions
        )
    }

    fn extremum_reduction(
        &mut self,
        operation: ReductionOperation,
        operand: &Tensor<'scope>,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.reduction(operation, operand, dimensions)?;

        if let Some(autograd) = self.autograd.as_mut() {
            let operand_value = operand.clone();
            let result = res.clone();
            let operand_shape = operand_value.shape();
            let result_shape = result.shape();

            autograd.backwards_reduction(
                operand,
                &res,
                move |encoder, output_grad| {
                    let result = encoder.broadcast(
                        &result,
                        operand_shape
                    )?;
                    let mask = encoder.equal(&operand_value, &result)?;
                    let count = encoder.sum(
                        &mask,
                        ShapeDiff::new(result_shape, operand_shape)
                    )?;
                    let output_grad = encoder.divide(output_grad, &count)?;
                    let output_grad = encoder.broadcast(
                        &output_grad,
                        operand_shape
                    )?;
                    encoder.multiply(&output_grad, &mask)
                }
            );
        }

        Ok(res)
    }
    
    fn reduction(
        &mut self,
        operation: ReductionOperation,
        operand: &Tensor<'scope>,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {

        let mut output_shape = operand.shape();
        for index in dimensions.indices() {
            if let Some(dimension) = output_shape.get_mut(index) {
                *dimension = 1;
            } else {
                return Err(TensorError::IndexOutOfBounds)
            }
        }

        let mut temp = Option::<Tensor<'scope>>::None;

        loop {
            let input = temp.as_ref().unwrap_or(operand);

            let Some(dimension) = input.shape().iter()
                .zip(output_shape.iter())
                .position(|(input, output)| input != output)
            else {
                let output = self.temp(input.shape())?;
                self.copy_inner(input, &output)?;
                return Ok(output)
            };

            let mut params = ReductionParameters::zeroed();
            params.operation = operation;
            params.reduction_size = input.shape()[dimension];
            params.inner_size = input.shape()[..dimension].iter()
                .try_fold(1u32, |lhs, rhs| lhs.checked_mul(*rhs))
                .ok_or(TensorError::OversizedDispatch)?;
            params.outer_size = input.shape()[dimension + 1..].iter()
                .try_fold(1u32, |lhs, rhs| lhs.checked_mul(*rhs))
                .ok_or(TensorError::OversizedDispatch)?;
            params.cluster_shift = params.reduction_size.min(256)
                .next_power_of_two().max(2)
                .trailing_zeros();

            let mut shape = input.shape();
            shape[dimension] =  ((params.reduction_size.max(1) - 1)
                >> params.cluster_shift) + 1;

            let output = self.temp(shape)?;
            let compute_pass = self.encoder.compute(
                Pipelines::reduction,
                &params
            );

            input.bind(compute_pass, 1, true);
            output.bind(compute_pass, 2, false);

            let cluster_count =
                params.inner_size
                .checked_mul(shape[dimension])
                .ok_or(TensorError::OversizedDispatch)?
                .checked_mul(params.outer_size)
                .ok_or(TensorError::OversizedDispatch)?;

            let clusters_per_workgroup =
                256 >> params.cluster_shift;

            let num_workgroups = cluster_count.div_ceil(clusters_per_workgroup);
            if num_workgroups != 0 && num_workgroups <= u16::MAX as u32 {
                compute_pass.dispatch_workgroups(num_workgroups, 1, 1); 
            } else if num_workgroups != 0 {
                let floor = num_workgroups.isqrt();
                let x = floor + (floor * floor != num_workgroups) as u32;
                let y = num_workgroups.div_ceil(x);
                compute_pass.dispatch_workgroups(x, y, 1);
            }

            if output.shape() == output_shape {
                return Ok(output)
            } else {
                temp = Some(output)
            }
        }
    }
}

impl<'scope> AutogradEncoder<'scope> {
    fn backwards_reduction(
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
    fn reduction(&mut self) -> &ComputePipeline {
        self.reduction.get_or_insert_with(|| {
            let module = self.device.create_shader_module(
                include_wgsl!(concat!(env!("OUT_DIR"), "/reduction.wgsl"))
            );

            let param_layout = self.param_layouts
                .get::<ReductionParameters>(&self.device);

            let layout = self.device.create_pipeline_layout(
                &PipelineLayoutDescriptor {
                    label: Some("Reduction"),
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
                    label: Some("Reduction"),
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
