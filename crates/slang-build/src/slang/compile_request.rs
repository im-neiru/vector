use std::{
    ffi::{CStr, CString},
    path::Path,
    ptr::NonNull,
};

use super::{
    bindings::*, global_session::IGlobalSessionRef, instance,
    source_language::SlangSourceLanguage,
};

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
    pub(crate) fn create(session: IGlobalSessionRef) -> Self {
        Self(unsafe {
            sp_create_compile_request(session).unwrap()
        })
    }

    #[inline]
    pub(crate) fn add_search_path(&self, path: &CString) {
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

    #[inline]
    pub(crate) fn add_translation_unit(
        &self,
        module_name: &CStr,
    ) -> u32 {
        unsafe {
            sp_add_translation_unit(
                self.0,
                SlangSourceLanguage::Slang,
                module_name.as_ptr(),
            )
        }
    }
}

impl Drop for CompileRequest {
    fn drop(&mut self) {
        unsafe {
            sp_destroy_compile_request(self.0);
        }
    }
}
