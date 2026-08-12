struct type_5 {
    member: array<f32>,
}

struct add_ShaderShape {
    d0_: u32,
    d1_: u32,
    d2_: u32,
    d3_: u32,
    d4_: u32,
    d5_: u32,
    d6_: u32,
    d7_: u32,
}

struct type_7 {
    member: add_ShaderShape,
}

var<private> invocation_1: vec3<u32>;
var<private> workgroups_1: vec3<u32>;
@group(0) @binding(0) 
var<storage> input_a_content: type_5;
@group(0) @binding(1) 
var<uniform> input_a_shape: type_7;
@group(1) @binding(0) 
var<storage> input_b_content: type_5;
@group(1) @binding(1) 
var<uniform> input_b_shape: type_7;
@group(2) @binding(0) 
var<storage, read_write> output_content: type_5;
@group(2) @binding(1) 
var<uniform> output_shape: type_7;

fn function_() {
    var phi_240_: u32;
    var phi_242_: u32;
    var phi_243_: u32;
    var phi_278_: u32;
    var phi_279_: u32;
    var phi_280_: u32;
    var phi_281_: u32;
    var phi_282_: u32;
    var phi_317_: u32;
    var phi_318_: u32;
    var phi_319_: u32;
    var phi_320_: u32;
    var phi_321_: u32;
    var phi_356_: u32;
    var phi_357_: u32;
    var phi_358_: u32;
    var phi_359_: u32;
    var phi_360_: u32;
    var phi_395_: u32;
    var phi_396_: u32;
    var phi_397_: u32;
    var phi_398_: u32;
    var phi_399_: u32;
    var phi_434_: u32;
    var phi_435_: u32;
    var phi_436_: u32;
    var phi_437_: u32;
    var phi_438_: u32;
    var phi_473_: u32;
    var phi_474_: u32;
    var phi_475_: u32;
    var phi_476_: u32;
    var phi_477_: u32;
    var phi_509_: u32;
    var phi_510_: u32;

    switch bitcast<i32>(0u) {
        default: {
            let _e18 = invocation_1;
            let _e19 = workgroups_1;
            let _e21 = arrayLength((&input_a_content.member));
            let _e23 = arrayLength((&input_b_content.member));
            let _e25 = arrayLength((&output_content.member));
            let _e35 = (_e18.x + (((_e18.y + (_e18.z * _e19.y)) * _e19.x) * 256u));
            let _e38 = output_shape.member.d0_;
            let _e41 = output_shape.member.d1_;
            let _e45 = output_shape.member.d2_;
            let _e49 = output_shape.member.d3_;
            let _e53 = output_shape.member.d4_;
            let _e57 = output_shape.member.d5_;
            let _e61 = output_shape.member.d6_;
            let _e65 = output_shape.member.d7_;
            if (_e35 >= (((((((_e38 * _e41) * _e45) * _e49) * _e53) * _e57) * _e61) * _e65)) {
            } else {
                let _e70 = input_a_shape.member.d0_;
                let _e74 = input_a_shape.member.d1_;
                let _e79 = input_a_shape.member.d2_;
                let _e84 = input_a_shape.member.d3_;
                let _e89 = input_a_shape.member.d4_;
                let _e94 = input_a_shape.member.d5_;
                let _e99 = input_a_shape.member.d6_;
                let _e104 = input_a_shape.member.d7_;
                let _e109 = input_b_shape.member.d0_;
                let _e113 = input_b_shape.member.d1_;
                let _e118 = input_b_shape.member.d2_;
                let _e123 = input_b_shape.member.d3_;
                let _e128 = input_b_shape.member.d4_;
                let _e133 = input_b_shape.member.d5_;
                let _e138 = input_b_shape.member.d6_;
                let _e143 = input_b_shape.member.d7_;
                if (((((((((_e70 == _e38) && (_e74 == _e41)) && (_e79 == _e45)) && (_e84 == _e49)) && (_e89 == _e53)) && (_e94 == _e57)) && (_e99 == _e61)) && (_e104 == _e65)) && ((((((((_e109 == _e38) && (_e113 == _e41)) && (_e118 == _e45)) && (_e123 == _e49)) && (_e128 == _e53)) && (_e133 == _e57)) && (_e138 == _e61)) && (_e143 == _e65))) {
                    if (_e35 < _e21) {
                    } else {
                        break;
                    }
                    let _e150 = input_a_content.member[_e35];
                    if (_e35 < _e23) {
                    } else {
                        break;
                    }
                    let _e154 = input_b_content.member[_e35];
                    if (_e35 < _e25) {
                    } else {
                        break;
                    }
                    output_content.member[_e35] = (_e150 + _e154);
                } else {
                    let _e159 = (_e38 > 1u);
                    if _e159 {
                        let _e160 = (_e38 == 0u);
                        if _e160 {
                            break;
                        }
                        let _e161 = (_e35 % _e38);
                        if _e160 {
                            break;
                        }
                        phi_240_ = (_e161 * select(0u, 1u, (_e109 != 1u)));
                        phi_242_ = (_e161 * select(0u, 1u, (_e70 != 1u)));
                        phi_243_ = (_e35 / _e38);
                    } else {
                        phi_240_ = 0u;
                        phi_242_ = 0u;
                        phi_243_ = _e35;
                    }
                    let _e170 = phi_240_;
                    let _e172 = phi_242_;
                    let _e174 = phi_243_;
                    let _e175 = select(1u, _e109, _e159);
                    let _e176 = select(1u, _e70, _e159);
                    if (_e41 > 1u) {
                        let _e178 = (_e41 == 0u);
                        if _e178 {
                            break;
                        }
                        let _e179 = (_e174 % _e41);
                        if _e178 {
                            break;
                        }
                        phi_278_ = (_e175 * _e113);
                        phi_279_ = (_e170 + ((_e179 * _e175) * select(0u, 1u, (_e113 != 1u))));
                        phi_280_ = (_e176 * _e74);
                        phi_281_ = (_e172 + ((_e179 * _e176) * select(0u, 1u, (_e74 != 1u))));
                        phi_282_ = (_e174 / _e41);
                    } else {
                        phi_278_ = _e175;
                        phi_279_ = _e170;
                        phi_280_ = _e176;
                        phi_281_ = _e172;
                        phi_282_ = _e174;
                    }
                    let _e194 = phi_278_;
                    let _e196 = phi_279_;
                    let _e198 = phi_280_;
                    let _e200 = phi_281_;
                    let _e202 = phi_282_;
                    if (_e45 > 1u) {
                        let _e204 = (_e45 == 0u);
                        if _e204 {
                            break;
                        }
                        let _e205 = (_e202 % _e45);
                        if _e204 {
                            break;
                        }
                        phi_317_ = (_e194 * _e118);
                        phi_318_ = (_e196 + ((_e205 * _e194) * select(0u, 1u, (_e118 != 1u))));
                        phi_319_ = (_e198 * _e79);
                        phi_320_ = (_e200 + ((_e205 * _e198) * select(0u, 1u, (_e79 != 1u))));
                        phi_321_ = (_e202 / _e45);
                    } else {
                        phi_317_ = _e194;
                        phi_318_ = _e196;
                        phi_319_ = _e198;
                        phi_320_ = _e200;
                        phi_321_ = _e202;
                    }
                    let _e220 = phi_317_;
                    let _e222 = phi_318_;
                    let _e224 = phi_319_;
                    let _e226 = phi_320_;
                    let _e228 = phi_321_;
                    if (_e49 > 1u) {
                        let _e230 = (_e49 == 0u);
                        if _e230 {
                            break;
                        }
                        let _e231 = (_e228 % _e49);
                        if _e230 {
                            break;
                        }
                        phi_356_ = (_e220 * _e123);
                        phi_357_ = (_e222 + ((_e231 * _e220) * select(0u, 1u, (_e123 != 1u))));
                        phi_358_ = (_e224 * _e84);
                        phi_359_ = (_e226 + ((_e231 * _e224) * select(0u, 1u, (_e84 != 1u))));
                        phi_360_ = (_e228 / _e49);
                    } else {
                        phi_356_ = _e220;
                        phi_357_ = _e222;
                        phi_358_ = _e224;
                        phi_359_ = _e226;
                        phi_360_ = _e228;
                    }
                    let _e246 = phi_356_;
                    let _e248 = phi_357_;
                    let _e250 = phi_358_;
                    let _e252 = phi_359_;
                    let _e254 = phi_360_;
                    if (_e53 > 1u) {
                        let _e256 = (_e53 == 0u);
                        if _e256 {
                            break;
                        }
                        let _e257 = (_e254 % _e53);
                        if _e256 {
                            break;
                        }
                        phi_395_ = (_e246 * _e128);
                        phi_396_ = (_e248 + ((_e257 * _e246) * select(0u, 1u, (_e128 != 1u))));
                        phi_397_ = (_e250 * _e89);
                        phi_398_ = (_e252 + ((_e257 * _e250) * select(0u, 1u, (_e89 != 1u))));
                        phi_399_ = (_e254 / _e53);
                    } else {
                        phi_395_ = _e246;
                        phi_396_ = _e248;
                        phi_397_ = _e250;
                        phi_398_ = _e252;
                        phi_399_ = _e254;
                    }
                    let _e272 = phi_395_;
                    let _e274 = phi_396_;
                    let _e276 = phi_397_;
                    let _e278 = phi_398_;
                    let _e280 = phi_399_;
                    if (_e57 > 1u) {
                        let _e282 = (_e57 == 0u);
                        if _e282 {
                            break;
                        }
                        let _e283 = (_e280 % _e57);
                        if _e282 {
                            break;
                        }
                        phi_434_ = (_e272 * _e133);
                        phi_435_ = (_e274 + ((_e283 * _e272) * select(0u, 1u, (_e133 != 1u))));
                        phi_436_ = (_e276 * _e94);
                        phi_437_ = (_e278 + ((_e283 * _e276) * select(0u, 1u, (_e94 != 1u))));
                        phi_438_ = (_e280 / _e57);
                    } else {
                        phi_434_ = _e272;
                        phi_435_ = _e274;
                        phi_436_ = _e276;
                        phi_437_ = _e278;
                        phi_438_ = _e280;
                    }
                    let _e298 = phi_434_;
                    let _e300 = phi_435_;
                    let _e302 = phi_436_;
                    let _e304 = phi_437_;
                    let _e306 = phi_438_;
                    if (_e61 > 1u) {
                        let _e308 = (_e61 == 0u);
                        if _e308 {
                            break;
                        }
                        let _e309 = (_e306 % _e61);
                        if _e308 {
                            break;
                        }
                        phi_473_ = (_e298 * _e138);
                        phi_474_ = (_e300 + ((_e309 * _e298) * select(0u, 1u, (_e138 != 1u))));
                        phi_475_ = (_e302 * _e99);
                        phi_476_ = (_e304 + ((_e309 * _e302) * select(0u, 1u, (_e99 != 1u))));
                        phi_477_ = (_e306 / _e61);
                    } else {
                        phi_473_ = _e298;
                        phi_474_ = _e300;
                        phi_475_ = _e302;
                        phi_476_ = _e304;
                        phi_477_ = _e306;
                    }
                    let _e324 = phi_473_;
                    let _e326 = phi_474_;
                    let _e328 = phi_475_;
                    let _e330 = phi_476_;
                    let _e332 = phi_477_;
                    if (_e65 > 1u) {
                        let _e334 = (_e65 == 0u);
                        if _e334 {
                            break;
                        }
                        let _e335 = (_e332 % _e65);
                        if _e334 {
                            break;
                        }
                        phi_509_ = (_e326 + ((_e335 * _e324) * select(0u, 1u, (_e143 != 1u))));
                        phi_510_ = (_e330 + ((_e335 * _e328) * select(0u, 1u, (_e104 != 1u))));
                    } else {
                        phi_509_ = _e326;
                        phi_510_ = _e330;
                    }
                    let _e347 = phi_509_;
                    let _e349 = phi_510_;
                    if (_e349 < _e21) {
                    } else {
                        break;
                    }
                    let _e353 = input_a_content.member[_e349];
                    if (_e347 < _e23) {
                    } else {
                        break;
                    }
                    let _e357 = input_b_content.member[_e347];
                    if (_e35 < _e25) {
                    } else {
                        break;
                    }
                    output_content.member[_e35] = (_e353 + _e357);
                }
            }
            break;
        }
    }
    return;
}

@compute @workgroup_size(256, 1, 1) 
fn main(@builtin(global_invocation_id) invocation: vec3<u32>, @builtin(num_workgroups) workgroups: vec3<u32>) {
    invocation_1 = invocation;
    workgroups_1 = workgroups;
    function_();
}
