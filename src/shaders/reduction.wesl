@group(0) @binding(0)
var<uniform> params: ReductionParameters;
@group(1) @binding(0)
var<storage, read> input: array<f32>;
@group(2) @binding(0)
var<storage, read_write> output: array<f32>;

var<workgroup> workgroupValues: array<f32, 256>;

struct ReductionParameters {
    operation: u32,
    clusterShift: u32,
    innerSize: u32,
    reductionSize: u32,
    outerSize: u32
}

const SUM: u32 = 0;
const PRODUCT: u32 = 1;
const MINIMUM: u32 = 2;
const MAXIMUM: u32 = 3;

struct Builtin {
    @builtin(local_invocation_id) invocation: vec3u,
    @builtin(workgroup_id) workgroup: vec3u,
    @builtin(num_workgroups) numWorkgroups: vec3u
}

@compute @workgroup_size(256)
fn main(builtin: Builtin) {
    let invocation = builtin.invocation.x;
    let cluster = clusterIndex(
        invocation,
        builtin.workgroup,
        builtin.numWorkgroups,
        params.clusterShift
    );

    let indices = tensorIndices(
        params.innerSize,
        params.reductionSize,
        params.clusterShift,
        cluster
    );

    let clusterSize = 1u << params.clusterShift;
    let laneInCluster = invocation & (clusterSize - 1u);
    let reductionIndex =
        (indices.y << params.clusterShift) + laneInCluster;
    let outerValid = indices.z < params.outerSize;
    let reductionValid = reductionIndex < params.reductionSize;

    if (outerValid && reductionValid) {
        let index = (
            indices.z * params.reductionSize + reductionIndex
        ) * params.innerSize + indices.x;
        workgroupValues[invocation] = input[index];
    } else {
        workgroupValues[invocation] = identity(params.operation);
    }

    workgroupBarrier();

    var stride = clusterSize >> 1u;
    while (stride != 0u) {
        if (laneInCluster < stride) {
            let lhs = workgroupValues[invocation];
            let rhs = workgroupValues[invocation + stride];
            workgroupValues[invocation] = reduce(params.operation, lhs, rhs);
        }

        workgroupBarrier();
        stride >>= 1u;
    }

    if (laneInCluster == 0u && outerValid) {
        output[cluster] = workgroupValues[invocation];
    }
}

fn tensorIndices(
    innerSize: u32,
    reductionSize: u32,
    clusterShift: u32,
    clusterIndex: u32
) -> vec3u {
    let outputReductionSize =
        ((max(reductionSize, 1u) - 1u) >> clusterShift) + 1u;
    let inner = clusterIndex % innerSize;
    let remaining = clusterIndex / innerSize;
    let chunk = remaining % outputReductionSize;
    let outer = remaining / outputReductionSize;

    return vec3u(inner, chunk, outer);
}

fn clusterIndex(
    invocation: u32,
    workgroup: vec3u,
    numWorkgroups: vec3u,
    clusterShift: u32
) -> u32 {
    let workgroupIndex =
        workgroup.x +
        workgroup.y * numWorkgroups.x +
        workgroup.z * numWorkgroups.x * numWorkgroups.y;
    let clustersShift = 8u - clusterShift;

    return
        (workgroupIndex << clustersShift) +
        (invocation >> clusterShift);
}

fn identity(operation: u32) -> f32 {
    switch (operation) {
        case SUM: { return 0.0; }
        case PRODUCT: { return 1.0; }
        case MINIMUM: {
            let sign = (operation - MINIMUM) << 31u;
            return bitcast<f32>(0x7f800000u | sign);
        }
        case MAXIMUM: {
            let sign = (operation - MINIMUM) << 31u;
            return bitcast<f32>(0x7f800000u | sign);
        }
        default: { return 0.0; }
    }
}

fn reduce(operation: u32, lhs: f32, rhs: f32) -> f32 {
    switch (operation) {
        case SUM: { return lhs + rhs; }
        case PRODUCT: { return lhs * rhs; }
        case MINIMUM: { return minimum(lhs, rhs); }
        case MAXIMUM: { return maximum(lhs, rhs); }
        default: { return 0.0; }
    }
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
