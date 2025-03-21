use std::ptr::NonNull;

use super::{
    blob::ISlangBlob, compile_request::ICompileRequestRef,
    compile_target::SlangCompileTarget,
    global_session::IGlobalSessionRef,
    global_session_desc::SlangGlobalSessionDesc,
    optimization_level::SlangOptimizationLevel,
    reflect::SlangReflectionRef,
    reflection_entry_point::SlangReflectionEntryPointRef,
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

    #[link_name = "spSetOptimizationLevel"]
    pub(crate) fn sp_set_optimization_level(
        compile_request: ICompileRequestRef,
        level: SlangOptimizationLevel,
    );

    #[link_name = "spAddCodeGenTarget"]
    pub(crate) fn sp_add_code_gen_target(
        compile_request: ICompileRequestRef,
        target: SlangCompileTarget,
    ) -> i32;

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
        translation_unit_index: u32,
        path: *const std::ffi::c_char,
    );

    #[link_name = "spGetDiagnosticOutput"]
    pub(crate) fn sp_get_diagnostic_output(
        compile_request: ICompileRequestRef,
    ) -> Option<NonNull<std::ffi::c_char>>;

    #[link_name = "spGetEntryPointCodeBlob"]
    pub(crate) fn sp_get_entry_point_code_blob(
        compile_request: ICompileRequestRef,
        entry_point_index: u32,
        target_index: i32,
        blob: &mut Option<std::ptr::NonNull<ISlangBlob>>,
    ) -> SlangResult;

    #[link_name = "spCompile"]
    pub(crate) fn sp_compile(
        compile_request: ICompileRequestRef,
    ) -> SlangResult;

    #[link_name = "spGetReflection"]
    pub(crate) fn sp_get_reflection(
        compile_request: ICompileRequestRef,
    ) -> Option<SlangReflectionRef>;

    #[link_name = "spReflection_getEntryPointCount"]
    pub(crate) fn sp_reflection_get_entry_point_count(
        reflection: SlangReflectionRef,
    ) -> u32;

    #[link_name = "spReflection_getEntryPointByIndex"]
    pub(crate) fn sp_reflection_get_entry_point_by_index(
        reflection: SlangReflectionRef,
    ) -> Option<SlangReflectionEntryPointRef>;

    #[link_name = "spReflectionEntryPoint_getName"]
    pub(crate) fn sp_reflection_entry_point_get_name(
        reflection_entry_point: SlangReflectionEntryPointRef,
    ) -> Option<NonNull<std::ffi::c_char>>;
}
