use crate::alloc_result::AllocationError;
use ::core::ptr;
use ::std::alloc;

// https://users.rust-lang.org/t/how-can-i-allocate-aligned-memory-in-rust/33293/6
pub fn alloc_aligned(
    num_bytes: usize,
    alignment: usize,
    clear: bool,
) -> Result<ptr::NonNull<std::ffi::c_void>, AllocationError> {
    if num_bytes == 0 {
        return Err(AllocationError::EmptyAllocation);
    }

    let layout = match alloc::Layout::from_size_align(num_bytes, alignment) {
        Err(e) => return Err(AllocationError::InvalidAlignment(e)),
        Ok(layout) => layout,
    };

    let address = ptr::NonNull::new(unsafe {
        if clear {
            // SAFETY: numbytes != 0
            alloc::alloc_zeroed(layout)
        } else {
            // SAFETY: numbytes != 0
            alloc::alloc(layout)
        }
    })
    .expect("ptr is null")
    .cast::<std::ffi::c_void>();

    return Ok(address);
}

/// # Safety
///
///   - `ptr`, when `NonNull`, must be a value returned by `alloc(numbytes, alignment)`
pub unsafe fn free_aligned(
    ptr: Option<ptr::NonNull<std::ffi::c_void>>,
    num_bytes: usize,
    alignment: usize,
) {
    let ptr = if let Some(ptr) = ptr {
        ptr
    } else {
        return;
    };

    let layout = alloc::Layout::from_size_align(num_bytes, alignment).unwrap_or_else(|err| {
        // Shouldn't happen if the layout is the same as on alloc.
        panic!("Memory layout error: {}", err)
    });

    // SAFETY: `ptr` came from alloc::alloc(layout);
    alloc::dealloc(ptr.cast::<u8>().as_ptr(), layout);
}
