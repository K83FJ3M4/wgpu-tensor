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
    var phi_92_: core_ops_Range_usize;
    var phi_95_: u32;
    var phi_93_: core_ops_Range_usize;
    var phi_118_: core_ops_Range_usize;
    var phi_161_: u32;
    var phi_96_: u32;
    var phi_354_: bool;
    var local_1: u32;
    var phi_211_: core_ops_Range_usize;
    var phi_214_: vec2<u32>;
    var phi_212_: core_ops_Range_usize;
    var phi_237_: core_ops_Range_usize;
    var phi_215_: vec2<u32>;
    var phi_360_: bool;
    var local_2: vec2<u32>;
    var local_3: vec2<u32>;

    switch bitcast<i32>(0u) {
        default: {
            let _e27 = invocation_1;
            let _e28 = num_workgroups_1;
            let _e44 = (_e27.x + ((_e28.x * 256u) * (_e27.y + (_e28.y * _e27.z))));
            let _e47 = params.member.length;
            if (_e44 >= _e47) {
            } else {
                let _e51 = params.member.num_dimensions;
                let _e54 = (select(1u, _e51, (1u < _e51)) - 1u);
                local = array<u32, 8>(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u);
                let _e55 = core_ops_Range_usize(0u, _e54);
                phi_92_ = _e55;
                phi_95_ = _e44;
                loop {
                    let _e57 = phi_92_;
                    let _e59 = phi_95_;
                    local_1 = _e59;
                    if (_e57.start < _e57.end) {
                        phi_93_ = core_ops_Range_usize((_e57.start + 1u), _e57.end);
                        phi_118_ = core_ops_Range_usize(1u, _e57.start);
                    } else {
                        phi_93_ = _e57;
                        phi_118_ = core_ops_Range_usize(0u, core_ops_Range_usize().end);
                    }
                    let _e72 = phi_93_;
                    let _e74 = phi_118_;
                    let _e78 = (bitcast<i32>(_e74.start) != 0i);
                    if _e78 {
                        if (_e74.end < 7u) {
                        } else {
                            phi_354_ = true;
                            break;
                        }
                        let _e84 = params.member.divisions[_e74.end].divisor;
                        let _e86 = params.member.divisions[_e74.end].magic;
                        let _e88 = params.member.divisions[_e74.end].shift;
                        if (_e84 == 1u) {
                            phi_161_ = _e59;
                        } else {
                            let _e90 = (_e59 & 65535u);
                            let _e92 = (_e59 >> bitcast<u32>(16i));
                            let _e93 = (_e86 & 65535u);
                            let _e95 = (_e86 >> bitcast<u32>(16i));
                            let _e100 = ((_e92 * _e93) + ((_e90 * _e93) >> bitcast<u32>(16i)));
                            let _e110 = (((_e92 * _e95) + (_e100 >> bitcast<u32>(16i))) + (((_e90 * _e95) + (_e100 & 65535u)) >> bitcast<u32>(16i)));
                            phi_161_ = ((((_e59 - _e110) >> bitcast<u32>(1i)) + _e110) >> bitcast<u32>((_e88 & 31u)));
                        }
                        let _e119 = phi_161_;
                        if (_e74.end < 8u) {
                        } else {
                            phi_354_ = true;
                            break;
                        }
                        local[_e74.end] = (_e59 - (_e119 * _e84));
                        phi_96_ = _e119;
                    } else {
                        phi_96_ = u32();
                    }
                    let _e125 = phi_96_;
                    continue;
                    continuing {
                        phi_92_ = _e72;
                        phi_95_ = _e125;
                        phi_354_ = false;
                        break if !(_e78);
                    }
                }
                let _e128 = phi_354_;
                if _e128 {
                    break;
                }
                let _e129 = (_e54 < 8u);
                if _e129 {
                } else {
                    break;
                }
                let _e132 = local_1;
                local[_e54] = _e132;
                let _e135 = params.member.masks;
                let _e136 = (_e135 & 65535u);
                let _e138 = (_e135 >> bitcast<u32>(16i));
                let _e139 = (_e54 & 31u);
                if _e129 {
                } else {
                    break;
                }
                let _e146 = local[_e54];
                phi_211_ = _e55;
                phi_214_ = vec2<u32>((_e146 * ((_e136 >> bitcast<u32>(_e139)) & 1u)), (_e146 * ((_e138 >> bitcast<u32>(_e139)) & 1u)));
                loop {
                    let _e151 = phi_211_;
                    let _e153 = phi_214_;
                    local_2 = _e153;
                    local_3 = _e153;
                    if (_e151.start < _e151.end) {
                        let _e160 = (_e151.end - 1u);
                        phi_212_ = core_ops_Range_usize(_e151.start, _e160);
                        phi_237_ = core_ops_Range_usize(1u, _e160);
                    } else {
                        phi_212_ = _e151;
                        phi_237_ = core_ops_Range_usize(0u, core_ops_Range_usize().end);
                    }
                    let _e166 = phi_212_;
                    let _e168 = phi_237_;
                    let _e172 = (bitcast<i32>(_e168.start) != 0i);
                    if _e172 {
                        let _e173 = (_e168.end & 31u);
                        let _e178 = ((_e136 >> bitcast<u32>(_e173)) & 1u);
                        let _e179 = ((_e138 >> bitcast<u32>(_e173)) & 1u);
                        if (_e168.end < 7u) {
                        } else {
                            phi_360_ = true;
                            break;
                        }
                        let _e185 = params.member.divisions[_e168.end].divisor;
                        let _e186 = (_e185 - 1u);
                        if (_e168.end < 8u) {
                        } else {
                            phi_360_ = true;
                            break;
                        }
                        let _e193 = local[_e168.end];
                        phi_215_ = vec2<u32>(((_e153.x * (1u + (_e178 * _e186))) + (_e193 * _e178)), ((_e153.y * (1u + (_e179 * _e186))) + (_e193 * _e179)));
                    } else {
                        phi_215_ = vec2<u32>();
                    }
                    let _e204 = phi_215_;
                    continue;
                    continuing {
                        phi_211_ = _e166;
                        phi_214_ = _e204;
                        phi_360_ = _e128;
                        break if !(_e172);
                    }
                }
                let _e207 = phi_360_;
                if _e207 {
                    break;
                }
                let _e209 = local_2;
                let _e212 = local_3;
                if (_e209.x < arrayLength((&input_a.member))) {
                } else {
                    break;
                }
                let _e217 = input_a.member[_e209.x];
                if (_e212.y < arrayLength((&input_b.member))) {
                } else {
                    break;
                }
                let _e221 = input_b.member[_e212.y];
                let _e224 = params.member.operation;
                if (_e224 == 0u) {
                    if (_e44 < arrayLength((&output.member))) {
                    } else {
                        break;
                    }
                    output.member[_e44] = (_e217 + _e221);
                }
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
