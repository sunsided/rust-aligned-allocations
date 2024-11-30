//! This module provides functions for allocating and deallocating aligned memory in Rust.
//!
//! The `alloc_aligned` function allocates a block of memory with a specified size and alignment,
//! and optionally clears the memory. It returns a pointer to the allocated memory or an error if
//! the allocation fails.
//!
//! The `free_aligned` function deallocates a block of memory that was previously allocated with
//! `alloc_aligned`. It takes a pointer to the memory, the size of the allocation, and the alignment
//! as arguments.
//!
//! # Safety
//!
//! The `free_aligned` function is marked as `unsafe` because it requires the caller to ensure that
//! the pointer passed to it was previously allocated by `alloc_aligned` with the same size and
//! alignment. Failure to uphold this contract can result in undefined behavior.

use crate::alloc_result::AllocationError;
use ::core::ptr;
use ::std::alloc;

/// Allocates memory according to the specified layout and returns a non-null pointer to the allocated memory.
/// 
/// If `clear` is true, the allocated memory is zero-initialized. Otherwise, the memory is uninitialized.
/// 
/// # Arguments
/// 
/// * `clear` - A boolean indicating whether the allocated memory should be zero-initialized.
/// * `layout` - The layout of the memory to be allocated.
/// 
/// # Returns
/// 
/// A `NonNull` pointer to the allocated memory, cast to `std::ffi::c_void`.
/// 
/// # Panics
/// 
/// This function will panic if the allocation fails and returns a null pointer.
/// 
/// # Safety
/// 
/// This function is unsafe because it performs a raw memory allocation, which can lead to undefined behavior if not used correctly.
/// The caller must ensure that the `layout` is valid and that the allocated memory is properly managed.
pub fn alloc_aligned(
    num_bytes: usize,
    alignment: usize,
    clear: bool,
) -> Result<ptr::NonNull<std::ffi::c_void>, AllocationError> {
    // https://users.rust-lang.org/t/how-can-i-allocate-aligned-memory-in-rust/33293/6
    if num_bytes == 0 {
        return Err(AllocationError::EmptyAllocation);
    }

    let layout = match alloc::Layout::from_size_align(num_bytes, alignment) {
        Err(e) => return Err(AllocationError::InvalidAlignment(e)),
        Ok(layout) => layout,
    };

    let address = ptr::NonNull::new(unsafe {
        if clear {
            // SAFETY: numbytes is guaranteed to be non-zero, ensuring that the allocation functions do not return null pointers unless an allocation error occurs.
            alloc::alloc_zeroed(layout)
        } else {
            // SAFETY: numbytes is guaranteed to be non-zero, ensuring that the allocation functions do not return null pointers unless an allocation error occurs.
            alloc::alloc(layout)
        }
    })
    .expect("ptr is null")
    .cast::<std::ffi::c_void>();

    Ok(address)
}


/// Deallocates a block of memory that was previously allocated with `alloc_aligned`.
///
/// # Arguments
///
/// * `ptr` - An `Option` containing a `NonNull` pointer to the memory to be deallocated, or `None`.
/// * `num_bytes` - The size of the allocation in bytes.
/// * `alignment` - The alignment of the allocation.
///
/// # Safety
///
/// This function is marked as `unsafe` because it requires the caller to ensure that:
/// - The pointer passed to it was previously allocated by `alloc_aligned` with the same size and alignment.
/// - The pointer is not null and is valid for the size and alignment specified.
///
/// Failure to uphold these requirements can result in undefined behavior.
///
/// If `ptr` is `None`, the function does nothing.
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(useless_ptr_null_checks)]
    fn test_alloc_aligned() {
        let num_bytes = 1024;
        let alignment = 16;
        let clear = true;

        let ptr = alloc_aligned(num_bytes, alignment, clear).expect("Allocation failed");
        assert!(!ptr.as_ptr().is_null());

        unsafe {
            free_aligned(Some(ptr), num_bytes, alignment);
        }
    }

    #[test]
    fn test_alloc_aligned_invalid_alignment() {
        let num_bytes = 1024;
        let alignment = 3; // Invalid alignment
        let clear = true;

        let result = alloc_aligned(num_bytes, alignment, clear);
        assert!(result.is_err());
    }

    #[test]
    fn test_alloc_aligned_zero_bytes() {
        let num_bytes = 0;
        let alignment = 16;
        let clear = true;

        let result = alloc_aligned(num_bytes, alignment, clear);
        assert!(result.is_err());
    }

    #[test]
    fn test_free_aligned_null_pointer() {
        let num_bytes = 1024;
        let alignment = 16;

        unsafe {
            free_aligned(None, num_bytes, alignment);
        }
    }
}