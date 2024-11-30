//! # alloc-madvise
//!
//! Rust-aligned-allocations provides aligned memory allocation utilities for Rust.
//!
//! This crate offers functionality to allocate and manage memory with specific alignment requirements.
//! The main features include:
//!
//! - [`Memory`] - A safe wrapper around aligned memory allocations
//! - [`AllocationError`] - Error type for memory allocation failures
//!
//! # Example
//!
//! ```
//! use alloc_madvise::{Memory, AllocationError};
//!
//! fn main() -> Result<(), AllocationError> {
//!     // Allocate 1024 bytes aligned to 64 bytes
//!     const SIZE: usize = 1024;
//!     const SEQUENTIAL: bool = true;
//!     const CLEAR: bool = true;
//!     let memory = Memory::allocate(SIZE, SEQUENTIAL, CLEAR)?;
//!     
//!     // Use the allocated memory...
//!     assert_ne!(memory.as_ptr(), std::ptr::null_mut());
//!     assert_eq!((memory.as_ptr() as usize) % 64, 0);
//!     assert_eq!(memory.len(), SIZE);
//!     assert!(!memory.is_empty());
//!     
//!     // Memory is automatically freed when dropped
//!     Ok(())
//! }
//! ```
//!
//! # Features
//!
//! - `ffi`: Enables FFI bindings for C interoperability (disabled by default)
#![allow(unsafe_code)]

#[cfg(feature = "ffi")]
mod ffi;

mod alignment;
mod alloc_free;
mod alloc_result;
mod memory;

pub use alloc_result::AllocationError;
pub use memory::Memory;
