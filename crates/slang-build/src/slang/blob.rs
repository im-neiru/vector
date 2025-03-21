use std::os::raw::c_void;

#[repr(C)]
pub(crate) struct ISlangBlobVtbl {
    pub(crate) query_interface: unsafe extern "C" fn(
        this: *mut ISlangBlob,
        riid: *const Guid,
        object: *mut *mut c_void,
    )
        -> i32,
    pub(crate) add_ref:
        unsafe extern "C" fn(this: *mut ISlangBlob) -> u32,
    pub(crate) release:
        unsafe extern "C" fn(this: *mut ISlangBlob) -> u32,

    pub(crate) get_buffer_pointer:
        unsafe extern "C" fn(
            this: *mut ISlangBlob,
        ) -> *const c_void,
    pub(crate) get_buffer_size:
        unsafe extern "C" fn(this: *mut ISlangBlob) -> usize,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Guid {
    pub(crate) d1: u32,
    pub(crate) d2: u16,
    pub(crate) d3: u16,
    pub(crate) d4: [u8; 8],
}

#[repr(C)]
pub(crate) struct ISlangBlob {
    pub(crate) lp_vtable: *const ISlangBlobVtbl,
}

impl ISlangBlob {
    pub(crate) unsafe fn get_buffer_pointer(
        &self,
    ) -> *const c_void {
        unsafe {
            ((*self.lp_vtable).get_buffer_pointer)(
                self as *const _ as *mut _,
            )
        }
    }

    pub(crate) unsafe fn get_buffer_size(&self) -> usize {
        unsafe {
            ((*self.lp_vtable).get_buffer_size)(
                self as *const _ as *mut _,
            )
        }
    }

    pub(crate) unsafe fn as_slice(&self) -> &[u8] {
        let ptr =
            unsafe { self.get_buffer_pointer() } as *const u8;
        let size = unsafe { self.get_buffer_size() };
        unsafe { std::slice::from_raw_parts(ptr, size) }
    }
}
