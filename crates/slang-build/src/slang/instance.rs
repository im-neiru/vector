use std::sync::{
    RwLock,
    atomic::{AtomicU32, Ordering, fence},
};

use super::{
    bindings, compile_request::CompileRequest,
    global_session::IGlobalSessionRef,
    slang_global_session_desc::SlangGlobalSessionDesc,
};

static GLOBAL_SESSION: GlobalSessionArc = GlobalSessionArc {
    session: RwLock::new(None),
    counter: AtomicU32::new(0),
};

struct GlobalSessionArc {
    session: RwLock<Option<IGlobalSessionRef>>,
    counter: AtomicU32,
}

pub struct Slang;

impl Slang {
    pub(crate) fn new() -> Self {
        let descriptor = SlangGlobalSessionDesc::default();
        let mut global_session = None;

        {
            let session =
                GLOBAL_SESSION.session.read().unwrap();
            if session.is_some() {
                GLOBAL_SESSION
                    .counter
                    .fetch_add(1, Ordering::Relaxed);
                return Self;
            }
        }

        unsafe {
            let result = bindings::slang_create_global_session2(
                &descriptor,
                &mut global_session,
            );

            if result.failed() {
                panic!("slang_createGlobalSession2 failed");
            }

            let mut session =
                GLOBAL_SESSION.session.write().unwrap();
            *session = global_session.take();

            GLOBAL_SESSION
                .counter
                .fetch_add(1, Ordering::Relaxed);
        }

        Self
    }

    pub(crate) fn create_compile_request(
        &self,
    ) -> CompileRequest {
        let session = GLOBAL_SESSION.session.read().unwrap();
        let session = session.unwrap();
        CompileRequest::create(session)
    }
}

impl Drop for Slang {
    fn drop(&mut self) {
        let previous = GLOBAL_SESSION
            .counter
            .fetch_sub(1, Ordering::AcqRel);
        if previous == 1 {
            fence(Ordering::Acquire);

            unsafe { bindings::slang_shutdown() };
            if let Ok(mut session) =
                GLOBAL_SESSION.session.write()
            {
                *session = None;
            }
        }
    }
}
