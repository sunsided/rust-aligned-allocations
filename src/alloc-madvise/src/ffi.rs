use allocate::{AllocResult, Memory as MemorySafe};

/// Information about the allocated memory.
#[repr(C)]
pub struct Memory {
    /// The allocation status: 0 if valid.
    pub status: u32,
    /// Allocation flags. Used internally when calling free.
    pub flags: u32,
    /// The number of allocated bytes. Used internally when calling free.
    pub num_bytes: usize,
    /// The address of the allocated memory.
    pub address: *mut std::ffi::c_void,
}

/// Allocates memory of the specified number of bytes.
///
/// The optimal alignment will be determined by the number of bytes provided.
/// If the amount of bytes is a multiple of 2MB, Huge/Large Page support is enabled.
#[no_mangle]
pub unsafe extern "C" fn allocate(num_bytes: usize, clear: bool) -> Memory {
    let memory = crate::allocate(num_bytes, clear);
    Memory {
        status: memory.status as u32,
        flags: memory.flags,
        num_bytes: memory.num_bytes,
        address: memory.address,
    }
}

/// Frees memory of the specified number of bytes.
///
/// The memory instance is required to be created by `allocate`.
#[no_mangle]
pub unsafe extern "C" fn free(memory: Memory) {
    let wrapped = MemorySafe::new(
        AllocResult::from(memory.status),
        memory.flags,
        memory.num_bytes,
        memory.address,
    );

    crate::free(wrapped)
}
