use allocate::{AllocResult, Memory as MemorySafe};
use git_version::git_version;

const GIT_VERSION: &str = git_version!();

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

/// Gets a git version reference in order to identify the library version.
#[no_mangle]
pub unsafe extern "C" fn git_version() -> *const libc::c_char {
    std::ffi::CString::new(GIT_VERSION).unwrap().into_raw()
}

/// Allocates memory of the specified number of bytes.
///
/// The optimal alignment will be determined by the number of bytes provided.
/// If the amount of bytes is a multiple of 2MB, Huge/Large Page support is enabled.
#[no_mangle]
pub unsafe extern "C" fn allocate_block(num_bytes: u32, sequential: bool, clear: bool) -> Memory {
    let memory = crate::allocate(num_bytes as usize, sequential, clear);
    Memory {
        status: memory.status as u32,
        flags: memory.flags,
        num_bytes: memory.num_bytes as u32,
        address: memory.address,
    }
}

/// Frees memory of the specified number of bytes.
///
/// The memory instance is required to be created by `allocate`.
#[no_mangle]
pub unsafe extern "C" fn free_block(memory: Memory) {
    // NOTE: If this method is called "free", it'll shadow the version from clib ... don't do that.
    let wrapped = MemorySafe::new(
        AllocResult::from(memory.status),
        memory.flags,
        memory.num_bytes as usize,
        memory.address,
    );

    crate::free(wrapped)
}
