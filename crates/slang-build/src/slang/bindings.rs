use super::{
    compile_request::ICompileRequestRef,
    global_session::IGlobalSessionRef,
    global_session_desc::SlangGlobalSessionDesc,
    result::SlangResult, source_language::SlangSourceLanguage,
};

#[link(name = "slang")]
unsafe extern "C" {
    #[link_name = "slang_createGlobalSession2"]
    pub(crate) fn slang_create_global_session2(
        desc: &SlangGlobalSessionDesc,
        out_global_session: &mut Option<IGlobalSessionRef>,
    ) -> SlangResult;

    #[link_name = "slang_shutdown"]
    pub(crate) fn slang_shutdown();

    #[link_name = "spCreateCompileRequest"]
    pub(crate) fn sp_create_compile_request(
        session: IGlobalSessionRef,
    ) -> Option<ICompileRequestRef>;

    #[link_name = "spDestroyCompileRequest"]
    pub(crate) fn sp_destroy_compile_request(
        compile_request: ICompileRequestRef,
    );

    #[link_name = "spAddSearchPath"]
    pub(crate) fn sp_add_search_path(
        compile_request: ICompileRequestRef,
        path: *const std::ffi::c_char,
    );

    #[link_name = "spAddTranslationUnit"]
    pub(crate) fn sp_add_translation_unit(
        compile_request: ICompileRequestRef,
        language: SlangSourceLanguage,
        module_name: *const std::ffi::c_char,
    ) -> u32;

    #[link_name = "spAddTranslationUnitSourceFile"]
    pub(crate) fn sp_add_translation_unit_source_file(
        compile_request: ICompileRequestRef,
        translation_unit_index: SlangSourceLanguage,
        path: *const std::ffi::c_char,
    );

    #[link_name = "spCompile"]
    pub(crate) fn sp_compile(
        compile_request: ICompileRequestRef,
    ) -> SlangResult;
}
