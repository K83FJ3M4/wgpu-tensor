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
    var phi_115_: core_ops_Range_usize;
    var phi_118_: u32;
    var phi_116_: core_ops_Range_usize;
    var phi_141_: core_ops_Range_usize;
    var phi_184_: u32;
    var phi_119_: u32;
    var phi_570_: bool;
    var local_1: u32;
    var phi_234_: core_ops_Range_usize;
    var phi_237_: vec2<u32>;
    var phi_235_: core_ops_Range_usize;
    var phi_260_: core_ops_Range_usize;
    var phi_238_: vec2<u32>;
    var phi_576_: bool;
    var local_2: vec2<u32>;
    var local_3: vec2<u32>;
    var phi_455_: f32;
    var phi_456_: f32;
    var phi_457_: f32;
    var phi_458_: f32;
    var phi_459_: bool;
    var phi_467_: f32;
    var phi_468_: f32;
    var phi_469_: f32;
    var phi_470_: bool;
    var phi_474_: f32;
    var phi_475_: f32;
    var phi_476_: bool;
    var phi_550_: bool;
    var phi_420_: f32;
    var phi_421_: f32;
    var phi_565_: bool;
    var phi_409_: f32;
    var phi_410_: f32;
    var phi_380_: f32;
    var phi_382_: f32;
    var phi_383_: bool;
    var phi_384_: f32;
    var phi_385_: bool;
    var phi_386_: f32;
    var phi_387_: bool;
    var phi_392_: f32;
    var phi_356_: f32;
    var phi_361_: f32;
    var phi_362_: f32;
    var phi_393_: f32;
    var phi_394_: f32;
    var phi_481_: f32;

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
                phi_115_ = _e63;
                phi_118_ = _e52;
                loop {
                    let _e65 = phi_115_;
                    let _e67 = phi_118_;
                    local_1 = _e67;
                    if (_e65.start < _e65.end) {
                        phi_116_ = core_ops_Range_usize((_e65.start + 1u), _e65.end);
                        phi_141_ = core_ops_Range_usize(1u, _e65.start);
                    } else {
                        phi_116_ = _e65;
                        phi_141_ = core_ops_Range_usize(0u, core_ops_Range_usize().end);
                    }
                    let _e80 = phi_116_;
                    let _e82 = phi_141_;
                    let _e86 = (bitcast<i32>(_e82.start) != 0i);
                    if _e86 {
                        if (_e82.end < 7u) {
                        } else {
                            phi_570_ = true;
                            break;
                        }
                        let _e92 = params.member.divisions[_e82.end].divisor;
                        let _e94 = params.member.divisions[_e82.end].magic;
                        let _e96 = params.member.divisions[_e82.end].shift;
                        if (_e92 == 1u) {
                            phi_184_ = _e67;
                        } else {
                            let _e98 = (_e67 & 65535u);
                            let _e100 = (_e67 >> bitcast<u32>(16i));
                            let _e101 = (_e94 & 65535u);
                            let _e103 = (_e94 >> bitcast<u32>(16i));
                            let _e108 = ((_e100 * _e101) + ((_e98 * _e101) >> bitcast<u32>(16i)));
                            let _e118 = (((_e100 * _e103) + (_e108 >> bitcast<u32>(16i))) + (((_e98 * _e103) + (_e108 & 65535u)) >> bitcast<u32>(16i)));
                            phi_184_ = ((((_e67 - _e118) >> bitcast<u32>(1i)) + _e118) >> bitcast<u32>((_e96 & 31u)));
                        }
                        let _e127 = phi_184_;
                        if (_e82.end < 8u) {
                        } else {
                            phi_570_ = true;
                            break;
                        }
                        local[_e82.end] = (_e67 - (_e127 * _e92));
                        phi_119_ = _e127;
                    } else {
                        phi_119_ = u32();
                    }
                    let _e133 = phi_119_;
                    continue;
                    continuing {
                        phi_115_ = _e80;
                        phi_118_ = _e133;
                        phi_570_ = false;
                        break if !(_e86);
                    }
                }
                let _e136 = phi_570_;
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
                phi_234_ = _e63;
                phi_237_ = vec2<u32>((_e154 * ((_e144 >> bitcast<u32>(_e147)) & 1u)), (_e154 * ((_e146 >> bitcast<u32>(_e147)) & 1u)));
                loop {
                    let _e159 = phi_234_;
                    let _e161 = phi_237_;
                    local_2 = _e161;
                    local_3 = _e161;
                    if (_e159.start < _e159.end) {
                        let _e168 = (_e159.end - 1u);
                        phi_235_ = core_ops_Range_usize(_e159.start, _e168);
                        phi_260_ = core_ops_Range_usize(1u, _e168);
                    } else {
                        phi_235_ = _e159;
                        phi_260_ = core_ops_Range_usize(0u, core_ops_Range_usize().end);
                    }
                    let _e174 = phi_235_;
                    let _e176 = phi_260_;
                    let _e180 = (bitcast<i32>(_e176.start) != 0i);
                    if _e180 {
                        let _e181 = (_e176.end & 31u);
                        let _e186 = ((_e144 >> bitcast<u32>(_e181)) & 1u);
                        let _e187 = ((_e146 >> bitcast<u32>(_e181)) & 1u);
                        if (_e176.end < 7u) {
                        } else {
                            phi_576_ = true;
                            break;
                        }
                        let _e193 = params.member.divisions[_e176.end].divisor;
                        let _e194 = (_e193 - 1u);
                        if (_e176.end < 8u) {
                        } else {
                            phi_576_ = true;
                            break;
                        }
                        let _e201 = local[_e176.end];
                        phi_238_ = vec2<u32>(((_e161.x * (1u + (_e186 * _e194))) + (_e201 * _e186)), ((_e161.y * (1u + (_e187 * _e194))) + (_e201 * _e187)));
                    } else {
                        phi_238_ = vec2<u32>();
                    }
                    let _e212 = phi_238_;
                    continue;
                    continuing {
                        phi_234_ = _e174;
                        phi_237_ = _e212;
                        phi_576_ = _e136;
                        break if !(_e180);
                    }
                }
                let _e215 = phi_576_;
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
                        phi_481_ = (_e225 + _e229);
                        break;
                    }
                    case 1: {
                        phi_481_ = (_e225 - _e229);
                        break;
                    }
                    case 2: {
                        phi_481_ = (_e225 * _e229);
                        break;
                    }
                    case 3: {
                        phi_481_ = (_e225 / _e229);
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
                                    phi_356_ = _e332;
                                } else {
                                    phi_356_ = -(_e332);
                                }
                                let _e337 = phi_356_;
                                phi_361_ = _e337;
                                phi_362_ = f32();
                            } else {
                                phi_361_ = f32();
                                phi_362_ = bitcast<f32>(((_e298 & 4194303u) | 2143289344u));
                            }
                            let _e339 = phi_361_;
                            let _e341 = phi_362_;
                            phi_393_ = _e339;
                            phi_394_ = _e341;
                        } else {
                            if (_e225 == 0f) {
                                if ((_e298 & 2147483648u) != 0u) {
                                    if _e303 {
                                        let _e307 = ((_e229 % 2f) != 0f);
                                        if _e307 {
                                            phi_380_ = -(pow(0f, _e229));
                                        } else {
                                            phi_380_ = f32();
                                        }
                                        let _e311 = phi_380_;
                                        phi_382_ = _e311;
                                        phi_383_ = select(true, false, _e307);
                                    } else {
                                        phi_382_ = f32();
                                        phi_383_ = true;
                                    }
                                    let _e314 = phi_382_;
                                    let _e316 = phi_383_;
                                    phi_384_ = _e314;
                                    phi_385_ = _e316;
                                } else {
                                    phi_384_ = f32();
                                    phi_385_ = true;
                                }
                                let _e318 = phi_384_;
                                let _e320 = phi_385_;
                                phi_386_ = _e318;
                                phi_387_ = _e320;
                            } else {
                                phi_386_ = f32();
                                phi_387_ = true;
                            }
                            let _e322 = phi_386_;
                            let _e324 = phi_387_;
                            if _e324 {
                                phi_392_ = pow(_e225, _e229);
                            } else {
                                phi_392_ = _e322;
                            }
                            let _e327 = phi_392_;
                            phi_393_ = _e327;
                            phi_394_ = f32();
                        }
                        let _e343 = phi_393_;
                        let _e345 = phi_394_;
                        phi_481_ = select(_e345, _e343, select(true, _e303, _e304));
                        break;
                    }
                    case 5: {
                        let _e288 = (_e225 != _e225);
                        if _e288 {
                            phi_410_ = _e225;
                        } else {
                            if (_e229 != _e229) {
                                phi_409_ = _e229;
                            } else {
                                if _e288 {
                                    phi_565_ = true;
                                } else {
                                    phi_565_ = (_e229 <= _e225);
                                }
                                let _e292 = phi_565_;
                                phi_409_ = select(_e225, _e229, _e292);
                            }
                            let _e295 = phi_409_;
                            phi_410_ = _e295;
                        }
                        let _e297 = phi_410_;
                        phi_481_ = _e297;
                        break;
                    }
                    case 6: {
                        let _e278 = (_e225 != _e225);
                        if _e278 {
                            phi_421_ = _e225;
                        } else {
                            if (_e229 != _e229) {
                                phi_420_ = _e229;
                            } else {
                                if _e278 {
                                    phi_550_ = true;
                                } else {
                                    phi_550_ = (_e229 >= _e225);
                                }
                                let _e282 = phi_550_;
                                phi_420_ = select(_e225, _e229, _e282);
                            }
                            let _e285 = phi_420_;
                            phi_421_ = _e285;
                        }
                        let _e287 = phi_421_;
                        phi_481_ = _e287;
                        break;
                    }
                    case 7: {
                        if (_e225 != _e225) {
                            phi_474_ = _e225;
                            phi_475_ = f32();
                            phi_476_ = true;
                        } else {
                            if (_e229 != _e229) {
                                phi_468_ = _e229;
                                phi_469_ = f32();
                                phi_470_ = true;
                            } else {
                                if (_e229 == 0f) {
                                    phi_458_ = f32();
                                    phi_459_ = true;
                                } else {
                                    let _e239 = ((bitcast<u32>(_e225) & 2147483647u) == 2139095040u);
                                    if _e239 {
                                        phi_457_ = f32();
                                    } else {
                                        let _e240 = (_e225 % _e229);
                                        if (_e240 == 0f) {
                                            phi_456_ = bitcast<f32>((bitcast<u32>(_e229) & 2147483648u));
                                        } else {
                                            if ((_e240 < 0f) != (_e229 < 0f)) {
                                                phi_455_ = (_e240 + _e229);
                                            } else {
                                                phi_455_ = _e240;
                                            }
                                            let _e247 = phi_455_;
                                            phi_456_ = _e247;
                                        }
                                        let _e252 = phi_456_;
                                        phi_457_ = _e252;
                                    }
                                    let _e254 = phi_457_;
                                    phi_458_ = _e254;
                                    phi_459_ = _e239;
                                }
                                let _e256 = phi_458_;
                                let _e258 = phi_459_;
                                if _e258 {
                                    phi_467_ = bitcast<f32>(((bitcast<u32>(_e225) & 4194303u) | 2143289344u));
                                } else {
                                    phi_467_ = f32();
                                }
                                let _e264 = phi_467_;
                                phi_468_ = _e264;
                                phi_469_ = _e256;
                                phi_470_ = _e258;
                            }
                            let _e266 = phi_468_;
                            let _e268 = phi_469_;
                            let _e270 = phi_470_;
                            phi_474_ = _e266;
                            phi_475_ = _e268;
                            phi_476_ = _e270;
                        }
                        let _e272 = phi_474_;
                        let _e274 = phi_475_;
                        let _e276 = phi_476_;
                        phi_481_ = select(_e274, _e272, _e276);
                        break;
                    }
                    default: {
                        phi_481_ = 0f;
                        break;
                    }
                }
                let _e353 = phi_481_;
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
