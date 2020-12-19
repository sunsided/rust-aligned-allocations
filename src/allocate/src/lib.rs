#[macro_use]
extern crate bitflags;

mod alignment;
mod alloc_flags;
mod alloc_free;
mod alloc_result;

pub use alignment::{get_alignment, AlignmentHint};
pub use alloc_flags::AllocFlags;
pub use alloc_free::{alloc_aligned, free_aligned};
pub use alloc_result::AllocResult;
