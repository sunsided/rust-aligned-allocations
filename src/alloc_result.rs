//! Provides the [`AllocationError`] struct.

use std::alloc::LayoutError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum AllocationError {
    /// An allocation of zero bytes was attempted.
    EmptyAllocation,
    /// The generated memory layout was invalid.
    InvalidAlignment(LayoutError),
}

impl Error for AllocationError {}

impl From<LayoutError> for AllocationError {
    fn from(value: LayoutError) -> Self {
        Self::InvalidAlignment(value)
    }
}

impl Display for AllocationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AllocationError::EmptyAllocation => f.write_str("zero-byte allocation"),
            AllocationError::InvalidAlignment(e) => write!(f, "invalid memory layout: {e}"),
        }
    }
}

impl From<AllocationError> for AllocResult {
    fn from(val: AllocationError) -> Self {
        match val {
            AllocationError::EmptyAllocation => AllocResult::Empty,
            AllocationError::InvalidAlignment(_) => AllocResult::InvalidAlignment,
        }
    }
}

#[repr(u32)]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AllocResult {
    Ok = 0,
    Empty = 1 << 0,
    InvalidAlignment = 1 << 1,
}

impl From<u32> for AllocResult {
    fn from(value: u32) -> Self {
        match value {
            0 => AllocResult::Ok,
            1 => AllocResult::Empty,
            2 => AllocResult::InvalidAlignment,
            _ => panic!(),
        }
    }
}
