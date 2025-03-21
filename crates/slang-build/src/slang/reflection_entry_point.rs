use std::ptr::NonNull;

#[repr(C)]
pub(crate) struct SlangReflectionEntryPoint {
    _phantom: u32,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub(crate) struct SlangReflectionEntryPointRef(
    NonNull<SlangReflectionEntryPoint>,
);

unsafe impl Send for SlangReflectionEntryPointRef {}
unsafe impl Sync for SlangReflectionEntryPointRef {}
