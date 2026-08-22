use glam::UVec3;
use spirv_std::arch::workgroup_memory_barrier_with_group_sync;
use spirv_std::spirv;

use crate::{ReductionOperation, ReductionParameters};

#[unsafe(no_mangle)]
#[spirv(compute(threads(256)))]
pub fn reduction_main(
    #[spirv(local_invocation_id)] invocation_id: UVec3,
    #[spirv(workgroup_id)] workgroup_id: UVec3,
    #[spirv(num_workgroups)] num_workgroups: UVec3,

    #[spirv(uniform, descriptor_set = 0, binding = 0)]
    params: &ReductionParameters,

    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)]
    input: &[f32],

    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)]
    output: &mut [f32],

    #[spirv(workgroup)]
    shared: &mut [f32; 256], 
) { 
    let invocation = invocation_id.x;
    let cluster = cluster_index(
        invocation,
        workgroup_id,
        num_workgroups,
        params.cluster_shift
    );

    let indices = tensor_indices(
        params.inner_size,
        params.reduction_size,
        params.cluster_shift,
        cluster
    );

    let cluster_size = 1u32 << params.cluster_shift;
    let lane_in_cluster = invocation_id.x & (cluster_size - 1);
    let reduction = (indices.y << params.cluster_shift) + lane_in_cluster;

    let outer_valid = indices.z < params.outer_size;
    let reduction_valid = reduction < params.reduction_size;

    shared[invocation as usize] = if outer_valid && reduction_valid {
        let index = (indices.z * params.reduction_size + reduction)
            * params.inner_size
            + indices.x;
        input[index as usize]
    } else {
        params.operation.identity() 
    };

    workgroup_memory_barrier_with_group_sync();
    let mut stride = cluster_size >> 1;

    while stride != 0 {
        if lane_in_cluster < stride {
            let lhs = shared[invocation as usize];
            let rhs = shared[(invocation + stride) as usize];
            let value = params.operation.reduce(lhs, rhs);
            shared[invocation as usize] = value; 
        }

        workgroup_memory_barrier_with_group_sync();

        stride >>= 1;
    }

    if lane_in_cluster == 0 && outer_valid {
        output[cluster as usize] = shared[invocation as usize];
    }
}

fn tensor_indices(
    inner_size: u32,
    reduction_size: u32,
    cluster_shift: u32,
    cluster_index: u32,
) -> UVec3 {
    let output_reduction_size =
        ((reduction_size.max(1u32) - 1u32) >> cluster_shift) + 1u32;
    let inner = cluster_index % inner_size;
    let remaining = cluster_index / inner_size;
    let chunk = remaining % output_reduction_size;
    let outer = remaining / output_reduction_size;
    UVec3::new(inner, chunk, outer)
}

fn cluster_index(
    invocation: u32,
    workgroup_id: UVec3,
    num_workgroups: UVec3,
    cluster_shift: u32
) -> u32 {
    let workgroup = workgroup_id.x +
        workgroup_id.y * num_workgroups.x +
        workgroup_id.z * num_workgroups.x * num_workgroups.y;
    let clusters_shift = 8u32 - cluster_shift;
    (workgroup << clusters_shift) + (invocation >> cluster_shift)
}

impl ReductionOperation {
    #[inline(always)]
    fn identity(self) -> f32 {
        match self {
            Self::SUM => 0.0,
            _ => 0.0
        }
    }

    #[inline(always)]
    fn reduce(self, lhs: f32, rhs: f32) -> f32 {
        match self {
            Self::SUM => lhs + rhs,
            _ => 0.0
        }
    }
}