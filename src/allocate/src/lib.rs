mod alignment;
mod alloc_free;
mod alloc_result;
mod memory;

pub mod alloc_flags;

pub use alignment::{get_alignment, AlignmentHint};
pub use alloc_free::{alloc_aligned, free_aligned};
pub use alloc_result::AllocResult;
pub use memory::Memory;
