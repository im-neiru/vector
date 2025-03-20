use std::env::{VarError, var};

fn main() {
    let slang_lib = match var("SLANG_LIB") {
        Ok(path) => path,
        Err(VarError::NotPresent) => {
            panic!(
                "Error: The environment variable 'SLANG_LIB' is not set.\n\
                Please set 'SLANG_LIB' to the directory containing 'slang.lib'.\n\
                Example (Unix): export SLANG_LIB=/path/to/slang/lib\n\
                Example (Windows): set SLANG_LIB=C:\\path\\to\\slang\\lib"
            );
        }
        Err(VarError::NotUnicode(_)) => {
            panic!(
                "Error: The environment variable 'SLANG_LIB' contains invalid characters.\n\
                Please ensure it only contains valid Unicode characters."
            );
        }
    };

    println!("cargo:rustc-link-search=native={}", slang_lib);
    println!("cargo:rustc-link-lib=static=slang");
}
