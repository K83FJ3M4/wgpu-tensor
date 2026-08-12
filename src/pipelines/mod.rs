
use crate::{Tensor, TensorEncoder};
use ::shaders::BinaryParameters;
use wgpu::ComputePipeline;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/output/generated.rs"));

#[derive(Default)]
pub(super) struct Pipelines {
    add: Option<ComputePipeline>
}

impl<'scope> TensorEncoder<'scope> {
    pub fn add(&mut self, input_a: &Tensor, input_b: &Tensor) -> Option<Tensor<'scope>> {
        let input_a_shape = input_a.shape();
        let input_b_shape = input_b.shape();
        let mut shape = input_a.broadcast(input_b)?;
        let output = self.temp(shape);

        let compute_pass = self.encoder.compute(
            &mut self.pipelines.add,
            shaders::add::main,
            &BinaryParameters {
                a: 0,
                b: 0,
                c: 0,
                d: 0
            }
        );

        input_a.bind(compute_pass, 0, true);
        input_b.bind(compute_pass, 1, true);
        output.bind(compute_pass, 2, false);

        //TODO add a dispatch

        Some(output)
    }
}
