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

var<workgroup> sharedLHS: array<f32, 256>;
var<workgroup> sharedRHS: array<f32, 256>;

struct MatmulParameters {
    info: BroadcastInfo,
    innerSize: u32,
    tileCount: u32,
    //vec2u(columns, rows)
    size: vec2u,
}

struct Builtin {
    @builtin(local_invocation_index) invocation: u32,
    @builtin(workgroup_id) workgroup: vec3u,
    @builtin(num_workgroups) numWorkgroups: vec3u
}

@compute @workgroup_size(256)
fn main(builtin: Builtin) {
    let tileIndex = workgroupIndex(
        builtin.workgroup,
        builtin.numWorkgroups
    );

    if (tileIndex >= params.tileCount) { return; }

    let outputMatrixTensorIndex = tensorIndex(
        params.info,
        tileIndex
    );

    let inputMatrixIndex = flattenTensorIndex(
        params.info,
        outputMatrixTensorIndex
    );

    let tile = vec2u(
        outputMatrixTensorIndex[0],
        outputMatrixTensorIndex[1]
    );

    let localIndex = vec2u(
        builtin.invocation & 15u,
        builtin.invocation >> 4u
    );

    let matrixIndex = tile * 16u + localIndex;
    let matrixOffset = inputMatrixIndex
        * params.size.yx * params.innerSize;

    var accumulator = 0.0;

    for (var i = 0u; i < params.innerSize; i += 16u) {
        let inner = i + localIndex;
        let matrixIndexValid = matrixIndex < params.size;
        let innerIndexValid = inner < vec2u(params.innerSize);
        let indexValid = matrixIndexValid.yx & innerIndexValid;

        if (indexValid.x) {
            sharedLHS[builtin.invocation] = inputA[
                matrixOffset.x +
                matrixIndex.y * params.innerSize +
                inner.x
            ];
        } else {
            sharedLHS[builtin.invocation] = 0.0;
        }

        if (indexValid.y) {
            sharedRHS[builtin.invocation] = inputB[
                matrixOffset.y +
                inner.y * params.size.x +
                matrixIndex.x
            ];
        } else {
            sharedRHS[builtin.invocation] = 0.0;
        }

        workgroupBarrier();

        for (var k = 0u; k < 16u; k++) {
            let lhs = sharedLHS[localIndex.y * 16u + k];
            let rhs = sharedRHS[k * 16u + localIndex.x];
            accumulator += lhs * rhs;
        }

        workgroupBarrier();
    }

    if (all(matrixIndex < params.size)) {
        let outputMatrixIndex = flattenOutputMatrixIndex(
            outputMatrixTensorIndex
        );
        let outputMatrixOffset =
            outputMatrixIndex * params.size.x * params.size.y;
        output[
            outputMatrixOffset +
            matrixIndex.y * params.size.x +
            matrixIndex.x
        ] = accumulator;
    }
}

fn flattenOutputMatrixIndex(tensorIndex: array<u32, 8>) -> u32 {
    let numDimensions = params.info.divisions[0].data;
    if (numDimensions <= 2u) { return 0u; }

    let maxIndex = numDimensions - 1u;
    var flat = tensorIndex[maxIndex];

    for (var i = maxIndex; i != 2u;) {
        i -= 1u;
        flat = flat * params.info.divisions[i].divisor +
            tensorIndex[i];
    }

    return flat;
}