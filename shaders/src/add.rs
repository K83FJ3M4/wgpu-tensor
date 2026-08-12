use glam::UVec3;
use spirv_std::spirv;

use crate::BinaryParameters;

#[unsafe(no_mangle)]
#[spirv(compute(threads(256)))]
pub fn main(
    #[spirv(global_invocation_id)] invocation: UVec3,

    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: BinaryParameters,
    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] input_a: &[f32],
    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] input_b: &[f32],
    #[spirv(storage_buffer, descriptor_set = 3, binding = 0)] content: &mut [f32],
) {

}
