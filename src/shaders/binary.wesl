import package::share::invocation::invocationIndex;

@group(0) @binding(0)
var<uniform> params: BinaryParameters;
@group(1) @binding(0)
var<storage, read> inputA: array<f32>;
@group(2) @binding(0)
var<storage, read> inputB: array<f32>;
@group(3) @binding(0)
var<storage, read_write> output: array<f32>;

struct FastDivU32 {
    divisor: u32,
    magic: u32,
    shift: u32,
    pad: u32
}

struct BinaryParameters {
    divisions: array<FastDivU32, 7>,
    numDimensions: u32,
    length: u32,
    masks: u32,
    operation: u32
}

const ADD: u32 = 0;
const SUBTRACT: u32 = 1;
const MULTIPLY: u32 = 2;
const DIVIDE: u32 = 3;
const POWER: u32 = 4;
const MINIMUM: u32 = 5;
const MAXIMUM: u32 = 6;
const REMAINDER: u32 = 7;

struct Builtin {
    @builtin(global_invocation_id) invocation: vec3u,
    @builtin(num_workgroups) numWorkgroups: vec3u
}

@compute @workgroup_size(256)
fn binary(builtin: Builtin) {
    var index = invocationIndex(
        builtin.invocation,
        builtin.numWorkgroups
    );
    let outputIndex = index;

    if (index >= params.length) { return; }

    let maxDimension = max(params.numDimensions, 1u) - 1u;
    var tensorIndex: array<u32, 8>;

    for (var i = 0u; i < maxDimension; i += 1u) {
        let division = params.divisions[i];
        let quotient = fastDiv(division, index);
        tensorIndex[i] = index - quotient * division.divisor;
        index = quotient;
    }

    tensorIndex[maxDimension] = index;
    let indices = flattenIndex(tensorIndex);
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
        default: { output[outputIndex] = 0.0; }
    }
}

fn flattenIndex(tensorIndex: array<u32, 8>) -> vec2u {
    let masks = vec2u(
        params.masks & 0xffffu,
        params.masks >> 16u
    );
    let maxIndex = max(params.numDimensions, 1u) - 1u;
    let highestValid = vec2u(
        (masks.x >> maxIndex) & 1u,
        (masks.y >> maxIndex) & 1u
    );
    var flat = vec2u(tensorIndex[maxIndex]) * highestValid;
    var i = maxIndex;

    while (i != 0u) {
        i -= 1u;
        let valid = vec2u(
            (masks.x >> i) & 1u,
            (masks.y >> i) & 1u
        );
        let size = vec2u(1u) + valid * vec2u(
            params.divisions[i].divisor - 1u
        );
        let item = vec2u(tensorIndex[i]) * valid;
        flat = flat * size + item;
    }

    return flat;
}

fn fastDiv(division: FastDivU32, numerator: u32) -> u32 {
    if (division.divisor == 1u) { return numerator; }

    let quotient = mulHiU32(numerator, division.magic);
    let intermediate = ((numerator - quotient) >> 1u) + quotient;
    return intermediate >> division.shift;
}

fn mulHiU32(lhs: u32, rhs: u32) -> u32 {
    let lhsLow = lhs & 0xffffu;
    let lhsHigh = lhs >> 16u;
    let rhsLow = rhs & 0xffffu;
    let rhsHigh = rhs >> 16u;
    let lowProduct = lhsLow * rhsLow;
    let first = lhsHigh * rhsLow + (lowProduct >> 16u);
    let firstLow = first & 0xffffu;
    let firstHigh = first >> 16u;
    let second = lhsLow * rhsHigh + firstLow;

    return lhsHigh * rhsHigh + firstHigh + (second >> 16u);
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
