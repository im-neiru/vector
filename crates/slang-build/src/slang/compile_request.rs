use std::{
    ffi::{CStr, CString},
    ptr::NonNull,
};

use super::{
    bindings::*, blob::ISlangBlob,
    compile_target::SlangCompileTarget,
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

pub(crate) struct CompileRequest {
    reference: ICompileRequestRef,
    target_index: i32,
}

impl CompileRequest {
    #[inline]
    pub(crate) fn create(session: IGlobalSessionRef) -> Self {
        let reference = unsafe {
            sp_create_compile_request(session).unwrap()
        };

        let target_index = unsafe {
            sp_add_code_gen_target(
                reference,
                SlangCompileTarget::Spirv,
            )
        };

        Self {
            reference,
            target_index,
        }
    }

    #[inline]
    pub(crate) fn add_search_path(&self, path: &CString) {
        unsafe {
            sp_add_search_path(self.reference, path.as_ptr())
        };
    }

    #[inline]
    pub(crate) fn add_translation_unit(
        &self,
        module_name: &CStr,
    ) -> u32 {
        unsafe {
            sp_add_translation_unit(
                self.reference,
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
                self.reference,
                translation_unit_index,
                path.as_ptr(),
            )
        }
    }

    #[inline]
    pub(crate) fn compile(&self) {
        unsafe {
            if sp_compile(self.reference).failed() {
                panic!("Failed compile")
            };

            let mut blob = None;

            if sp_get_entry_point_code_blob(
                self.reference,
                0,
                self.target_index,
                &mut blob,
            )
            .failed()
            {
                println!("Failed sp_get_entry_point_code_blob");
            }

            let blob = blob.unwrap();

            let bytes = blob.as_ref().as_slice();

            if let Some(diagnostic) =
                sp_get_diagnostic_output(self.reference)
                    .map(|v| CStr::from_ptr(v.as_ptr()))
            {
                if !diagnostic.is_empty() {
                    println!("{diagnostic:?}");
                }
            }
        };
    }
}

impl Drop for CompileRequest {
    fn drop(&mut self) {
        unsafe {
            sp_destroy_compile_request(self.reference);
        }
    }
}

impl core::fmt::Debug for CompileRequest {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        core::fmt::Debug::fmt(&self.reference.0, f)
    }
}
