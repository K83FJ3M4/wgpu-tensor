
use wgpu::ComputePipeline;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/output/generated.rs"));

mod binary;

#[derive(Default)]
pub(super) struct Pipelines {
    binary: Option<ComputePipeline>
}