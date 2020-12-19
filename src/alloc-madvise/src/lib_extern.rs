use allocate::Memory;

/// Allocates memory of the specified number of bytes.
///
/// The optimal alignment will be determined by the number of bytes provided.
/// If the amount of bytes is a multiple of 2MB, Huge/Large Page support is enabled.
#[no_mangle]
pub unsafe extern "fastcall" fn allocate(num_bytes: usize, clear: bool) -> Memory {
    crate::allocate(num_bytes, clear)
}

/// Frees memory of the specified number of bytes.
///
/// The memory instance is required to be created by `allocate`.
#[no_mangle]
pub unsafe extern "fastcall" fn free(memory: Memory) {
    crate::free(memory)
}
