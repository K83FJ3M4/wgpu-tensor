use core::ops::Sub;

use glam::{UVec2, UVec3};
use spirv_std::spirv;

use crate::{BinaryParameters, FastDivU32};

#[unsafe(no_mangle)]
#[spirv(compute(threads(256)))]
pub fn add(
    #[spirv(global_invocation_id)] invocation: UVec3,
    #[spirv(num_workgroups)] num_workgroups: UVec3,

    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &BinaryParameters,
    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] input_a: &[f32],
    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] input_b: &[f32],
    #[spirv(storage_buffer, descriptor_set = 3, binding = 0)] output: &mut [f32],
) {
    binary(
        core::ops::Add::add,
        invocation,
        num_workgroups,
        params,
        input_a,
        input_b,
        output
    )
}

fn binary(
    operation: impl FnOnce(f32, f32) -> f32,
    invocation: UVec3,
    num_workgroups: UVec3,

    params: &BinaryParameters,
    input_a: &[f32],
    input_b: &[f32],
    output: &mut [f32],
) {
    let mut index = BinaryParameters::invocation_index(invocation, num_workgroups);
    let output_index = index as usize;
    if index >= params.length { return }
    let max_dimension = params.num_dimensions.max(1).sub(1) as usize;
    let mut tensor_index = [0; 8];

    for i in 0..max_dimension {
        let division = params.divisions[i];
        let quotient = division.fast_div(index);
        tensor_index[i] = index - quotient * division.divisor;
        index = quotient;
    } 

    tensor_index[max_dimension] = index;
    let indices = params.flatten_index(&tensor_index);

    let lhs = input_a[indices.x as usize];
    let rhs = input_b[indices.y as usize];
    output[output_index] = operation(lhs, rhs);
}

impl BinaryParameters {
    #[inline(always)]
    fn flatten_index(&self, tensor_index: &[u32; 8]) -> UVec2 {
        let mask = UVec2::new(self.mask_a, self.mask_b);
        let max_index = self.num_dimensions.max(1) as usize - 1;
        let highest_valid = (mask >> (max_index as u32)) & 1;
        let mut flat = tensor_index[max_index] * highest_valid;

        for i in (0..max_index).rev() {
            let valid = (mask >> (i as u32)) & 1;
            let size = 1 + valid * (self.divisions[i].divisor - 1);
            let index = tensor_index[i] * valid;
            flat = flat * size + index;
        }

        flat
    } 

    fn invocation_index(id: UVec3, size: UVec3) -> u32 {
        let width = size.x * 256;
        id.x + width * (id.y + size.y * id.z)
    }
}

impl FastDivU32 {
    #[inline(always)]
    fn fast_div(self, n: u32) -> u32 {
        if self.divisor == 1 { return n; }
        let q = Self::mul_hi_u32(n, self.magic);
        let t = ((n - q) >> 1) + q;
        t >> self.shift
    }

    #[inline(always)]
    pub fn mul_hi_u32(x: u32, y: u32) -> u32 {
        let x0 = x & 0xffff;
        let x1 = x >> 16;

        let y0 = y & 0xffff;
        let y1 = y >> 16;

        let w0 = x0 * y0;

        let t = x1 * y0 + (w0 >> 16);
        let w1 = t & 0xffff;
        let w2 = t >> 16;

        let w1 = x0 * y1 + w1;

        x1 * y1 + w2 + (w1 >> 16) 
    }
}