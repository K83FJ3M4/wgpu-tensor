struct FastDivU32_ {
    divisor: u32,
    magic: u32,
    shift: u32,
    pad: u32,
}

struct BinaryParameters {
    divisions: array<FastDivU32_, 7>,
    num_dimensions: u32,
    length: u32,
    masks: u32,
    operation: u32,
}

struct type_4 {
    member: BinaryParameters,
}

struct type_8 {
    member: array<f32>,
}

struct core_ops_Range_usize {
    start: u32,
    end: u32,
}

var<private> invocation_1: vec3<u32>;
var<private> num_workgroups_1: vec3<u32>;
@group(0) @binding(0) 
var<uniform> params: type_4;
@group(1) @binding(0) 
var<storage> input_a: type_8;
@group(2) @binding(0) 
var<storage> input_b: type_8;
@group(3) @binding(0) 
var<storage, read_write> output: type_8;

fn function_() {
    var local: array<u32, 8>;
    var phi_108_: core_ops_Range_usize;
    var phi_111_: u32;
    var phi_109_: core_ops_Range_usize;
    var phi_134_: core_ops_Range_usize;
    var phi_177_: u32;
    var phi_112_: u32;
    var phi_588_: bool;
    var local_1: u32;
    var phi_227_: core_ops_Range_usize;
    var phi_230_: vec2<u32>;
    var phi_228_: core_ops_Range_usize;
    var phi_253_: core_ops_Range_usize;
    var phi_231_: vec2<u32>;
    var phi_594_: bool;
    var local_2: vec2<u32>;
    var local_3: vec2<u32>;
    var phi_448_: f32;
    var phi_449_: f32;
    var phi_450_: f32;
    var phi_451_: f32;
    var phi_452_: bool;
    var phi_460_: f32;
    var phi_461_: f32;
    var phi_462_: f32;
    var phi_463_: bool;
    var phi_467_: f32;
    var phi_468_: f32;
    var phi_469_: bool;
    var phi_568_: bool;
    var phi_413_: f32;
    var phi_414_: f32;
    var phi_583_: bool;
    var phi_402_: f32;
    var phi_403_: f32;
    var phi_373_: f32;
    var phi_375_: f32;
    var phi_376_: bool;
    var phi_377_: f32;
    var phi_378_: bool;
    var phi_379_: f32;
    var phi_380_: bool;
    var phi_385_: f32;
    var phi_349_: f32;
    var phi_354_: f32;
    var phi_355_: f32;
    var phi_386_: f32;
    var phi_387_: f32;
    var phi_474_: f32;

    switch bitcast<i32>(0u) {
        default: {
            let _e35 = invocation_1;
            let _e36 = num_workgroups_1;
            let _e52 = (_e35.x + ((_e36.x * 256u) * (_e35.y + (_e36.y * _e35.z))));
            let _e55 = params.member.length;
            if (_e52 >= _e55) {
            } else {
                let _e59 = params.member.num_dimensions;
                let _e62 = (select(1u, _e59, (1u < _e59)) - 1u);
                local = array<u32, 8>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                let _e63 = core_ops_Range_usize(0u, _e62);
                phi_108_ = _e63;
                phi_111_ = _e52;
                loop {
                    let _e65 = phi_108_;
                    let _e67 = phi_111_;
                    local_1 = _e67;
                    if (_e65.start < _e65.end) {
                        phi_109_ = core_ops_Range_usize((_e65.start + 1u), _e65.end);
                        phi_134_ = core_ops_Range_usize(1u, _e65.start);
                    } else {
                        phi_109_ = _e65;
                        phi_134_ = core_ops_Range_usize(0u, core_ops_Range_usize().end);
                    }
                    let _e80 = phi_109_;
                    let _e82 = phi_134_;
                    let _e86 = (bitcast<i32>(_e82.start) != 0i);
                    if _e86 {
                        if (_e82.end < 7u) {
                        } else {
                            phi_588_ = true;
                            break;
                        }
                        let _e92 = params.member.divisions[_e82.end].divisor;
                        let _e94 = params.member.divisions[_e82.end].magic;
                        let _e96 = params.member.divisions[_e82.end].shift;
                        if (_e92 == 1u) {
                            phi_177_ = _e67;
                        } else {
                            let _e98 = (_e67 & 65535u);
                            let _e100 = (_e67 >> bitcast<u32>(16i));
                            let _e101 = (_e94 & 65535u);
                            let _e103 = (_e94 >> bitcast<u32>(16i));
                            let _e108 = ((_e100 * _e101) + ((_e98 * _e101) >> bitcast<u32>(16i)));
                            let _e118 = (((_e100 * _e103) + (_e108 >> bitcast<u32>(16i))) + (((_e98 * _e103) + (_e108 & 65535u)) >> bitcast<u32>(16i)));
                            phi_177_ = ((((_e67 - _e118) >> bitcast<u32>(1i)) + _e118) >> bitcast<u32>((_e96 & 31u)));
                        }
                        let _e127 = phi_177_;
                        if (_e82.end < 8u) {
                        } else {
                            phi_588_ = true;
                            break;
                        }
                        local[_e82.end] = (_e67 - (_e127 * _e92));
                        phi_112_ = _e127;
                    } else {
                        phi_112_ = u32();
                    }
                    let _e133 = phi_112_;
                    continue;
                    continuing {
                        phi_108_ = _e80;
                        phi_111_ = _e133;
                        phi_588_ = false;
                        break if !(_e86);
                    }
                }
                let _e136 = phi_588_;
                if _e136 {
                    break;
                }
                let _e137 = (_e62 < 8u);
                if _e137 {
                } else {
                    break;
                }
                let _e140 = local_1;
                local[_e62] = _e140;
                let _e143 = params.member.masks;
                let _e144 = (_e143 & 65535u);
                let _e146 = (_e143 >> bitcast<u32>(16i));
                let _e147 = (_e62 & 31u);
                if _e137 {
                } else {
                    break;
                }
                let _e154 = local[_e62];
                phi_227_ = _e63;
                phi_230_ = vec2<u32>((_e154 * ((_e144 >> bitcast<u32>(_e147)) & 1u)), (_e154 * ((_e146 >> bitcast<u32>(_e147)) & 1u)));
                loop {
                    let _e159 = phi_227_;
                    let _e161 = phi_230_;
                    local_2 = _e161;
                    local_3 = _e161;
                    if (_e159.start < _e159.end) {
                        let _e168 = (_e159.end - 1u);
                        phi_228_ = core_ops_Range_usize(_e159.start, _e168);
                        phi_253_ = core_ops_Range_usize(1u, _e168);
                    } else {
                        phi_228_ = _e159;
                        phi_253_ = core_ops_Range_usize(0u, core_ops_Range_usize().end);
                    }
                    let _e174 = phi_228_;
                    let _e176 = phi_253_;
                    let _e180 = (bitcast<i32>(_e176.start) != 0i);
                    if _e180 {
                        let _e181 = (_e176.end & 31u);
                        let _e186 = ((_e144 >> bitcast<u32>(_e181)) & 1u);
                        let _e187 = ((_e146 >> bitcast<u32>(_e181)) & 1u);
                        if (_e176.end < 7u) {
                        } else {
                            phi_594_ = true;
                            break;
                        }
                        let _e193 = params.member.divisions[_e176.end].divisor;
                        let _e194 = (_e193 - 1u);
                        if (_e176.end < 8u) {
                        } else {
                            phi_594_ = true;
                            break;
                        }
                        let _e201 = local[_e176.end];
                        phi_231_ = vec2<u32>(((_e161.x * (1u + (_e186 * _e194))) + (_e201 * _e186)), ((_e161.y * (1u + (_e187 * _e194))) + (_e201 * _e187)));
                    } else {
                        phi_231_ = vec2<u32>();
                    }
                    let _e212 = phi_231_;
                    continue;
                    continuing {
                        phi_227_ = _e174;
                        phi_230_ = _e212;
                        phi_594_ = _e136;
                        break if !(_e180);
                    }
                }
                let _e215 = phi_594_;
                if _e215 {
                    break;
                }
                let _e217 = local_2;
                let _e220 = local_3;
                if (_e217.x < arrayLength((&input_a.member))) {
                } else {
                    break;
                }
                let _e225 = input_a.member[_e217.x];
                if (_e220.y < arrayLength((&input_b.member))) {
                } else {
                    break;
                }
                let _e229 = input_b.member[_e220.y];
                let _e232 = params.member.operation;
                switch bitcast<i32>(_e232) {
                    case 0: {
                        phi_474_ = (_e225 + _e229);
                        break;
                    }
                    case 1: {
                        phi_474_ = (_e225 - _e229);
                        break;
                    }
                    case 2: {
                        phi_474_ = (_e225 * _e229);
                        break;
                    }
                    case 3: {
                        phi_474_ = (_e225 / _e229);
                        break;
                    }
                    case 4: {
                        let _e298 = bitcast<u32>(_e225);
                        let _e303 = ((_e229 - trunc(_e229)) == 0f);
                        let _e304 = (_e225 < 0f);
                        if _e304 {
                            if _e303 {
                                let _e332 = pow(-(_e225), _e229);
                                if ((_e229 % 2f) == 0f) {
                                    phi_349_ = _e332;
                                } else {
                                    phi_349_ = -(_e332);
                                }
                                let _e337 = phi_349_;
                                phi_354_ = _e337;
                                phi_355_ = f32();
                            } else {
                                phi_354_ = f32();
                                phi_355_ = bitcast<f32>(((_e298 & 4194303u) | 2143289344u));
                            }
                            let _e339 = phi_354_;
                            let _e341 = phi_355_;
                            phi_386_ = _e339;
                            phi_387_ = _e341;
                        } else {
                            if (_e225 == 0f) {
                                if ((_e298 & 2147483648u) != 0u) {
                                    if _e303 {
                                        let _e307 = ((_e229 % 2f) != 0f);
                                        if _e307 {
                                            phi_373_ = -(pow(0f, _e229));
                                        } else {
                                            phi_373_ = f32();
                                        }
                                        let _e311 = phi_373_;
                                        phi_375_ = _e311;
                                        phi_376_ = select(true, false, _e307);
                                    } else {
                                        phi_375_ = f32();
                                        phi_376_ = true;
                                    }
                                    let _e314 = phi_375_;
                                    let _e316 = phi_376_;
                                    phi_377_ = _e314;
                                    phi_378_ = _e316;
                                } else {
                                    phi_377_ = f32();
                                    phi_378_ = true;
                                }
                                let _e318 = phi_377_;
                                let _e320 = phi_378_;
                                phi_379_ = _e318;
                                phi_380_ = _e320;
                            } else {
                                phi_379_ = f32();
                                phi_380_ = true;
                            }
                            let _e322 = phi_379_;
                            let _e324 = phi_380_;
                            if _e324 {
                                phi_385_ = pow(_e225, _e229);
                            } else {
                                phi_385_ = _e322;
                            }
                            let _e327 = phi_385_;
                            phi_386_ = _e327;
                            phi_387_ = f32();
                        }
                        let _e343 = phi_386_;
                        let _e345 = phi_387_;
                        phi_474_ = select(_e345, _e343, select(true, _e303, _e304));
                        break;
                    }
                    case 5: {
                        let _e288 = (_e225 != _e225);
                        if _e288 {
                            phi_403_ = _e225;
                        } else {
                            if (_e229 != _e229) {
                                phi_402_ = _e229;
                            } else {
                                if _e288 {
                                    phi_583_ = true;
                                } else {
                                    phi_583_ = (_e229 <= _e225);
                                }
                                let _e292 = phi_583_;
                                phi_402_ = select(_e225, _e229, _e292);
                            }
                            let _e295 = phi_402_;
                            phi_403_ = _e295;
                        }
                        let _e297 = phi_403_;
                        phi_474_ = _e297;
                        break;
                    }
                    case 6: {
                        let _e278 = (_e225 != _e225);
                        if _e278 {
                            phi_414_ = _e225;
                        } else {
                            if (_e229 != _e229) {
                                phi_413_ = _e229;
                            } else {
                                if _e278 {
                                    phi_568_ = true;
                                } else {
                                    phi_568_ = (_e229 >= _e225);
                                }
                                let _e282 = phi_568_;
                                phi_413_ = select(_e225, _e229, _e282);
                            }
                            let _e285 = phi_413_;
                            phi_414_ = _e285;
                        }
                        let _e287 = phi_414_;
                        phi_474_ = _e287;
                        break;
                    }
                    case 7: {
                        if (_e225 != _e225) {
                            phi_467_ = _e225;
                            phi_468_ = f32();
                            phi_469_ = true;
                        } else {
                            if (_e229 != _e229) {
                                phi_461_ = _e229;
                                phi_462_ = f32();
                                phi_463_ = true;
                            } else {
                                if (_e229 == 0f) {
                                    phi_451_ = f32();
                                    phi_452_ = true;
                                } else {
                                    let _e239 = ((bitcast<u32>(_e225) & 2147483647u) == 2139095040u);
                                    if _e239 {
                                        phi_450_ = f32();
                                    } else {
                                        let _e240 = (_e225 % _e229);
                                        if (_e240 == 0f) {
                                            phi_449_ = bitcast<f32>((bitcast<u32>(_e229) & 2147483648u));
                                        } else {
                                            if ((_e240 < 0f) != (_e229 < 0f)) {
                                                phi_448_ = (_e240 + _e229);
                                            } else {
                                                phi_448_ = _e240;
                                            }
                                            let _e247 = phi_448_;
                                            phi_449_ = _e247;
                                        }
                                        let _e252 = phi_449_;
                                        phi_450_ = _e252;
                                    }
                                    let _e254 = phi_450_;
                                    phi_451_ = _e254;
                                    phi_452_ = _e239;
                                }
                                let _e256 = phi_451_;
                                let _e258 = phi_452_;
                                if _e258 {
                                    phi_460_ = bitcast<f32>(((bitcast<u32>(_e225) & 4194303u) | 2143289344u));
                                } else {
                                    phi_460_ = f32();
                                }
                                let _e264 = phi_460_;
                                phi_461_ = _e264;
                                phi_462_ = _e256;
                                phi_463_ = _e258;
                            }
                            let _e266 = phi_461_;
                            let _e268 = phi_462_;
                            let _e270 = phi_463_;
                            phi_467_ = _e266;
                            phi_468_ = _e268;
                            phi_469_ = _e270;
                        }
                        let _e272 = phi_467_;
                        let _e274 = phi_468_;
                        let _e276 = phi_469_;
                        phi_474_ = select(_e274, _e272, _e276);
                        break;
                    }
                    default: {
                        phi_474_ = 0f;
                        break;
                    }
                }
                let _e353 = phi_474_;
                if (_e52 < arrayLength((&output.member))) {
                } else {
                    break;
                }
                output.member[_e52] = _e353;
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
