mod lib_extern;

use allocate::{alloc_aligned, alloc_flags, free_aligned, get_alignment, AllocResult, Memory};
use libc::madvise;
use std::ptr::null_mut;

/// Allocates memory of the specified number of bytes.
///
/// The optimal alignment will be determined by the number of bytes provided.
/// If the amount of bytes is a multiple of 2MB, Huge/Large Page support is enabled.
pub unsafe fn allocate(num_bytes: usize, clear: bool) -> Memory {
    if num_bytes == 0 {
        return Memory::default();
    }

    let alignment = get_alignment(num_bytes);
    let memory = alloc_aligned(num_bytes, alignment.alignment, clear);
    if memory.is_none() {
        return Memory::from_error(AllocResult::InvalidAlignment);
    }

    let ptr: *mut std::ffi::c_void = memory.unwrap().as_ptr().cast::<std::ffi::c_void>();

    let flags = if alignment.use_huge_pages {
        // See https://www.man7.org/linux/man-pages/man2/madvise.2.html
        // SAFETY: `ptr` came from alloc_aligned(num_bytes, alignment)
        madvise(ptr, num_bytes, libc::MADV_HUGEPAGE);
        alloc_flags::HUGE_PAGES
    } else {
        alloc_flags::NONE
    };

    Memory::new(AllocResult::Ok, flags, num_bytes, ptr)
}

/// Frees memory of the specified number of bytes.
///
/// The memory instance is required to be created by `allocate`.
pub unsafe fn free(memory: Memory) {
    if memory.status != AllocResult::Ok {
        debug_assert_eq!(memory.address, null_mut());
        return;
    }

    let alignment = get_alignment(memory.num_bytes);

    debug_assert_ne!(memory.address, null_mut());
    let ptr = ::core::ptr::NonNull::new(memory.address);

    if (memory.flags & alloc_flags::HUGE_PAGES) == alloc_flags::HUGE_PAGES {
        debug_assert!(alignment.use_huge_pages);

        // See https://www.man7.org/linux/man-pages/man2/madvise.2.html
        // SAFETY: `ptr` came from alloc_aligned(num_bytes, alignment)
        madvise(memory.address, memory.num_bytes, libc::MADV_FREE);
    }

    // SAFETY:
    // - `ptr` is checked for null before
    // - `num_bytes` and `alignment` are required to be correct by the caller
    free_aligned(ptr, memory.num_bytes, alignment.alignment);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO_MEGABYTES: usize = 2 * 1024 * 1024;
    const SIXTY_FOUR_BYTES: usize = 64;

    #[test]
    fn alloc_4mb_is_2mb_aligned_hugepage() {
        const SIZE: usize = TWO_MEGABYTES * 2;
        let memory = unsafe { allocate(SIZE, true) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % TWO_MEGABYTES, 0);
        assert_eq!(
            memory.flags & alloc_flags::HUGE_PAGES,
            alloc_flags::HUGE_PAGES
        );

        unsafe { free(memory) };
    }

    #[test]
    fn alloc_2mb_is_2mb_aligned_hugepage() {
        const SIZE: usize = TWO_MEGABYTES;
        let memory = unsafe { allocate(SIZE, true) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % TWO_MEGABYTES, 0);
        assert_eq!(
            memory.flags & alloc_flags::HUGE_PAGES,
            alloc_flags::HUGE_PAGES
        );

        unsafe { free(memory) };
    }

    #[test]
    fn alloc_1mb_is_64b_aligned() {
        const SIZE: usize = TWO_MEGABYTES / 2;
        let memory = unsafe { allocate(SIZE, true) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % SIXTY_FOUR_BYTES, 0);
        assert_ne!(
            memory.flags & alloc_flags::HUGE_PAGES,
            alloc_flags::HUGE_PAGES
        );

        unsafe { free(memory) };
    }

    #[test]
    fn alloc_63kb_is_64b_aligned() {
        const SIZE: usize = 63 * 1024;
        let memory = unsafe { allocate(SIZE, true) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % SIXTY_FOUR_BYTES, 0);
        assert_ne!(
            memory.flags & alloc_flags::HUGE_PAGES,
            alloc_flags::HUGE_PAGES
        );

        unsafe { free(memory) };
    }

    #[test]
    fn alloc_64kb_is_64b_aligned() {
        const SIZE: usize = 64 * 1024;
        let memory = unsafe { allocate(SIZE, true) };

        assert_eq!(memory.status, AllocResult::Ok);
        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % SIXTY_FOUR_BYTES, 0);
        assert_ne!(
            memory.flags & alloc_flags::HUGE_PAGES,
            alloc_flags::HUGE_PAGES
        );

        unsafe { free(memory) };
    }

    #[test]
    fn alloc_0b_is_not_allocated() {
        const SIZE: usize = 0;
        let memory = unsafe { allocate(SIZE, true) };

        assert_eq!(memory.status, AllocResult::Empty);
        assert_eq!(memory.address, null_mut());
        assert_ne!(
            memory.flags & alloc_flags::HUGE_PAGES,
            alloc_flags::HUGE_PAGES
        );

        // We're still calling free in this test because it shouldn't panic.
        unsafe { free(memory) };
    }
}
