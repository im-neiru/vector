pub(crate) struct Generator {
    id: syn::Expr,
    words: Box<[u32]>,
}

impl Generator {
    pub(crate) fn load(
        input: crate::input::IncludeSpirvInput,
    ) -> Self {
        let manifest_dir =
            std::env::var("CARGO_MANIFEST_DIR").unwrap();

        let file = std::path::Path::new(&manifest_dir)
            .join("src/spirv")
            .join(input.path.value());

        let words = {
            let bytes =
                std::fs::read(&file).unwrap_or_else(|_| {
                    panic!(
                        "{}",
                        file.to_str()
                            .as_ref()
                            .unwrap()
                            .to_string()
                    )
                });

            if bytes.len() % 4 != 0 {
                panic!(
                    "Invalid SPIR-V file: length {} is not a multiple of 4",
                    bytes.len()
                );
            }

            bytes
                .chunks_exact(4)
                .map(|chunk| {
                    let mut arr = [0u8; 4];
                    arr[..chunk.len()].copy_from_slice(chunk);
                    u32::from_le_bytes(arr)
                })
                .collect::<Box<[u32]>>()
        };

        Self {
            id: input.id,
            words,
        }
    }
}

impl From<Generator> for proc_macro::TokenStream {
    fn from(val: Generator) -> Self {
        let id = val.id;
        let words = val.words;

        quote::quote! {
            super::ShaderSource {
                id: #id,
                words:  &[#(#words),*]
            }
        }
        .into()
    }
}
