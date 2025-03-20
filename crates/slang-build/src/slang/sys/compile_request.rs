use std::ptr::NonNull;

#[repr(C)]
pub(crate) struct ICompileRequest {
    _phantom: u32,
}

#[repr(transparent)]
pub(crate) struct ICompileRequestRef(NonNull<ICompileRequest>);

unsafe impl Send for ICompileRequestRef {}
unsafe impl Sync for ICompileRequestRef {}
