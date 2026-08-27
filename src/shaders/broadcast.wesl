import package::share::indices::invocationIndex;
import package::share::broadcast::{
    BroadcastInfo,
    tensorIndex,
    flattenTensorIndex
};

@group(0) @binding(0)
var<uniform> params: BroadcastParameters;
@group(1) @binding(0)
var<storage, read> input: array<f32>;
@group(2) @binding(0)
var<storage, read_write> output: array<f32>;

struct BroadcastParameters {
    info: BroadcastInfo,
    length: u32,
    pad_0: u32,
    pad_1: u32,
    pad_2: u32
}

struct Builtin {
    @builtin(global_invocation_id) invocation: vec3u,
    @builtin(num_workgroups) numWorkgroups: vec3u
}

@compute @workgroup_size(256)
fn main(builtin: Builtin) {
    var index = invocationIndex(
        builtin.invocation,
        builtin.numWorkgroups
    );
    let outputIndex = index;

    if (index >= params.length) { return; }

    let tensorIndex = tensorIndex(
        params.info,
        index,
    );

    let indices = flattenTensorIndex(
        params.info,
        tensorIndex
    );

    output[outputIndex] = input[indices.x];
}