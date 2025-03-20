use std::ptr::NonNull;

#[link(name = "slang")]
unsafe extern "C" {

    #[link_name = "slang_createGlobalSession"]
    pub(super) fn slang_create_global_session(version: i32);
}
