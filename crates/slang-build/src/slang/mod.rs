use std::ptr::NonNull;

mod sys;

pub struct Slang {
    global_session: NonNull<sys::IGlobalSession>,
}

impl Slang {
    pub unsafe fn new() -> Self {
        let mut global_session = None;

        unsafe {
            sys::slang_create_global_session2(
                0,
                &mut global_session,
            );
        };

        let global_session = global_session.unwrap();

        Self { global_session }
    }
}
