#[macro_use]
extern crate bitflags;

mod alloc_flags;
mod alloc_result;
mod allocate;

use crate::alloc_flags::AllocFlags;
use crate::alloc_result::AllocResult;
use allocate::{alloc_aligned, free_aligned};
use libc::{c_void, madvise};
use std::ptr::null_mut;

#[repr(C)]
pub struct Memory {
    status: AllocResult,
    flags: AllocFlags,
    address: *mut c_void,
}

const TWO_MEGABYTES: usize = 2 * 1024 * 1024;
const SIXTY_FOUR_BYTES: usize = 64;

struct AlignmentHint {
    alignment: usize,
    use_huge_pages: bool,
}

/// Gets the optimal alignment for the number of bytes.
///
/// If the number of bytes is a multiple of 2 MB, a natural 2 MB boundary
/// is selected and a hint for using Huge/Large Pages is issued.
///
/// In any other case, an alignment of 64 byte boundaries is produced, which
/// should be optimal for both AVX-2 and AVX-512 operations.
fn get_alignment(num_bytes: usize) -> AlignmentHint {
    if (num_bytes & (TWO_MEGABYTES - 1)) == 0 {
        AlignmentHint {
            alignment: TWO_MEGABYTES,
            use_huge_pages: true,
        }
    } else {
        AlignmentHint {
            alignment: SIXTY_FOUR_BYTES,
            use_huge_pages: false,
        }
    }
}

/// Allocates memory of the specified number of bytes, given the specified alignment.
pub unsafe fn allocate(num_bytes: usize) -> Memory {
    if num_bytes == 0 {
        return Memory {
            status: AllocResult::Empty,
            flags: AllocFlags::NONE,
            address: null_mut(),
        };
    }

    let alignment = get_alignment(num_bytes);
    let memory = alloc_aligned(num_bytes, alignment.alignment);
    if memory.is_none() {
        return Memory {
            status: AllocResult::InvalidAlignment,
            flags: AllocFlags::NONE,
            address: null_mut(),
        };
    }

    let ptr: *mut c_void = memory.unwrap().as_ptr().cast::<c_void>();

    let flags = if alignment.use_huge_pages {
        // See https://www.man7.org/linux/man-pages/man2/madvise.2.html
        // SAFETY: `ptr` came from alloc_aligned(num_bytes, alignment)
        madvise(ptr, num_bytes, libc::MADV_HUGEPAGE);
        AllocFlags::HUGE_PAGES
    } else {
        AllocFlags::NONE
    };

    Memory {
        status: AllocResult::Ok,
        flags,
        address: ptr,
    }
}

pub unsafe fn free(memory: Memory, num_bytes: usize) {
    if memory.status != AllocResult::Ok {
        debug_assert_eq!(memory.address, null_mut());
        return;
    }

    let alignment = get_alignment(num_bytes);

    debug_assert_ne!(memory.address, null_mut());
    let ptr = ::core::ptr::NonNull::new(memory.address);

    if memory.flags.contains(AllocFlags::HUGE_PAGES) {
        debug_assert!(alignment.use_huge_pages);

        // See https://www.man7.org/linux/man-pages/man2/madvise.2.html
        // SAFETY: `ptr` came from alloc_aligned(num_bytes, alignment)
        madvise(memory.address, num_bytes, libc::MADV_FREE);
    }

    // SAFETY:
    // - `ptr` is checked for null before
    // - `num_bytes` and `alignment` are required to be correct by the caller
    free_aligned(ptr, num_bytes, alignment.alignment);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alloc_4mb_is_2mb_aligned_hugepage() {
        const SIZE: usize = TWO_MEGABYTES * 2;
        let memory = unsafe { allocate(SIZE) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % TWO_MEGABYTES, 0);
        assert!(memory.flags.contains(AllocFlags::HUGE_PAGES));

        unsafe { free(memory, SIZE) };
    }

    #[test]
    fn alloc_2mb_is_2mb_aligned_hugepage() {
        const SIZE: usize = TWO_MEGABYTES;
        let memory = unsafe { allocate(SIZE) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % TWO_MEGABYTES, 0);
        assert!(memory.flags.contains(AllocFlags::HUGE_PAGES));

        unsafe { free(memory, SIZE) };
    }

    #[test]
    fn alloc_1mb_is_64b_aligned() {
        const SIZE: usize = TWO_MEGABYTES / 2;
        let memory = unsafe { allocate(SIZE) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % SIXTY_FOUR_BYTES, 0);
        assert!(!memory.flags.contains(AllocFlags::HUGE_PAGES));

        unsafe { free(memory, SIZE) };
    }

    #[test]
    fn alloc_63kb_is_64b_aligned() {
        const SIZE: usize = 63 * 1024;
        let memory = unsafe { allocate(SIZE) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % SIXTY_FOUR_BYTES, 0);
        assert!(!memory.flags.contains(AllocFlags::HUGE_PAGES));

        unsafe { free(memory, SIZE) };
    }

    #[test]
    fn alloc_64kb_is_64b_aligned() {
        const SIZE: usize = 64 * 1024;
        let memory = unsafe { allocate(SIZE) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % SIXTY_FOUR_BYTES, 0);
        assert!(!memory.flags.contains(AllocFlags::HUGE_PAGES));

        unsafe { free(memory, SIZE) };
    }

    #[test]
    fn alloc_0b_is_not_allocated() {
        const SIZE: usize = 0;
        let memory = unsafe { allocate(SIZE) };

        assert_eq!(memory.status, AllocResult::Empty);
        assert_eq!(memory.address, null_mut());
        assert!(!memory.flags.contains(AllocFlags::HUGE_PAGES));

        // We're still calling free in this test because it shouldn't panic.
        unsafe { free(memory, SIZE) };
    }
}
