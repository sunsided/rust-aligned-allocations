//! This module provides FFI (Foreign Function Interface) bindings for memory allocation and deallocation.
//!
//! It includes functions to allocate and free memory blocks, as well as a structure to hold information
//! about the allocated memory. The module also provides a function to get the version of the library.
//!
//! # Structures
//!
//! - [`Memory`]: Holds information about the allocated memory, including status, flags, number of bytes, and address.
//!
//! # Functions
//!
//! - `version`: Returns a pointer to a C string containing the version of the library.
//! - `allocate_block`: Allocates a memory block of the specified number of bytes, with options for sequential and clear allocation.
//! - `free_block`: Frees a previously allocated memory block.
//!
//! # Safety
//!
//! All functions in this module are marked as `unsafe` because they involve raw pointers and FFI, which can lead to undefined behavior if misused.

use crate::alloc_result::AllocResult;
use std::mem::ManuallyDrop;
use std::ptr::null_mut;

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
        }
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

impl From<Memory> for crate::memory::Memory {
    fn from(val: Memory) -> Self {
        crate::memory::Memory::new(
            AllocResult::from(val.status),
            val.flags,
            val.num_bytes as usize,
            val.address,
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        unsafe {
            let version_ptr = version();
            let version_cstr = std::ffi::CStr::from_ptr(version_ptr);
            assert_eq!(version_cstr.to_str().unwrap(), env!("CARGO_PKG_VERSION"));
        }
    }

    #[test]
    fn test_allocate_block_success() {
        unsafe {
            let memory = allocate_block(1024, false, false);
            assert_eq!(memory.status, AllocResult::Ok as u32);
            assert_eq!(memory.num_bytes, 1024);
            assert!(!memory.address.is_null());
            free_block(memory);
        }
    }

    #[test]
    fn test_allocate_block_failure() {
        unsafe {
            let memory = allocate_block(0, false, false);
            assert_ne!(memory.status, AllocResult::Ok as u32);
            assert_eq!(memory.num_bytes, 0);
            assert!(memory.address.is_null());
        }
    }

    #[test]
    fn test_free_block() {
        unsafe {
            let memory = allocate_block(1024, false, false);
            assert_eq!(memory.status, AllocResult::Ok as u32);
            assert_eq!(memory.num_bytes, 1024);
            assert!(!memory.address.is_null());
            free_block(memory);
        }
    }
}
