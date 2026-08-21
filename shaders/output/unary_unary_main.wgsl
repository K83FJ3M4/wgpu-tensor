struct type_5 {
    member: array<f32>,
}

struct core_ops_Range_usize {
    start: u32,
    end: u32,
}

struct type_10 {
    member: core_ops_Range_usize,
}

var<private> invocation_1: vec3<u32>;
var<private> num_workgroups_1: vec3<u32>;
@group(0) @binding(0) 
var<uniform> params: type_10;
@group(1) @binding(0) 
var<storage> input: type_5;
@group(2) @binding(0) 
var<storage, read_write> output: type_5;

fn function_() {
    var phi_67_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e10 = invocation_1;
            let _e11 = num_workgroups_1;
            let _e25 = (_e10.x + ((_e11.x * 256u) * (_e10.y + (_e11.y * _e10.z))));
            let _e28 = params.member.start;
            if (_e25 >= _e28) {
            } else {
                let _e32 = params.member.end;
                if (_e32 == 0u) {
                    if (_e25 < arrayLength((&input.member))) {
                    } else {
                        break;
                    }
                    let _e37 = input.member[_e25];
                    phi_67_ = -(_e37);
                } else {
                    phi_67_ = 0f;
                }
                let _e40 = phi_67_;
                if (_e25 < arrayLength((&output.member))) {
                } else {
                    break;
                }
                output.member[_e25] = _e40;
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(256, 1, 1) 
fn main(@builtin(global_invocation_id) invocation: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    invocation_1 = invocation;
    num_workgroups_1 = num_workgroups;
    function_();
}
