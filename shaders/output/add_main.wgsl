struct type_5 {
    member: array<f32>,
}

struct add_ShaderShape {
    lower: vec4<u32>,
    upper: vec4<u32>,
}

struct type_8 {
    member: add_ShaderShape,
}

var<private> index_1: vec3<u32>;
@group(0) @binding(0) 
var<storage> input_a_content: type_5;
@group(0) @binding(1) 
var<uniform> input_a_shape: type_8;
@group(1) @binding(0) 
var<storage> input_b_content: type_5;
@group(1) @binding(1) 
var<uniform> input_b_shape: type_8;
@group(2) @binding(0) 
var<storage, read_write> output_content: type_5;
@group(2) @binding(1) 
var<uniform> output_shape: type_8;

fn function_() {
    var local: array<u32, 8>;
    var local_1: array<u32, 8>;
    var local_2: array<u32, 8>;
    var phi_183_: u32;
    var phi_203_: u32;
    var phi_222_: u32;
    var phi_225_: u32;
    var phi_227_: u32;
    var phi_229_: u32;
    var phi_231_: u32;
    var phi_233_: u32;
    var phi_235_: u32;
    var phi_237_: u32;
    var phi_281_: u32;
    var phi_295_: u32;
    var phi_314_: u32;
    var phi_315_: u32;
    var phi_316_: u32;
    var phi_317_: u32;
    var phi_318_: u32;
    var phi_319_: u32;
    var phi_320_: u32;
    var phi_321_: u32;
    var phi_223_: u32;
    var phi_226_: u32;
    var phi_228_: u32;
    var phi_230_: u32;
    var phi_232_: u32;
    var phi_234_: u32;
    var phi_236_: u32;
    var phi_238_: u32;
    var phi_323_: bool;
    var phi_324_: bool;
    var phi_374_: bool;
    var phi_373_: bool;
    var local_3: u32;
    var local_4: u32;
    var local_5: u32;
    var local_6: u32;
    var local_7: u32;
    var local_8: u32;
    var local_9: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e20 = index_1;
            let _e30 = input_a_shape.member.lower[0u];
            let _e34 = input_a_shape.member.lower[1u];
            let _e38 = input_a_shape.member.lower[2u];
            let _e42 = input_a_shape.member.lower[3u];
            let _e46 = input_a_shape.member.upper[0u];
            let _e50 = input_a_shape.member.upper[1u];
            let _e54 = input_a_shape.member.upper[2u];
            let _e58 = input_a_shape.member.upper[3u];
            local = array<u32, 8>(_e30, _e34, _e38, _e42, _e46, _e50, _e54, _e58);
            let _e63 = input_b_shape.member.lower[0u];
            let _e67 = input_b_shape.member.lower[1u];
            let _e71 = input_b_shape.member.lower[2u];
            let _e75 = input_b_shape.member.lower[3u];
            let _e79 = input_b_shape.member.upper[0u];
            let _e83 = input_b_shape.member.upper[1u];
            let _e87 = input_b_shape.member.upper[2u];
            let _e91 = input_b_shape.member.upper[3u];
            local_1 = array<u32, 8>(_e63, _e67, _e71, _e75, _e79, _e83, _e87, _e91);
            let _e96 = output_shape.member.lower[0u];
            let _e100 = output_shape.member.lower[1u];
            let _e104 = output_shape.member.lower[2u];
            let _e108 = output_shape.member.lower[3u];
            let _e112 = output_shape.member.upper[0u];
            let _e116 = output_shape.member.upper[1u];
            let _e120 = output_shape.member.upper[2u];
            let _e124 = output_shape.member.upper[3u];
            local_2 = array<u32, 8>(_e96, _e100, _e104, _e108, _e112, _e116, _e120, _e124);
            let _e128 = local_2[0u];
            if (_e20.x >= _e128) {
            } else {
                let _e132 = local_2[1u];
                if (_e20.y >= _e132) {
                } else {
                    let _e134 = local_2[0u];
                    let _e138 = local[0u];
                    let _e142 = local[1u];
                    if (_e142 == 1u) {
                        phi_183_ = 0u;
                    } else {
                        let _e144 = local[0u];
                        phi_183_ = (_e20.y * _e144);
                    }
                    let _e147 = phi_183_;
                    let _e150 = local_1[0u];
                    let _e154 = local_1[1u];
                    if (_e154 == 1u) {
                        phi_203_ = 0u;
                    } else {
                        let _e156 = local_1[0u];
                        phi_203_ = (_e20.y * _e156);
                    }
                    let _e159 = phi_203_;
                    let _e161 = local_2[0u];
                    let _e162 = local_2[1u];
                    let _e164 = local[0u];
                    let _e165 = local[1u];
                    let _e167 = local_1[0u];
                    let _e168 = local_1[1u];
                    phi_222_ = 2u;
                    phi_225_ = _e20.z;
                    phi_227_ = (_e167 * _e168);
                    phi_229_ = (_e164 * _e165);
                    phi_231_ = (_e161 * _e162);
                    phi_233_ = (select(_e20.x, 0u, (_e150 == 1u)) + _e159);
                    phi_235_ = (select(_e20.x, 0u, (_e138 == 1u)) + _e147);
                    phi_237_ = (_e20.x + (_e20.y * _e134));
                    loop {
                        let _e172 = phi_222_;
                        let _e174 = phi_225_;
                        let _e176 = phi_227_;
                        let _e178 = phi_229_;
                        let _e180 = phi_231_;
                        let _e182 = phi_233_;
                        let _e184 = phi_235_;
                        let _e186 = phi_237_;
                        local_3 = _e174;
                        local_4 = _e184;
                        local_5 = _e184;
                        local_6 = _e182;
                        local_7 = _e182;
                        local_8 = _e186;
                        local_9 = _e186;
                        let _e187 = (_e172 < 8u);
                        if _e187 {
                            if _e187 {
                            } else {
                                phi_374_ = true;
                                phi_373_ = bool();
                                break;
                            }
                            let _e189 = local_2[_e172];
                            let _e190 = (_e189 == 0u);
                            if _e190 {
                                phi_314_ = u32();
                                phi_315_ = u32();
                                phi_316_ = u32();
                                phi_317_ = u32();
                                phi_318_ = u32();
                                phi_319_ = u32();
                                phi_320_ = u32();
                                phi_321_ = u32();
                            } else {
                                if _e190 {
                                    phi_374_ = true;
                                    phi_373_ = bool();
                                    break;
                                }
                                let _e191 = (_e174 % _e189);
                                if _e190 {
                                    phi_374_ = true;
                                    phi_373_ = bool();
                                    break;
                                }
                                if _e187 {
                                } else {
                                    phi_374_ = true;
                                    phi_373_ = bool();
                                    break;
                                }
                                let _e196 = local[_e172];
                                if (_e196 == 1u) {
                                    phi_281_ = _e184;
                                } else {
                                    phi_281_ = (_e184 + (_e191 * _e178));
                                }
                                let _e201 = phi_281_;
                                if _e187 {
                                } else {
                                    phi_374_ = true;
                                    phi_373_ = bool();
                                    break;
                                }
                                let _e203 = local_1[_e172];
                                if (_e203 == 1u) {
                                    phi_295_ = _e182;
                                } else {
                                    phi_295_ = (_e182 + (_e191 * _e176));
                                }
                                let _e208 = phi_295_;
                                if _e187 {
                                } else {
                                    phi_374_ = true;
                                    phi_373_ = bool();
                                    break;
                                }
                                let _e210 = local[_e172];
                                if _e187 {
                                } else {
                                    phi_374_ = true;
                                    phi_373_ = bool();
                                    break;
                                }
                                let _e212 = local_1[_e172];
                                phi_314_ = (_e172 + 1u);
                                phi_315_ = (_e174 / _e189);
                                phi_316_ = (_e176 * _e212);
                                phi_317_ = (_e178 * _e210);
                                phi_318_ = (_e180 * _e189);
                                phi_319_ = _e208;
                                phi_320_ = _e201;
                                phi_321_ = (_e186 + (_e191 * _e180));
                            }
                            let _e216 = phi_314_;
                            let _e218 = phi_315_;
                            let _e220 = phi_316_;
                            let _e222 = phi_317_;
                            let _e224 = phi_318_;
                            let _e226 = phi_319_;
                            let _e228 = phi_320_;
                            let _e230 = phi_321_;
                            phi_223_ = _e216;
                            phi_226_ = _e218;
                            phi_228_ = _e220;
                            phi_230_ = _e222;
                            phi_232_ = _e224;
                            phi_234_ = _e226;
                            phi_236_ = _e228;
                            phi_238_ = _e230;
                            phi_323_ = select(true, false, _e190);
                            phi_324_ = _e190;
                        } else {
                            phi_223_ = u32();
                            phi_226_ = u32();
                            phi_228_ = u32();
                            phi_230_ = u32();
                            phi_232_ = u32();
                            phi_234_ = u32();
                            phi_236_ = u32();
                            phi_238_ = u32();
                            phi_323_ = false;
                            phi_324_ = false;
                        }
                        let _e233 = phi_223_;
                        let _e235 = phi_226_;
                        let _e237 = phi_228_;
                        let _e239 = phi_230_;
                        let _e241 = phi_232_;
                        let _e243 = phi_234_;
                        let _e245 = phi_236_;
                        let _e247 = phi_238_;
                        let _e249 = phi_323_;
                        let _e251 = phi_324_;
                        continue;
                        continuing {
                            phi_222_ = _e233;
                            phi_225_ = _e235;
                            phi_227_ = _e237;
                            phi_229_ = _e239;
                            phi_231_ = _e241;
                            phi_233_ = _e243;
                            phi_235_ = _e245;
                            phi_237_ = _e247;
                            phi_374_ = false;
                            phi_373_ = _e251;
                            break if !(_e249);
                        }
                    }
                    let _e254 = phi_374_;
                    let _e256 = phi_373_;
                    if _e254 {
                        break;
                    }
                    if select(true, false, _e256) {
                        let _e259 = local_3;
                        if (_e259 == 0u) {
                            let _e262 = local_4;
                            if (_e262 < arrayLength((&input_a_content.member))) {
                            } else {
                                break;
                            }
                            let _e266 = local_5;
                            let _e268 = input_a_content.member[_e266];
                            let _e270 = local_6;
                            if (_e270 < arrayLength((&input_b_content.member))) {
                            } else {
                                break;
                            }
                            let _e274 = local_7;
                            let _e276 = input_b_content.member[_e274];
                            let _e278 = local_8;
                            if (_e278 < arrayLength((&output_content.member))) {
                            } else {
                                break;
                            }
                            let _e282 = local_9;
                            output_content.member[_e282] = (_e268 + _e276);
                        }
                    }
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(1, 1, 1) 
fn main(@builtin(global_invocation_id) index: vec3<u32>) {
    index_1 = index;
    function_();
}
