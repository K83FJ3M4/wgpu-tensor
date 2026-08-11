
use crate::{Tensor, TensorEncoder};
use wgpu::ComputePipeline;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/output/generated.rs"));

#[derive(Default)]
pub(super) struct Pipelines {
    add: Option<ComputePipeline>
}

impl<'scope> TensorEncoder<'scope> {
    pub fn add(&mut self, input_a: &Tensor, input_b: &Tensor) -> Option<Tensor<'scope>> {
        let shape = input_a.broadcast(input_b)?;
        let output = self.temp(shape);

        let compute_pass = self.encoders.compute(); 
        let pipeline = self.pipelines.add.get_or_insert_with(|| {
            shaders::add::main(&mut self.bind_group_layouts)
        });

        input_a.bind(compute_pass, 0, true);
        input_b.bind(compute_pass, 1, true);
        output.bind(compute_pass, 2, false);
        compute_pass.set_pipeline(pipeline);

        Some(output)
    }
}