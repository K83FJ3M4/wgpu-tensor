use core::arch::asm;

use glam::UVec3;
use spirv_std::spirv;

#[allow(unused)]
use spirv_std::num_traits::Float;

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

    let input = input[index as usize];
    output[index as usize] = match params.operation {
        UnaryOperation::NEGATE => -input,
        UnaryOperation::ABSOLUTE => input.abs(),
        UnaryOperation::RECIPROCAL => input.recip(),
        UnaryOperation::SQUARE_ROOT => input.sqrt(),
        UnaryOperation::RECIPROCAL_SQUARE_ROOT => inverse_sqrt(input),
        UnaryOperation::EXPONENTIAL => input.exp(),
        UnaryOperation::LOGARITHM => input.ln(),
        UnaryOperation::COPY => input,
        _ => 0.0
    }
}

#[allow(asm_sub_register)]
#[inline(always)]
fn inverse_sqrt(value: f32) -> f32 {
    let result;
    unsafe {
        asm!(
            "%glsl = OpExtInstImport \"GLSL.std.450\"",
            "{result} = OpExtInst typeof*{result} %glsl 32 {value}",
            value = in(reg) value,
            result = out(reg) result,
        );
    }
    result
}

#[inline]
fn invocation_index(id: UVec3, size: UVec3) -> u32 {
    let width = size.x * 256;
    id.x + width * (id.y + size.y * id.z)
}
