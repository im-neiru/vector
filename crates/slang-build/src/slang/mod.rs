mod sys;

use std::sync::{
    RwLock,
    atomic::{AtomicU32, Ordering, fence},
};

static GLOBAL_SESSION: GlobalSessionArc = GlobalSessionArc {
    session: RwLock::new(None),
    counter: AtomicU32::new(0),
};

struct GlobalSessionArc {
    session: RwLock<Option<sys::IGlobalSessionRef>>,
    counter: AtomicU32,
}

pub struct Slang {}

impl Slang {
    pub(crate) fn new() -> Self {
        let descriptor = sys::SlangGlobalSessionDesc::default();
        let mut global_session = None;

        {
            let session =
                GLOBAL_SESSION.session.read().unwrap();
            if session.is_some() {
                GLOBAL_SESSION
                    .counter
                    .fetch_add(1, Ordering::Relaxed);
                return Self::inner_new();
            }
        }

        unsafe {
            let result = sys::slang_create_global_session2(
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

        Self::inner_new()
    }

    #[inline]
    fn inner_new() -> Self {
        Self {}
    }
}

impl Drop for Slang {
    fn drop(&mut self) {
        let previous = GLOBAL_SESSION
            .counter
            .fetch_sub(1, Ordering::AcqRel);
        if previous == 1 {
            fence(Ordering::Acquire);

            unsafe { sys::slang_shutdown() };
            if let Ok(mut session) =
                GLOBAL_SESSION.session.write()
            {
                *session = None;
            }
        }
    }
}
