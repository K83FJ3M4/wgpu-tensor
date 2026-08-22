
use wgpu::ComputePipeline;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/output/generated.rs"));

mod reduction;
mod binary;
mod unary;

#[derive(Default)]
pub(super) struct Pipelines {
    reduction: Option<ComputePipeline>,
    binary: Option<ComputePipeline>,
    unary: Option<ComputePipeline>
}