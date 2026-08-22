struct type_5 {
    member: array<f32>,
}

struct ReductionParameters {
    operation: u32,
    cluster_shift: u32,
    inner_size: u32,
    reduction_size: u32,
    outer_size: u32,
}

struct type_11 {
    member: ReductionParameters,
}

var<private> num_workgroups_1: vec3<u32>;
var<private> invocation_id_1: vec3<u32>;
var<private> workgroup_id_1: vec3<u32>;
@group(0) @binding(0) 
var<uniform> params: type_11;
@group(1) @binding(0) 
var<storage> input: type_5;
@group(2) @binding(0) 
var<storage, read_write> output: type_5;
var<workgroup> shared_: array<f32, 256>;

fn function_() {
    var phi_131_: f32;
    var phi_133_: f32;
    var phi_134_: bool;
    var phi_149_: u32;
    var phi_184_: f32;
    var phi_150_: u32;
    var phi_261_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e23 = invocation_id_1;
            let _e24 = workgroup_id_1;
            let _e25 = num_workgroups_1;
            let _e33 = params.member.cluster_shift;
            let _e48 = (_e33 & 31u);
            let _e51 = ((((_e24.x + (_e24.y * _e25.x)) + ((_e24.z * _e25.x) * _e25.y)) << bitcast<u32>(((8u - _e33) & 31u))) + (_e23.x >> bitcast<u32>(_e48)));
            let _e54 = params.member.inner_size;
            let _e57 = params.member.reduction_size;
            let _e61 = (((_e57 - 1u) >> bitcast<u32>(_e48)) + 1u);
            let _e62 = (_e54 == 0u);
            if _e62 {
                break;
            }
            if _e62 {
                break;
            }
            let _e64 = (_e51 / _e54);
            let _e65 = (_e61 == 0u);
            if _e65 {
                break;
            }
            if _e65 {
                break;
            }
            let _e67 = (_e64 / _e61);
            let _e69 = (1u << bitcast<u32>(_e48));
            let _e71 = (_e23.x & (_e69 - 1u));
            let _e74 = (((_e64 % _e61) << bitcast<u32>(_e48)) + _e71);
            let _e77 = params.member.outer_size;
            let _e78 = (_e67 < _e77);
            let _e79 = (_e74 < _e57);
            if _e78 {
                if _e79 {
                    let _e83 = ((((_e67 * _e57) + _e74) * _e54) + (_e51 % _e54));
                    if (_e83 < arrayLength((&input.member))) {
                    } else {
                        break;
                    }
                    let _e87 = input.member[_e83];
                    phi_131_ = _e87;
                } else {
                    phi_131_ = f32();
                }
                let _e89 = phi_131_;
                phi_133_ = _e89;
                phi_134_ = select(true, false, _e79);
            } else {
                phi_133_ = f32();
                phi_134_ = true;
            }
            let _e92 = phi_133_;
            let _e94 = phi_134_;
            let _e96 = (_e23.x < 256u);
            if _e96 {
            } else {
                break;
            }
            shared_[_e23.x] = select(_e92, 0f, _e94);
            workgroupBarrier();
            phi_149_ = (_e69 >> bitcast<u32>(1i));
            loop {
                let _e101 = phi_149_;
                let _e102 = (_e101 == 0u);
                if _e102 {
                    phi_150_ = u32();
                } else {
                    if (_e71 < _e101) {
                        if _e96 {
                        } else {
                            phi_261_ = true;
                            break;
                        }
                        let _e104 = shared_[_e23.x];
                        let _e105 = (_e23.x + _e101);
                        if (_e105 < 256u) {
                        } else {
                            phi_261_ = true;
                            break;
                        }
                        let _e108 = shared_[_e105];
                        let _e111 = params.member.operation;
                        if (_e111 == 0u) {
                            phi_184_ = (_e104 + _e108);
                        } else {
                            phi_184_ = 0f;
                        }
                        let _e115 = phi_184_;
                        if _e96 {
                        } else {
                            phi_261_ = true;
                            break;
                        }
                        shared_[_e23.x] = _e115;
                    }
                    workgroupBarrier();
                    phi_150_ = (_e101 >> bitcast<u32>(1i));
                }
                let _e119 = phi_150_;
                continue;
                continuing {
                    phi_149_ = _e119;
                    phi_261_ = false;
                    break if !(select(true, false, _e102));
                }
            }
            let _e123 = phi_261_;
            if _e123 {
                break;
            }
            if (_e71 == 0u) {
                if _e78 {
                    if _e96 {
                    } else {
                        break;
                    }
                    let _e125 = shared_[_e23.x];
                    if (_e51 < arrayLength((&output.member))) {
                    } else {
                        break;
                    }
                    output.member[_e51] = _e125;
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(256, 1, 1) 
fn main(@builtin(local_invocation_id) invocation_id: vec3<u32>, @builtin(workgroup_id) workgroup_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    invocation_id_1 = invocation_id;
    workgroup_id_1 = workgroup_id;
    num_workgroups_1 = num_workgroups;
    function_();
}
