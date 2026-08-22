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
    var phi_177_: f32;
    var phi_179_: f32;
    var phi_180_: bool;
    var phi_192_: f32;
    var phi_193_: bool;
    var phi_201_: f32;
    var phi_202_: f32;
    var phi_211_: u32;
    var phi_359_: bool;
    var phi_258_: f32;
    var phi_259_: f32;
    var phi_374_: bool;
    var phi_269_: f32;
    var phi_270_: f32;
    var phi_271_: f32;
    var phi_212_: u32;
    var phi_379_: bool;

    switch bitcast<i32>(0u) {
        default: {
            let _e26 = invocation_id_1;
            let _e27 = workgroup_id_1;
            let _e28 = num_workgroups_1;
            let _e36 = params.member.cluster_shift;
            let _e51 = (_e36 & 31u);
            let _e54 = ((((_e27.x + (_e27.y * _e28.x)) + ((_e27.z * _e28.x) * _e28.y)) << bitcast<u32>(((8u - _e36) & 31u))) + (_e26.x >> bitcast<u32>(_e51)));
            let _e57 = params.member.inner_size;
            let _e60 = params.member.reduction_size;
            let _e66 = (((select(1u, _e60, (1u < _e60)) - 1u) >> bitcast<u32>(_e51)) + 1u);
            let _e67 = (_e57 == 0u);
            if _e67 {
                break;
            }
            if _e67 {
                break;
            }
            let _e69 = (_e54 / _e57);
            let _e70 = (_e66 == 0u);
            if _e70 {
                break;
            }
            if _e70 {
                break;
            }
            let _e72 = (_e69 / _e66);
            let _e74 = (1u << bitcast<u32>(_e51));
            let _e76 = (_e26.x & (_e74 - 1u));
            let _e79 = (((_e69 % _e66) << bitcast<u32>(_e51)) + _e76);
            let _e82 = params.member.outer_size;
            let _e83 = (_e72 < _e82);
            let _e84 = (_e79 < _e60);
            if _e83 {
                if _e84 {
                    let _e88 = ((((_e72 * _e60) + _e79) * _e57) + (_e54 % _e57));
                    if (_e88 < arrayLength((&input.member))) {
                    } else {
                        break;
                    }
                    let _e92 = input.member[_e88];
                    phi_177_ = _e92;
                } else {
                    phi_177_ = f32();
                }
                let _e94 = phi_177_;
                phi_179_ = _e94;
                phi_180_ = select(true, false, _e84);
            } else {
                phi_179_ = f32();
                phi_180_ = true;
            }
            let _e97 = phi_179_;
            let _e99 = phi_180_;
            if _e99 {
                let _e102 = params.member.operation;
                switch bitcast<i32>(_e102) {
                    case 0: {
                        phi_192_ = 0f;
                        phi_193_ = false;
                        break;
                    }
                    case 1: {
                        phi_192_ = 1f;
                        phi_193_ = false;
                        break;
                    }
                    case 2: {
                        phi_192_ = f32();
                        phi_193_ = true;
                        break;
                    }
                    case 3: {
                        phi_192_ = f32();
                        phi_193_ = true;
                        break;
                    }
                    default: {
                        phi_192_ = 0f;
                        phi_193_ = false;
                        break;
                    }
                }
                let _e105 = phi_192_;
                let _e107 = phi_193_;
                if _e107 {
                    phi_201_ = bitcast<f32>((2139095040u | ((_e102 - 2u) << bitcast<u32>(31i))));
                } else {
                    phi_201_ = _e105;
                }
                let _e114 = phi_201_;
                phi_202_ = _e114;
            } else {
                phi_202_ = _e97;
            }
            let _e116 = phi_202_;
            let _e117 = (_e26.x < 256u);
            if _e117 {
            } else {
                break;
            }
            shared_[_e26.x] = _e116;
            workgroupBarrier();
            phi_211_ = (_e74 >> bitcast<u32>(1i));
            loop {
                let _e122 = phi_211_;
                let _e123 = (_e122 == 0u);
                if _e123 {
                    phi_212_ = u32();
                } else {
                    if (_e76 < _e122) {
                        if _e117 {
                        } else {
                            phi_379_ = true;
                            break;
                        }
                        let _e125 = shared_[_e26.x];
                        let _e126 = (_e26.x + _e122);
                        if (_e126 < 256u) {
                        } else {
                            phi_379_ = true;
                            break;
                        }
                        let _e129 = shared_[_e126];
                        let _e132 = params.member.operation;
                        switch bitcast<i32>(_e132) {
                            case 0: {
                                phi_271_ = (_e125 + _e129);
                                break;
                            }
                            case 1: {
                                phi_271_ = (_e125 * _e129);
                                break;
                            }
                            case 2: {
                                let _e136 = (_e125 != _e125);
                                if _e136 {
                                    phi_259_ = _e125;
                                } else {
                                    if (_e129 != _e129) {
                                        phi_258_ = _e129;
                                    } else {
                                        if _e136 {
                                            phi_359_ = true;
                                        } else {
                                            phi_359_ = (_e129 <= _e125);
                                        }
                                        let _e140 = phi_359_;
                                        phi_258_ = select(_e125, _e129, _e140);
                                    }
                                    let _e143 = phi_258_;
                                    phi_259_ = _e143;
                                }
                                let _e145 = phi_259_;
                                phi_271_ = _e145;
                                break;
                            }
                            case 3: {
                                let _e146 = (_e125 != _e125);
                                if _e146 {
                                    phi_270_ = _e125;
                                } else {
                                    if (_e129 != _e129) {
                                        phi_269_ = _e129;
                                    } else {
                                        if _e146 {
                                            phi_374_ = true;
                                        } else {
                                            phi_374_ = (_e129 >= _e125);
                                        }
                                        let _e150 = phi_374_;
                                        phi_269_ = select(_e125, _e129, _e150);
                                    }
                                    let _e153 = phi_269_;
                                    phi_270_ = _e153;
                                }
                                let _e155 = phi_270_;
                                phi_271_ = _e155;
                                break;
                            }
                            default: {
                                phi_271_ = 0f;
                                break;
                            }
                        }
                        let _e157 = phi_271_;
                        if _e117 {
                        } else {
                            phi_379_ = true;
                            break;
                        }
                        shared_[_e26.x] = _e157;
                    }
                    workgroupBarrier();
                    phi_212_ = (_e122 >> bitcast<u32>(1i));
                }
                let _e161 = phi_212_;
                continue;
                continuing {
                    phi_211_ = _e161;
                    phi_379_ = false;
                    break if !(select(true, false, _e123));
                }
            }
            let _e165 = phi_379_;
            if _e165 {
                break;
            }
            if (_e76 == 0u) {
                if _e83 {
                    if _e117 {
                    } else {
                        break;
                    }
                    let _e167 = shared_[_e26.x];
                    if (_e54 < arrayLength((&output.member))) {
                    } else {
                        break;
                    }
                    output.member[_e54] = _e167;
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
