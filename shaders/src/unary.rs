use glam::UVec3;
use spirv_std::spirv;

use crate::{UnaryOperation, UnaryParameters};

#[unsafe(no_mangle)]
#[spirv(compute(threads(256)))]
pub fn unary_main(
    #[spirv(global_invocation_id)] invocation: UVec3,
    #[spirv(num_workgroups)] num_workgroups: UVec3,

    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &UnaryParameters,
    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] input: &[f32],
    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] output: &mut [f32],
) {
    let index = invocation_index(invocation, num_workgroups);
    if index >= params.length { return }

    output[index as usize] = match params.operation {
        UnaryOperation::NEGATE => -input[index as usize],
        _ => 0.0
    }
}

#[inline]
fn invocation_index(id: UVec3, size: UVec3) -> u32 {
    let width = size.x * 256;
    id.x + width * (id.y + size.y * id.z)
}