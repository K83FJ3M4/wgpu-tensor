import package::share::indices::invocationIndex;

@group(0) @binding(0)
var<uniform> params: ConstantParams;
@group(1) @binding(0)
var<storage, read_write> output: array<f32>;

struct ConstantParams {
    length: u32,
    value: f32
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
    output[index] = params.value;
}

