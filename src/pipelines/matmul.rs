use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};
use crate::{Tensor, TensorEncoder, TensorError};
use crate::pipelines::{BroadcastInfo, Pipelines};
use crate::tensor::ShapeDiff;

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct MatmulParameters {
    info: BroadcastInfo,
    inner_size: u32,
    tile_count: u32,
    size: [u32; 2]
}

impl<'scope> TensorEncoder<'scope> {
    pub fn matmul(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>
    ) -> Result<Tensor<'scope>, TensorError> {
        self.matmul_with_transpose(lhs, rhs, false, false)
    }

    pub fn matmul_with_transpose(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>,
        transpose_lhs: bool,
        transpose_rhs: bool
    ) -> Result<Tensor<'scope>, TensorError> {
        let res = self.matmul_inner(
            lhs,
            rhs,
            transpose_lhs,
            transpose_rhs
        )?;

        if let Some(autograd) = self.autograd.as_mut() {
            let lhs_required = autograd.require([&res], lhs);
            let rhs_required = autograd.require([&res], rhs);

            if lhs_required || rhs_required {
                let res_weak = res.downgrade();
                let lhs_weak = lhs.downgrade();
                let rhs_weak = rhs.downgrade();
                let lhs_shape = lhs.shape();
                let rhs_shape = rhs.shape();

                let lhs_value = rhs_required.then(|| lhs.clone());
                let rhs_value = lhs_required.then(|| rhs.clone());

                autograd.backwards(move |encoder, gradients| {
                    let Some(output_grad) = gradients.remove(res_weak) else {
                        return Ok(());
                    };

                    if let Some(rhs) = rhs_value {
                        let lhs_grad = if transpose_lhs {
                            encoder.matmul_with_transpose(
                                &rhs,
                                &output_grad,
                                transpose_rhs,
                                true
                            )?
                        } else {
                            encoder.matmul_with_transpose(
                                &output_grad,
                                &rhs,
                                false,
                                !transpose_rhs
                            )?
                        };

                        let diff = ShapeDiff::new(
                            lhs_shape,
                            lhs_grad.shape()
                        );
                        let lhs_grad = encoder.sum(&lhs_grad, diff)?;
                        gradients.insert(
                            encoder,
                            lhs_weak,
                            lhs_grad,
                        )?;
                    }

                    if let Some(lhs) = lhs_value {
                        let rhs_grad = if transpose_rhs {
                            encoder.matmul_with_transpose(
                                &output_grad,
                                &lhs,
                                true,
                                transpose_lhs
                            )?
                        } else {
                            encoder.matmul_with_transpose(
                                &lhs,
                                &output_grad,
                                !transpose_lhs,
                                false
                            )?
                        };

                        let diff = ShapeDiff::new(
                            rhs_shape,
                            rhs_grad.shape()
                        );
                        let rhs_grad = encoder.sum(&rhs_grad, diff)?;
                        gradients.insert(
                            encoder,
                            rhs_weak,
                            rhs_grad,
                        )?;
                    }

                    Ok(())
                });
            }
        }

        Ok(res)
    }

    fn matmul_inner(
        &mut self,
        lhs: &Tensor<'scope>,
        rhs: &Tensor<'scope>,
        transpose_lhs: bool,
        transpose_rhs: bool
    ) -> Result<Tensor<'scope>, TensorError> {
        let lhs_shape = lhs.shape();
        let rhs_shape = rhs.shape();

        let lhs_matrix_shape = if transpose_lhs {
            [lhs_shape[1], lhs_shape[0]]
        } else {
            [lhs_shape[0], lhs_shape[1]]
        };
        let rhs_matrix_shape = if transpose_rhs {
            [rhs_shape[1], rhs_shape[0]]
        } else {
            [rhs_shape[0], rhs_shape[1]]
        };

        let [lhs_columns, lhs_rows] = lhs_matrix_shape;
        let [rhs_columns, rhs_rows] = rhs_matrix_shape;

        let mut params = MatmulParameters::zeroed();
        if lhs_columns != rhs_rows {
            return Err(TensorError::IncompatibleMatrices)
        }

        params.inner_size = lhs_columns;
        params.size = [rhs_columns, lhs_rows];

        let mut shape = lhs_shape;
        shape[0] = params.size[0];
        shape[1] = params.size[1];

        let mut zero = params.size.contains(&0);
        for index in 2..shape.len() {
            shape[index] = BroadcastInfo::boradcast_dimension(
                lhs_shape[index],
                rhs_shape[index]
            )?;
            zero |= shape[index] == 0;
        }

        let output = self.temp(shape)?;
        if zero { return Ok(output) }

        let tiles = params.size.map(|dimension| dimension.div_ceil(16));
        params.tile_count = tiles.into_iter()
            .chain(shape[2..].iter().copied())
            .try_fold(1u32, u32::checked_mul)
            .ok_or(TensorError::OversizedDispatch)?;
        params.info = BroadcastInfo::with_prefix(
            lhs_shape,
            rhs_shape,
            &tiles
        )?;

        params.info.set_data(0, transpose_lhs as u32);
        params.info.set_data(1, transpose_rhs as u32);
        let compute_pass = self.encoder.compute(
            Pipelines::matmul,
            &params
        );

        lhs.bind(compute_pass, 1, true);
        rhs.bind(compute_pass, 2, true);
        output.bind(compute_pass, 3, false);

        let num_workgroups = params.tile_count;
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
    fn matmul(&self) -> &ComputePipeline {
        self.matmul.get_or_init(|| {
            let module = self.device.create_shader_module(
                include_wgsl!(concat!(env!("OUT_DIR"), "/matmul.wgsl"))
            );

            let param_layout = self.param_layout::<MatmulParameters>(&self.device);
            let layout = self.device.create_pipeline_layout(
                &PipelineLayoutDescriptor {
                    label: Some("Matmul"),
                    immediate_size: 0,
                    bind_group_layouts: &[
                        Some(&param_layout),
                        Some(&self.tensor_input_layout),
                        Some(&self.tensor_input_layout),
                        Some(&self.tensor_output_layout)
                    ]
                }
            );

            self.device.create_compute_pipeline(
                &ComputePipelineDescriptor {
                    label: Some("Matmul"),
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
