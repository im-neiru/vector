use std::ptr::NonNull;

#[repr(C)]
pub(crate) struct SlangReflection {
    _phantom: u32,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct SlangReflectionRef(NonNull<SlangReflection>);

unsafe impl Send for SlangReflectionRef {}
unsafe impl Sync for SlangReflectionRef {}
