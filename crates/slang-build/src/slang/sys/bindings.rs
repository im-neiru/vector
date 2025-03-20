#[link(name = "slang")]
unsafe extern "C" {
    #[link_name = "slang_createGlobalSession2"]
    pub(crate) fn slang_create_global_session2(
        desc: &super::SlangGlobalSessionDesc,
        out_global_session: &mut Option<
            super::IGlobalSessionRef,
        >,
    ) -> super::SlangResult;

    #[link_name = "slang_shutdown"]
    pub(crate) fn slang_shutdown();

    #[link_name = "spCreateCompileRequest"]
    pub(crate) fn sp_create_compile_request(
        session: super::IGlobalSessionRef,
    ) -> Option<super::ICompileRequestRef>;

    #[link_name = "spDestroyCompileRequest"]
    pub(crate) fn sp_destroy_compile_request(
        session: super::ICompileRequestRef,
    );

    #[link_name = "spAddSearchPath"]
    pub(crate) fn sp_add_search_path(
        session: super::ICompileRequestRef,
        path: *const std::ffi::c_char,
    );

    #[link_name = "spCompile"]
    pub(crate) fn sp_compile(
        session: super::ICompileRequestRef,
    ) -> super::SlangResult;
}
