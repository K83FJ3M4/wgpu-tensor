struct type_3 {
    member: array<f32>,
}

@group(0) @binding(0) 
var<storage> input_a_content: type_3;
@group(2) @binding(0) 
var<storage, read_write> output_content: type_3;

fn function_() {
    switch bitcast<i32>(0u) {
        default: {
            if (0u < arrayLength((&input_a_content.member))) {
            } else {
                break;
            }
            let _e16 = input_a_content.member[0u];
            if (0u < arrayLength((&output_content.member))) {
            } else {
                break;
            }
            output_content.member[0u] = f32((select(select(u64(_e16), 0lu, (_e16 < 0f)), 18446744073709551615lu, (_e16 > 18446743000000000000f)) + 23lu));
            break;
        }
    }
    return;
}

@compute @workgroup_size(256, 1, 1) 
fn main() {
    function_();
}
