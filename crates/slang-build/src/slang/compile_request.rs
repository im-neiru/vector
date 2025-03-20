use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use super::{
    bindings::*, compile_target::SlangCompileTarget,
    global_session::IGlobalSessionRef,
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
        let reference = unsafe {
            sp_create_compile_request(session).unwrap()
        };

        unsafe {
            sp_set_code_gen_target(
                reference,
                SlangCompileTarget::Spirv,
            );
        }

        Self(reference)
    }

    #[inline]
    pub(crate) fn add_search_path(&self, path: &CString) {
        unsafe { sp_add_search_path(self.0, path.as_ptr()) };
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

    #[inline]
    pub(crate) fn add_translation_unit_source_file(
        &self,
        translation_unit_index: u32,
        path: &CStr,
    ) {
        unsafe {
            sp_add_translation_unit_source_file(
                self.0,
                translation_unit_index,
                path.as_ptr(),
            )
        }
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

impl core::fmt::Debug for CompileRequest {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self.0.0, f)
    }
}
