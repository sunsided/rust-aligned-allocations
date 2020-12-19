use ::core::ptr;
use ::std::alloc; // or extern crate alloc; use ::alloc::alloc;

// https://users.rust-lang.org/t/how-can-i-allocate-aligned-memory-in-rust/33293/6
pub fn alloc_aligned(num_bytes: usize, alignment: usize) -> Option<ptr::NonNull<libc::c_void>> {
    if num_bytes == 0 {
        return None;
    }

    let layout = alloc::Layout::from_size_align(num_bytes, alignment)
        .map_err(|err| eprintln!("Memory layout error: {}", err))
        .ok()?;

    let address = ptr::NonNull::new(unsafe {
        // SAFETY: numbytes != 0
        alloc::alloc(layout)
    })?
    .cast::<libc::c_void>();

    return Some(address);
}

/// # Safety
///
///   - `ptr`, when `NonNull`, must be a value returned by `alloc(numbytes, alignment)`
pub unsafe fn free_aligned(
    ptr: Option<ptr::NonNull<libc::c_void>>,
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
