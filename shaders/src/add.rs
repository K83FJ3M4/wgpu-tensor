use glam::{UVec3, UVec4};
use spirv_std::spirv;

#[repr(C)]
pub struct ShaderShape {
    lower: UVec4,
    upper: UVec4,
}

impl ShaderShape {
    fn dimensions(&self) -> [u32; 8] {
        [
            self.lower.x,
            self.lower.y,
            self.lower.z,
            self.lower.w,
            self.upper.x,
            self.upper.y,
            self.upper.z,
            self.upper.w,
        ]
    }
}

#[unsafe(no_mangle)]
#[spirv(compute(threads(256)))]
pub fn main(
    #[spirv(global_invocation_id)] index: UVec3,

    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] input_a_content: &[f32],
    #[spirv(uniform, descriptor_set = 0, binding = 1)] input_a_shape: &ShaderShape,

    #[spirv(storage_buffer, descriptor_set = 1, binding = 0)] input_b_content: &[f32],
    #[spirv(uniform, descriptor_set = 1, binding = 1)] input_b_shape: &ShaderShape,

    #[spirv(storage_buffer, descriptor_set = 2, binding = 0)] output_content: &mut [f32],
    #[spirv(uniform, descriptor_set = 2, binding = 1)] output_shape: &ShaderShape,
) {
    let input_a_shape = input_a_shape.dimensions();
    let input_b_shape = input_b_shape.dimensions();
    let output_shape = output_shape.dimensions();

    if index.x >= output_shape[0] || index.y >= output_shape[1] {
        return;
    }

    let mut output_index = index.x + index.y * output_shape[0];
    let mut input_a_index = if input_a_shape[0] == 1 { 0 } else { index.x }
        + if input_a_shape[1] == 1 {
            0
        } else {
            index.y * input_a_shape[0]
        };
    let mut input_b_index = if input_b_shape[0] == 1 { 0 } else { index.x }
        + if input_b_shape[1] == 1 {
            0
        } else {
            index.y * input_b_shape[0]
        };

    let mut output_stride = output_shape[0] * output_shape[1];
    let mut input_a_stride = input_a_shape[0] * input_a_shape[1];
    let mut input_b_stride = input_b_shape[0] * input_b_shape[1];
    let mut remaining_z = index.z;
    let mut dimension = 2;

    while dimension < 8 {
        let dimension_size = output_shape[dimension];
        if dimension_size == 0 {
            return;
        }

        let coordinate = remaining_z % dimension_size;
        remaining_z /= dimension_size;

        output_index += coordinate * output_stride;
        if input_a_shape[dimension] != 1 {
            input_a_index += coordinate * input_a_stride;
        }
        if input_b_shape[dimension] != 1 {
            input_b_index += coordinate * input_b_stride;
        }

        output_stride *= dimension_size;
        input_a_stride *= input_a_shape[dimension];
        input_b_stride *= input_b_shape[dimension];
        dimension += 1;
    }

    if remaining_z != 0 {
        return;
    }

    output_content[output_index as usize] =
        input_a_content[input_a_index as usize] + input_b_content[input_b_index as usize];
}
