
struct Builtin {
    @builtin(global_invocation_id) invocation: vec3u,
    @builtin(num_workgroups) numWorkgroups: vec3u
}

@compute @workgroup_size(256)
fn main(builtin: Builtin) {

}