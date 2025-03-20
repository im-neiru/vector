use std::ptr::NonNull;

#[repr(C)]
pub(crate) struct IGlobalSession {
    _phantom: u32,
}

#[repr(transparent)]
pub(crate) struct IGlobalSessionRef(NonNull<IGlobalSession>);

unsafe impl Send for IGlobalSessionRef {}
unsafe impl Sync for IGlobalSessionRef {}
