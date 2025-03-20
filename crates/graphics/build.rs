use std::path::Path;

fn main() {
    slang_build::Builder::new(slang_build::Config {
        input_dir: Path::new("shaders"),
        output_dir: Path::new("src/shaders"),
    })
    .compile();
}
