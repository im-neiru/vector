use std::sync::Arc;

#[derive(Debug)]
#[allow(unused)]
pub(crate) struct SourceInfo {
    name: Box<str>,
    symbol_name: Box<str>,
    kind: shaderc::ShaderKind,
    output: Box<std::path::Path>,
    source: Box<std::path::Path>,
}

impl SourceInfo {
    pub(crate) fn get_sources(
        builder: Arc<crate::builder::Directories>,
        kind: shaderc::ShaderKind,
    ) -> Box<[Self]> {
        let dir = std::fs::read_dir(match kind {
            shaderc::ShaderKind::Vertex => {
                builder.vertex_shader_dir.as_ref()
            }
            shaderc::ShaderKind::Fragment => {
                builder.fragment_shader_dir.as_ref()
            }
            shaderc::ShaderKind::Compute => {
                builder.compute_shader_dir.as_ref()
            }
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
                    _ => {
                        panic!("Shader kind not available yet")
                    }
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

            let output = builder
                .output_dir
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
                    Some(Self {
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
                Some(Self {
                    name,
                    symbol_name,
                    kind,
                    output,
                    source: path.into_boxed_path(),
                })
            }
        });

        iterator.collect::<Box<[Self]>>()
    }

    pub(crate) fn compile(
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
