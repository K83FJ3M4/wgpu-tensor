use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};

use crate::pipelines::Pipelines;
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
        operand: &Tensor,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        self.reduction(
            ReductionOperation::SUM,
            operand,
            dimensions
        )
    }

    pub fn prod(
        &mut self,
        operand: &Tensor,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        self.reduction(
            ReductionOperation::PRODUCT,
            operand,
            dimensions
        )
    }

    pub fn min(
        &mut self,
        operand: &Tensor,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        self.reduction(
            ReductionOperation::MINIMUM,
            operand,
            dimensions
        )
    }

    pub fn max(
        &mut self,
        operand: &Tensor,
        dimensions: impl IntoIndices
    ) -> Result<Tensor<'scope>, TensorError> {
        self.reduction(
            ReductionOperation::MAXIMUM,
            operand,
            dimensions
        )
    }
    
    fn reduction(
        &mut self,
        operation: ReductionOperation,
        operand: &Tensor,
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
                return self.copy(input)
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
                    entry_point: Some("reduction"),
                    module: &module
                }
            )
        })
    }
}