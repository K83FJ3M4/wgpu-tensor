import package::share::indices::invocationIndex;

@group(0) @binding(0)
var<uniform> params: UniformParams;
@group(1) @binding(0)
var<storage, read> input: array<f32>;
@group(2) @binding(0)
var<storage, read_write> output: array<f32>;

struct UniformParams {
    length: u32,
    operation: u32
}

const NEGATE: u32 = 0;
const ABSOLUTE: u32 = 1;
const RECIPROCAL: u32 = 2;
const SQUARE_ROOT: u32 = 3;
const RECIPROCAL_SQUARE_ROOT: u32 = 4;
const EXPONENTIAL: u32 = 5;
const LOGARITHM: u32 = 6;
const COPY: u32 = 7;
const SIGN: u32 = 8;

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
    let item = input[index];

    switch (params.operation) {
        case NEGATE: { output[index] = -item; }
        case ABSOLUTE: { output[index] = abs(item); }
        case RECIPROCAL: { output[index] = 1.0 / item; }
        case SQUARE_ROOT: { output[index] = sqrt(item); }
        case RECIPROCAL_SQUARE_ROOT: { output[index] = inverseSqrt(item); }
        case EXPONENTIAL: { output[index] = exp(item); }
        case LOGARITHM: { output[index] = log(item); }
        case COPY: { output[index] = item; }
        case SIGN: { output[index] = sign(item); }
        default: { output[index] = 0.0; }
    }
}

fn sign(value: f32) -> f32 {
    if (value != value) { return value; }
    if (value < 0.0) { return -1.0; }
    if (value > 0.0) { return 1.0; }
    return 0.0;
}
