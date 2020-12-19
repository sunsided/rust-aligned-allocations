use crate::AllocResult;
use std::ffi::c_void;

#[repr(C)]
pub struct Memory {
    pub status: AllocResult,
    pub flags: u32,
    pub num_bytes: usize,
    pub address: *mut c_void,
}

impl Memory {
    pub fn new(
        status: AllocResult,
        flags: u32,
        num_bytes: usize,
        address: *mut std::ffi::c_void,
    ) -> Self {
        Memory {
            status,
            flags,
            num_bytes,
            address,
        }
    }

    pub fn from_error(status: AllocResult) -> Self {
        assert_ne!(status, AllocResult::Ok);
        Memory {
            status,
            flags: 0,
            num_bytes: 0,
            address: std::ptr::null_mut(),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory::from_error(AllocResult::Empty)
    }
}
