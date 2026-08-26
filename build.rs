use wesl::Wesl;

fn main() {
    Wesl::new("./src/shaders")
        .build_artifact(
            &"package::unary".parse().unwrap(),
            "unary",
        );

    Wesl::new("./src/shaders")
        .build_artifact(
            &"package::binary".parse().unwrap(),
            "binary",
        );

    Wesl::new("./src/shaders")
        .build_artifact(
            &"package::reduction".parse().unwrap(),
            "reduction",
        );

    Wesl::new("./src/shaders")
        .build_artifact(
            &"package::matmul".parse().unwrap(),
            "matmul",
        );

    Wesl::new("./src/shaders")
        .build_artifact(
            &"package::constant".parse().unwrap(),
            "constant",
        );
}