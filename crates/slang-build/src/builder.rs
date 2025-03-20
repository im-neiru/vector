use std::ffi::CString;

use crate::{config, slang::Slang};

pub struct Builder {
    slang: Slang,
    search_path: CString,
}

impl Builder {
    #[allow(clippy::new_without_default)]
    pub fn new(config: config::Config) -> Self {
        let search_path =
            CString::new(config.input_dir.to_str().unwrap())
                .unwrap();

        Self {
            slang: Slang::new(),
            search_path,
        }
    }

    pub fn compile(self) {
        let compile_request =
            self.slang.create_compile_request();

        compile_request.add_search_path(&self.search_path);
        compile_request.compile();
    }
}
