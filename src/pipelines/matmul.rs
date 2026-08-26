use bytemuck::{Pod, Zeroable};
use wgpu::{ComputePipeline, ComputePipelineDescriptor, PipelineLayoutDescriptor, include_wgsl};
use crate::{Tensor, TensorEncoder, TensorError};
use crate::pipelines::{BroadcastInfo, Pipelines};

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
        let lhs_shape = lhs.shape();
        let rhs_shape = rhs.shape();

        let mut params = MatmulParameters::zeroed();
        if lhs_shape[0] != rhs_shape[1] {
            return Err(TensorError::IncompatibleMatrices)
        }

        params.inner_size = lhs_shape[0];
        params.size = [rhs_shape[0], lhs_shape[1]];

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
    fn matmul(&mut self) -> &ComputePipeline {
        self.matmul.get_or_insert_with(|| {
            let module = self.device.create_shader_module(
                include_wgsl!(concat!(env!("OUT_DIR"), "/matmul.wgsl"))
            );

            let param_layout = self.param_layouts
                .get::<MatmulParameters>(&self.device);

            let layout = self.device.create_pipeline_layout(
                &PipelineLayoutDescriptor {
                    label: Some("Matmul"),
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
