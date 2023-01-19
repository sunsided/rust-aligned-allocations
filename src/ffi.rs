use std::mem::ManuallyDrop;
use std::ptr::null_mut;
use crate::alloc_result::AllocResult;

/// Information about the allocated memory.
#[repr(C)]
pub struct Memory {
    /// The allocation status: 0 if valid.
    pub status: u32,
    /// Allocation flags. Used internally when calling free.
    pub flags: u32,
    /// The number of allocated bytes. Used internally when calling free.
    pub num_bytes: u32,
    /// The address of the allocated memory.
    pub address: *mut std::ffi::c_void,
}

pub static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0");

/// Gets a version reference in order to identify the library version.
#[no_mangle]
pub unsafe extern "C" fn version() -> *const libc::c_char {
    VERSION.as_ptr() as *const libc::c_char
}

/// Allocates memory of the specified number of bytes.
///
/// The optimal alignment will be determined by the number of bytes provided.
/// If the amount of bytes is a multiple of 2MB, Huge/Large Page support is enabled.
#[no_mangle]
pub unsafe extern "C" fn allocate_block(num_bytes: u32, sequential: bool, clear: bool) -> Memory {
    match crate::memory::Memory::allocate(num_bytes as usize, sequential, clear) {
        Ok(memory) => {
            let memory = ManuallyDrop::new(memory);
            Memory {
                status: AllocResult::Ok as u32,
                flags: memory.flags,
                num_bytes: memory.num_bytes as u32,
                address: memory.address,
            }
        },
        Err(e) => {
            let result: AllocResult = e.into();
            Memory {
                status: result as u32,
                flags: 0,
                num_bytes: 0,
                address: null_mut(),
            }
        }
    }

}

/// Frees memory of the specified number of bytes.
///
/// The memory instance is required to be created by `allocate`.
#[no_mangle]
pub unsafe extern "C" fn free_block(memory: Memory) {
    // NOTE: If this method is called "free", it'll shadow the version from clib ... don't do that.
    let mut wrapped: crate::memory::Memory = memory.into();
    wrapped.free();
}

impl Into<crate::memory::Memory> for Memory {
    fn into(self) -> crate::memory::Memory {
        crate::memory::Memory::new(
            AllocResult::from(self.status),
            self.flags,
            self.num_bytes as usize,
            self.address,
        )
    }
}
