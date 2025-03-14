use std::path::PathBuf;

fn main() {
    shader_compile::Builder::create(
        PathBuf::from("shaders/vertex").into_boxed_path(),
        PathBuf::from("shaders/fragment").into_boxed_path(),
        PathBuf::from("shaders/compute").into_boxed_path(),
        PathBuf::from("src/spirv").into_boxed_path(),
    )
    .compile_shaders();
}
