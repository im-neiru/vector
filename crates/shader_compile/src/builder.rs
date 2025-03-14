use std::{path::Path, sync::Arc};

use shaderc::ShaderKind;

pub struct Builder {
    directories: Arc<Directories>,
}

pub(crate) struct Directories {
    pub(crate) vertex_shader_dir: Box<Path>,
    pub(crate) fragment_shader_dir: Box<Path>,
    pub(crate) compute_shader_dir: Box<Path>,
    pub(crate) output_dir: Box<Path>,
}

impl Builder {
    #[inline]
    pub fn create(
        vertex_shader_dir: Box<Path>,
        fragment_shader_dir: Box<Path>,
        compute_shader_dir: Box<Path>,
        output_dir: Box<Path>,
    ) -> Self {
        if !vertex_shader_dir.is_dir() {
            panic!("{vertex_shader_dir:?} is not a directory")
        }
        if !fragment_shader_dir.is_dir() {
            panic!("{fragment_shader_dir:?} is not a directory")
        }
        if !compute_shader_dir.is_dir() {
            panic!("{compute_shader_dir:?} is not a directory")
        }

        if !output_dir.is_dir() {
            panic!("{compute_shader_dir:?} is not a directory")
        }

        Self {
            directories: Arc::new(Directories {
                vertex_shader_dir,
                fragment_shader_dir,
                compute_shader_dir,
                output_dir,
            }),
        }
    }

    pub fn compile_shaders(self) {
        use std::thread;

        let vertex_handle = {
            let directories = self.directories.clone();

            thread::spawn(move || {
                let vertex_shaders =
                    crate::SourceInfo::get_sources(
                        directories,
                        ShaderKind::Vertex,
                    );

                if vertex_shaders.is_empty() {
                    return;
                }

                let compiler =
                    shaderc::Compiler::new().unwrap();
                let option =
                    shaderc::CompileOptions::new().unwrap();

                for info in vertex_shaders {
                    info.compile(&compiler, &option);
                }
            })
        };

        let fragment_handle = {
            let directories = self.directories.clone();
            thread::spawn(move || {
                let fragment_shaders =
                    crate::SourceInfo::get_sources(
                        directories,
                        ShaderKind::Fragment,
                    );

                if fragment_shaders.is_empty() {
                    return;
                }

                let compiler =
                    shaderc::Compiler::new().unwrap();
                let option =
                    shaderc::CompileOptions::new().unwrap();

                for info in fragment_shaders {
                    info.compile(&compiler, &option);
                }
            })
        };

        let compute_handle = {
            let directories = self.directories.clone();
            thread::spawn(move || {
                let compute_shaders =
                    crate::SourceInfo::get_sources(
                        directories,
                        shaderc::ShaderKind::Compute,
                    );

                if compute_shaders.is_empty() {
                    return;
                }

                let compiler =
                    shaderc::Compiler::new().unwrap();
                let option =
                    shaderc::CompileOptions::new().unwrap();

                for info in compute_shaders {
                    info.compile(&compiler, &option);
                }
            })
        };

        vertex_handle.join().unwrap();
        fragment_handle.join().unwrap();
        compute_handle.join().unwrap();
    }
}
