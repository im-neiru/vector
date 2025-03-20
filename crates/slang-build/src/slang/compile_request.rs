use std::{ffi::CString, path::Path, ptr::NonNull};

use super::bindings::*;

#[repr(C)]
pub(crate) struct ICompileRequest {
    _phantom: u32,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct ICompileRequestRef(NonNull<ICompileRequest>);

unsafe impl Send for ICompileRequestRef {}
unsafe impl Sync for ICompileRequestRef {}

pub(crate) struct CompileRequest(pub(super) ICompileRequestRef);

impl CompileRequest {
    #[inline]
    pub(crate) fn add_search_path(&self, path: &Path) {
        let path =
            CString::new(path.to_str().unwrap().as_bytes())
                .unwrap();

        unsafe { sp_add_search_path(self.0, path.as_ptr()) };
    }

    #[inline]
    pub(crate) fn compile(&self) {
        unsafe {
            if sp_compile(self.0).failed() {
                panic!("Failed compile")
            }
        };
    }
}

impl Drop for CompileRequest {
    fn drop(&mut self) {
        unsafe {
            sp_destroy_compile_request(self.0);
        }
    }
}
