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
    var phi_82_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e11 = invocation_1;
            let _e12 = num_workgroups_1;
            let _e26 = (_e11.x + ((_e12.x * 256u) * (_e11.y + (_e12.y * _e11.z))));
            let _e29 = params.member.start;
            if (_e26 >= _e29) {
            } else {
                if (_e26 < arrayLength((&input.member))) {
                } else {
                    break;
                }
                let _e34 = input.member[_e26];
                let _e37 = params.member.end;
                switch bitcast<i32>(_e37) {
                    case 0: {
                        phi_82_ = -(_e34);
                        break;
                    }
                    case 1: {
                        phi_82_ = abs(_e34);
                        break;
                    }
                    case 2: {
                        phi_82_ = (1f / _e34);
                        break;
                    }
                    case 3: {
                        phi_82_ = sqrt(_e34);
                        break;
                    }
                    case 4: {
                        phi_82_ = inverseSqrt(_e34);
                        break;
                    }
                    case 5: {
                        phi_82_ = exp(_e34);
                        break;
                    }
                    case 6: {
                        phi_82_ = log(_e34);
                        break;
                    }
                    default: {
                        phi_82_ = 0f;
                        break;
                    }
                }
                let _e47 = phi_82_;
                if (_e26 < arrayLength((&output.member))) {
                } else {
                    break;
                }
                output.member[_e26] = _e47;
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
