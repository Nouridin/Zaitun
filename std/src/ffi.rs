use std::os::raw::c_void;

pub struct ForeignPtr {
    ptr: *mut c_void,
    owner: bool,
}

impl ForeignPtr {
    pub unsafe fn new(ptr: *mut c_void, owner: bool) -> Self {
        ForeignPtr { ptr, owner }
    }

    pub fn as_ptr(&self) -> *const c_void {
        self.ptr
    }
}

impl Drop for ForeignPtr {
    fn drop(&mut self) {
        if self.owner && !self.ptr.is_null() {
            // Safety: Caller must ensure proper deallocation
            unsafe { libc::free(self.ptr) };
        }
    }
}