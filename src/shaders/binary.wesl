import package::share::indices::invocationIndex;
import package::share::broadcast::{
    BroadcastInfo,
    tensorIndex,
    flattenTensorIndex
};

@group(0) @binding(0)
var<uniform> params: BinaryParameters;
@group(1) @binding(0)
var<storage, read> inputA: array<f32>;
@group(2) @binding(0)
var<storage, read> inputB: array<f32>;
@group(3) @binding(0)
var<storage, read_write> output: array<f32>;

struct BinaryParameters {
    info: BroadcastInfo,
    length: u32,
    operation: u32,
    pad_0: u32,
    pad_1: u32
}

const ADD: u32 = 0;
const SUBTRACT: u32 = 1;
const MULTIPLY: u32 = 2;
const DIVIDE: u32 = 3;
const POWER: u32 = 4;
const MINIMUM: u32 = 5;
const MAXIMUM: u32 = 6;
const REMAINDER: u32 = 7;
const EQUAL: u32 = 8;

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

    let lhs = inputA[indices.x];
    let rhs = inputB[indices.y];

    switch (params.operation) {
        case ADD: { output[outputIndex] = lhs + rhs; }
        case SUBTRACT: { output[outputIndex] = lhs - rhs; }
        case MULTIPLY: { output[outputIndex] = lhs * rhs; }
        case DIVIDE: { output[outputIndex] = lhs / rhs; }
        case POWER: { output[outputIndex] = power(lhs, rhs); }
        case MINIMUM: { output[outputIndex] = minimum(lhs, rhs); }
        case MAXIMUM: { output[outputIndex] = maximum(lhs, rhs); }
        case REMAINDER: { output[outputIndex] = remainder(lhs, rhs); }
        case EQUAL: {
            output[outputIndex] = select(0.0, 1.0, lhs == rhs);
        }
        default: { output[outputIndex] = 0.0; }
    }
}

fn power(lhs: f32, rhs: f32) -> f32 {
    let negative = (bitcast<u32>(lhs) & 0x80000000u) != 0u;
    let integralExponent = fract(rhs) == 0.0;

    if (lhs < 0.0) {
        if (!integralExponent) { return nanFrom(lhs); }

        let result = pow(-lhs, rhs);
        if (rhs % 2.0 == 0.0) { return result; }
        return -result;
    }

    if (
        lhs == 0.0 &&
        negative &&
        integralExponent &&
        rhs % 2.0 != 0.0
    ) {
        return -pow(0.0, rhs);
    }

    return pow(lhs, rhs);
}

fn minimum(lhs: f32, rhs: f32) -> f32 {
    if (lhs != lhs) { return lhs; }
    if (rhs != rhs) { return rhs; }
    return min(lhs, rhs);
}

fn maximum(lhs: f32, rhs: f32) -> f32 {
    if (lhs != lhs) { return lhs; }
    if (rhs != rhs) { return rhs; }
    return max(lhs, rhs);
}

fn remainder(lhs: f32, rhs: f32) -> f32 {
    if (lhs != lhs) { return lhs; }
    if (rhs != rhs) { return rhs; }

    let lhsBits = bitcast<u32>(lhs);
    if (
        rhs == 0.0 ||
        (lhsBits & 0x7fffffffu) == 0x7f800000u
    ) {
        return nanFrom(lhs);
    }

    let value = lhs % rhs;
    if (value == 0.0) {
        return bitcast<f32>(bitcast<u32>(rhs) & 0x80000000u);
    }

    if ((value < 0.0) != (rhs < 0.0)) {
        return value + rhs;
    }

    return value;
}

fn nanFrom(value: f32) -> f32 {
    return bitcast<f32>(
        (bitcast<u32>(value) & 0x003fffffu) |
        0x7fc00000u
    );
}
