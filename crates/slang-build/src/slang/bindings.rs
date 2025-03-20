use super::{
    compile_request::ICompileRequestRef,
    global_session::IGlobalSessionRef,
    slang_global_session_desc::SlangGlobalSessionDesc,
    slang_result::SlangResult,
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
        session: ICompileRequestRef,
    );

    #[link_name = "spAddSearchPath"]
    pub(crate) fn sp_add_search_path(
        session: ICompileRequestRef,
        path: *const std::ffi::c_char,
    );

    #[link_name = "spCompile"]
    pub(crate) fn sp_compile(
        session: ICompileRequestRef,
    ) -> SlangResult;
}
