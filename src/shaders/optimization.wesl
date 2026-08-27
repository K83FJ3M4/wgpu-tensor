import package::share::indices::invocationIndex;

@group(0) @binding(0)
var<uniform> params: OptimizationParams;
@group(1) @binding(0)
var<storage, read> gradients: array<f32>;
@group(2) @binding(0)
var<storage, read_write> weights: array<f32>;

struct OptimizationParams {
    learningRate: f32,
    length: u32,
}

struct Builtin {
    @builtin(global_invocation_id) invocation: vec3u,
    @builtin(num_workgroups) numWorkgroups: vec3u
}

@compute @workgroup_size(256)
fn main(builtin: Builtin) {
    let index = invocationIndex(
        builtin.invocation,
        builtin.numWorkgroups
    );

    if (index >= params.length) { return; }
    weights[index] -= params.learningRate * gradients[index];
}

