fn main() {
    compile_shaders();
}

fn compile_shaders() {
    use std::thread;

    let vertex_handle = thread::spawn(move || {
        let vertex_shaders =
            get_entries(shaderc::ShaderKind::Vertex);

        if vertex_shaders.is_empty() {
            return;
        }

        let compiler = shaderc::Compiler::new().unwrap();
        let option = shaderc::CompileOptions::new().unwrap();

        for shader in vertex_shaders {
            shader.compile(&compiler, &option);
        }
    });

    let fragment_handle = thread::spawn(move || {
        let fragment_shaders =
            get_entries(shaderc::ShaderKind::Fragment);

        if fragment_shaders.is_empty() {
            return;
        }

        let compiler = shaderc::Compiler::new().unwrap();
        let option = shaderc::CompileOptions::new().unwrap();

        for shader in fragment_shaders {
            shader.compile(&compiler, &option);
        }
    });

    let compute_handle = thread::spawn(move || {
        let compute_shaders =
            get_entries(shaderc::ShaderKind::Compute);

        if compute_shaders.is_empty() {
            return;
        }

        let compiler = shaderc::Compiler::new().unwrap();
        let option = shaderc::CompileOptions::new().unwrap();

        for shader in compute_shaders {
            shader.compile(&compiler, &option);
        }
    });

    vertex_handle.join().unwrap();
    fragment_handle.join().unwrap();
    compute_handle.join().unwrap();
}

fn get_entries(
    kind: shaderc::ShaderKind,
) -> Box<[CompilerShaderEntry]> {
    let dir = std::fs::read_dir(match kind {
        shaderc::ShaderKind::Vertex => "shaders/vertex",
        shaderc::ShaderKind::Fragment => "shaders/fragment",
        shaderc::ShaderKind::Compute => "shaders/compute",
        _ => panic!("Shader kind not available yet"),
    })
    .unwrap();

    let iterator = dir.filter_map(|entry| {
        let entry = entry.ok()?;

        let metadata = entry.metadata().ok()?;

        if metadata.is_dir() {
            return None;
        }

        let path = entry.path();

        let extension =
            path.extension().unwrap().to_str().unwrap();

        if !(extension.eq("glsl")
            || match kind {
                shaderc::ShaderKind::Vertex => {
                    extension.eq("vert")
                }
                shaderc::ShaderKind::Fragment => {
                    extension.eq("frag")
                }
                shaderc::ShaderKind::Compute => {
                    extension.eq("comp")
                }
                _ => panic!("Shader kind not available yet"),
            })
        {
            return None;
        }

        let modified = metadata.modified().unwrap();
        let name = path
            .file_name()?
            .to_str()?
            .trim_end_matches(".glsl")
            .trim_end_matches(".vert")
            .trim_end_matches(".frag")
            .trim_end_matches(".comp")
            .to_string()
            .into_boxed_str();

        let symbol_name = match kind {
            shaderc::ShaderKind::Vertex => {
                format!("vs_{name}")
            }
            shaderc::ShaderKind::Fragment => {
                format!("fs_{name}")
            }
            shaderc::ShaderKind::Compute => {
                format!("cs_{name}")
            }
            _ => panic!("Shader kind not available yet"),
        }
        .into_boxed_str();

        let output = std::path::Path::new("src/spirv")
            .join(symbol_name.as_ref())
            .with_extension("spv")
            .into_boxed_path();

        if output.as_ref().is_file() {
            let dest_metadata =
                std::fs::metadata(output.as_ref())
                    .unwrap()
                    .modified()
                    .unwrap();

            // Check if the source is modified
            if modified > dest_metadata {
                Some(CompilerShaderEntry {
                    name,
                    symbol_name,
                    kind,
                    output,
                    source: path.into_boxed_path(),
                })
            } else {
                None
            }
        } else {
            Some(CompilerShaderEntry {
                name,
                symbol_name,
                kind,
                output,
                source: path.into_boxed_path(),
            })
        }
    });

    iterator.collect::<Box<[CompilerShaderEntry]>>()
}

#[derive(Debug)]
struct CompilerShaderEntry {
    name: Box<str>,
    symbol_name: Box<str>,
    kind: shaderc::ShaderKind,
    output: Box<std::path::Path>,
    source: Box<std::path::Path>,
}

impl CompilerShaderEntry {
    fn compile(
        &self,
        compiler: &shaderc::Compiler,
        option: &shaderc::CompileOptions,
    ) {
        let binary = compiler
            .compile_into_spirv(
                std::fs::read_to_string(self.source.as_ref())
                    .unwrap()
                    .as_str(),
                self.kind,
                self.source
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap(),
                "main",
                Some(option),
            )
            .unwrap();

        std::fs::write(
            self.output.as_ref(),
            binary.as_binary_u8(),
        )
        .unwrap();
    }
}
