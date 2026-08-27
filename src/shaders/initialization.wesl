import package::share::indices::invocationIndex;

@group(0) @binding(0)
var<uniform> params: InitializationParameters;
@group(1) @binding(0)
var<storage, read_write> output: array<f32>;

struct InitializationParameters {
    length: u32,
    operation: u32,
    constantValue: f32,
    randomLower: f32,
    randomUpper: f32,
    seedLow: u32,
    seedHigh: u32,
    stream: u32,
}

const CONSTANT: u32 = 0;
const RANDOM_UNIFORM: u32 = 1;

struct Builtin {
    @builtin(global_invocation_id) invocation: vec3u,
    @builtin(num_workgroups) numWorkgroups: vec3u
}

fn pcgHash(input: u32) -> u32 {
    let state = input * 747796405u + 2891336453u;
    let word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn randomBits(index: u32) -> u32 {
    let low = pcgHash(params.seedLow ^ 0xa511e9b3u);
    let high = pcgHash(params.seedHigh ^ low);
    let stream = pcgHash(params.stream ^ high);
    return pcgHash(index ^ stream);
}

@compute @workgroup_size(256)
fn main(builtin: Builtin) {
    let index = invocationIndex(
        builtin.invocation,
        builtin.numWorkgroups
    );

    if (index >= params.length) { return; }

    switch (params.operation) {
        case CONSTANT: {
            output[index] = params.constantValue;
        }
        case RANDOM_UNIFORM: {
            let unit = f32(randomBits(index) >> 8u) / 16777216.0;
            output[index] = params.randomLower
                + unit * (params.randomUpper - params.randomLower);
        }
        default: {
            output[index] = 0.0;
        }
    }
}
