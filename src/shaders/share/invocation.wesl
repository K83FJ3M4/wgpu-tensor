fn invocationIndex(id: vec3u, numWorkgroups: vec3u) -> u32 {
    return id.x + (numWorkgroups.x * 256u) * (
        id.y + numWorkgroups.y * id.z
    );
}
