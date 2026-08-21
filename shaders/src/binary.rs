use core::ops::Sub;

#[allow(unused)]
use spirv_std::num_traits::Float;
use glam::{UVec2, UVec3};
use spirv_std::spirv;

use crate::{BinaryOperation, BinaryParameters, FastDivU32};

#[unsafe(no_mangle)]
#[spirv(compute(threads(256)))]
pub fn binary_main(
    #[spirv(global_invocation_id)] invocation: UVec3,
    #[spirv(num_workgroups)] num_workgroups: UVec3,

    #[spirv(uniform, descriptor_set = 0, binding = 0)] params: &BinaryParameters,
    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] input_a: &[f32],
    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] input_b: &[f32],
    #[spirv(storage_buffer, descriptor_set = 3, binding = 0)] output: &mut [f32],
) {
    let mut index = invocation_index(invocation, num_workgroups);
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
    output[output_index] = match params.operation {
        BinaryOperation::ADD => lhs + rhs,
        BinaryOperation::SUBTRACT => lhs - rhs,
        BinaryOperation::MULTIPLY => lhs * rhs,
        BinaryOperation::DIVIDE => lhs / rhs,
        BinaryOperation::POWER => power(lhs, rhs),
        BinaryOperation::MINIMUM => minimum(lhs, rhs),
        BinaryOperation::MAXIMUM => maximum(lhs, rhs),
        BinaryOperation::REMAINDER => remainder(lhs, rhs),
        _ => 0.0
    };
}

impl BinaryParameters {
    #[inline(always)]
    fn flatten_index(&self, tensor_index: &[u32; 8]) -> UVec2 {
        let mask = unpack_2x_u16(self.masks);
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

#[inline(always)]
fn power(lhs: f32, rhs: f32) -> f32 {
    let negative = lhs.to_bits() & 0x8000_0000 != 0;
    let integral_exponent = rhs.fract() == 0.0;

    if lhs < 0.0 {
        if !integral_exponent {
            return nan_from(lhs);
        }

        let result = (-lhs).powf(rhs);
        if rhs % 2.0 == 0.0 { result } else { -result }
    } else if lhs == 0.0 && negative && integral_exponent && rhs % 2.0 != 0.0 {
        -(0.0_f32.powf(rhs))
    } else {
        lhs.powf(rhs)
    }
}

#[inline(always)]
fn minimum(lhs: f32, rhs: f32) -> f32 {
    if lhs.is_nan() {
        lhs
    } else if rhs.is_nan() {
        rhs
    } else {
        lhs.min(rhs)
    }
}

#[inline(always)]
fn maximum(lhs: f32, rhs: f32) -> f32 {
    if lhs.is_nan() {
        lhs
    } else if rhs.is_nan() {
        rhs
    } else {
        lhs.max(rhs)
    }
}

#[inline(always)]
fn remainder(lhs: f32, rhs: f32) -> f32 {
    if lhs.is_nan() {
        return lhs;
    } else if rhs.is_nan() {
        return rhs;
    } else if rhs == 0.0 || lhs.to_bits() & 0x7fff_ffff == 0x7f80_0000 {
        return nan_from(lhs);
    }

    let remainder = lhs % rhs;

    if remainder == 0.0 {
        f32::from_bits(rhs.to_bits() & 0x8000_0000)
    } else if (remainder < 0.0) != (rhs < 0.0) {
        remainder + rhs
    } else {
        remainder
    }
}

#[inline(always)]
fn nan_from(value: f32) -> f32 {
    f32::from_bits((value.to_bits() & 0x003f_ffff) | 0x7fc0_0000)
}

#[inline]
pub fn unpack_2x_u16(v: u32) -> UVec2 {
    UVec2::new(
        v & 0xFFFF,
        v >> 16,
    )
}

#[inline]
fn invocation_index(id: UVec3, size: UVec3) -> u32 {
    let width = size.x * 256;
    id.x + width * (id.y + size.y * id.z)
}