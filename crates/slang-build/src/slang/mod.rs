use std::ptr::NonNull;

mod sys;

pub struct Slang {
    global_session: NonNull<sys::IGlobalSession>,
}

impl Slang {
    pub unsafe fn new() -> Self {
        let descriptor = sys::SlangGlobalSessionDesc::default();
        let mut global_session = None;

        unsafe {
            let result = sys::slang_create_global_session2(
                &descriptor,
                &mut global_session,
            );

            if result.failed() {
                panic!("slang_createGlobalSession2 failed");
            }
        };

        let global_session = global_session.unwrap();

        Self { global_session }
    }
}
