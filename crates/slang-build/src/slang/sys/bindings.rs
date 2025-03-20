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
}
