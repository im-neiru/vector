use std::{ffi::CString, path::Path, str::FromStr};

use crate::{config, slang::Slang};

pub struct Builder {
    slang: Slang,
    search_path: CString,
    output_dir: Box<Path>,
    entries: Box<[BuildEntry]>,
    optimization: crate::OptimizationLevel,
}

#[derive(Debug)]
struct BuildEntry {
    input: CString,
    module_name: CString,
}

impl Builder {
    #[allow(clippy::new_without_default)]
    pub fn new(config: config::Config) -> Self {
        let manifest =
            std::env::var_os("CARGO_MANIFEST_DIR").unwrap();
        let manifest = Path::new(&manifest);

        let input_dir_iter =
            if config.input_dir.is_absolute() {
                std::fs::read_dir(config.input_dir)
            } else {
                std::fs::read_dir(
                    manifest.join(config.input_dir),
                )
            }
            .unwrap();

        let entries = input_dir_iter
            .into_iter()
            .filter_map(|entry| {
                let entry = entry.ok()?;

                let input = entry.path();

                if !input.is_file() {
                    return None;
                }

                if let Some("slang") =
                    input.extension().and_then(|v| v.to_str())
                {
                    let file_name = input
                        .with_extension("")
                        .file_name()?
                        .to_owned();

                    // let output =
                    //     if config.output_dir.is_absolute() {
                    //         config.output_dir.join(&file_name)
                    //     } else {
                    //         manifest
                    //             .join(config.output_dir)
                    //             .join(&file_name)
                    //     }
                    //     .with_extension("spv");

                    // if output.exists() {
                    //     let input_modified_date = entry
                    //         .metadata()
                    //         .ok()?
                    //         .modified()
                    //         .ok()?;

                    //     let output_modified_date = output
                    //         .metadata()
                    //         .ok()?
                    //         .modified()
                    //         .ok()?;

                    //     if output_modified_date
                    //         > input_modified_date
                    //     {
                    //         return None;
                    //     }
                    // }

                    Some(BuildEntry {
                        input: CString::from_str(
                            input.to_str()?,
                        )
                        .ok()?,
                        module_name: CString::from_str(
                            &file_name.to_str()?.to_uppercase(),
                        )
                        .ok()?,
                    })
                } else {
                    None
                }
            })
            .collect::<Box<[BuildEntry]>>();

        let search_path =
            CString::new(config.input_dir.to_str().unwrap())
                .unwrap();

        Self {
            slang: Slang::new(),
            search_path,
            entries,
            output_dir: if config.output_dir.is_absolute() {
                config.output_dir.into()
            } else {
                manifest
                    .join(config.output_dir)
                    .into_boxed_path()
            },
            optimization: config.optimization,
        }
    }

    pub fn compile(self) {
        let compile_request = self
            .slang
            .create_compile_request(self.optimization);

        compile_request.add_search_path(&self.search_path);

        for entry in self.entries {
            let unit_index = compile_request
                .add_translation_unit(&entry.module_name);

            compile_request.add_translation_unit_source_file(
                unit_index,
                &entry.input,
            );
        }

        compile_request.compile(self.output_dir.as_ref());
    }
}
