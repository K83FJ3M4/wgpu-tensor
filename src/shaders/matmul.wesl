import package::share::indices::workgroupIndex;
import package::share::broadcast::{
    BroadcastInfo,
    tensorIndex,
    flattenTensorIndex
};

@group(0) @binding(0)
var<uniform> params: MatmulParameters;
@group(1) @binding(0)
var<storage, read> inputA: array<f32>;
@group(2) @binding(0)
var<storage, read> inputB: array<f32>;
@group(3) @binding(0)
var<storage, read_write> output: array<f32>;

struct MatmulParameters {
    info: BroadcastInfo,
    tilesPerMatrix: u32,
    pad_0: u32,
    pad_1: u32,
    pad_2: u32 
}

struct Builtin {
    @builtin(workgroup_id) workgroup: vec3u,
    @builtin(num_workgroups) numWorkgroups: vec3u
}

@compute @workgroup_size(16, 16)
fn main(builtin: Builtin) {
    let tileIndex = workgroupIndex(
        builtin.workgroup,
        builtin.numWorkgroups
    );

    let outputMatrixIndex = tileIndex / params.tilesPerMatrix;
    let outputMatrixTensorIndex = tensorIndex(
        params.info,
        outputMatrixIndex
    );

    let inputMatrixIndex = flattenTensorIndex(
        params.info,
        outputMatrixTensorIndex
    );


}