mod sys;

use std::path::Path;

pub struct Slang {}

impl Slang {
    pub unsafe fn new() -> Self {
        unsafe { sys::slang_create_global_session(2) };
        Self {}
    }
}
