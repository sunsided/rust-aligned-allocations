use std::ffi::c_void;
use std::ptr::{null_mut};
use libc::madvise;
use crate::alignment::get_alignment;
use crate::alloc_free::{alloc_aligned, free_aligned};
use crate::alloc_result::{AllocationError, AllocResult};

/// No special instructions.
const ALLOC_FLAGS_NONE: u32 = 0;

/// Indicates that huge pages should be used.
const ALLOC_FLAGS_HUGE_PAGES: u32 = 1 << 0;

/// Indicates that memory access is mainly sequential rather than random-access.
const ALLOC_FLAGS_SEQUENTIAL: u32 = 1 << 1;

/// Allocated memory.
#[derive(Debug)]
pub struct Memory {
    pub(crate) flags: u32,
    pub(crate) num_bytes: usize,
    pub(crate) address: *mut c_void,
}

impl Memory {
    /// Allocates memory of the specified number of bytes.
    ///
    /// The optimal alignment will be determined by the number of bytes provided.
    /// If the amount of bytes is a multiple of 2MB, Huge/Large Page support is enabled.
    ///
    /// ## Arguments
    /// * `num_bytes` - The number of bytes to allocate.
    /// * `sequential` - Whether or not the memory access pattern is sequential mostly.
    /// * `clear` - Whether or not to zero out the allocated memory.
    pub fn allocate(num_bytes: usize, sequential: bool, clear: bool) -> Result<Self, AllocationError> {
        if num_bytes == 0 {
            return Err(AllocationError::EmptyAllocation);
        }

        let alignment = get_alignment(num_bytes);
        let ptr = alloc_aligned(num_bytes, alignment.alignment, clear)?;

        let ptr: *mut c_void = ptr.as_ptr().cast::<c_void>();

        let mut advice = if sequential {
            libc::MADV_SEQUENTIAL
        } else {
            libc::MADV_NORMAL
        };

        let mut flags = if sequential {
            ALLOC_FLAGS_SEQUENTIAL
        } else {
            ALLOC_FLAGS_NONE
        };

        if alignment.use_huge_pages {
            advice |= libc::MADV_HUGEPAGE;
            flags |= ALLOC_FLAGS_HUGE_PAGES;
        };

        if advice != 0 {
            // See https://www.man7.org/linux/man-pages/man2/madvise.2.html
            // SAFETY: `ptr` came from alloc_aligned(num_bytes, alignment)
            unsafe { madvise(ptr, num_bytes, advice); }
        }

        Ok(Self::new(AllocResult::Ok, flags, num_bytes, ptr))
    }

    /// Frees memory of the specified number of bytes.
    ///
    /// The memory instance is required to be created by `allocate`.
    pub fn free(&mut self) {
        if self.address == null_mut() {
            return;
        }

        let alignment = get_alignment(self.num_bytes);

        debug_assert_ne!(self.address, null_mut());
        let ptr = core::ptr::NonNull::new(self.address);

        if (self.flags & ALLOC_FLAGS_HUGE_PAGES) == ALLOC_FLAGS_HUGE_PAGES {
            debug_assert!(alignment.use_huge_pages);

            // See https://www.man7.org/linux/man-pages/man2/madvise.2.html
            // SAFETY: `ptr` came from alloc_aligned(num_bytes, alignment)
            unsafe { madvise(self.address, self.num_bytes, libc::MADV_FREE); }
        }

        // SAFETY:
        // - `ptr` is checked for null before
        // - `num_bytes` and `alignment` are required to be correct by the caller
        unsafe { free_aligned(ptr, self.num_bytes, alignment.alignment); }

        // Zero out the fields.
        self.address = null_mut();
        self.num_bytes = 0;
    }

    pub(crate) fn new(
        status: AllocResult,
        flags: u32,
        num_bytes: usize,
        address: *mut c_void,
    ) -> Self {
        debug_assert!(status == AllocResult::Ok && address != null_mut() || address == null_mut(),
            "Found null pointer when allocation status was okay");
        Memory {
            flags,
            num_bytes,
            address,
        }
    }

    pub fn from_error(status: AllocResult) -> Self {
        assert_ne!(status, AllocResult::Ok);
        Memory {
            flags: 0,
            num_bytes: 0,
            address: null_mut(),
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory::from_error(AllocResult::Empty)
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        self.free()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO_MEGABYTES: usize = 2 * 1024 * 1024;
    const SIXTY_FOUR_BYTES: usize = 64;

    #[test]
    fn alloc_4mb_is_2mb_aligned_hugepage() {
        const SIZE: usize = TWO_MEGABYTES * 2;
        let memory = Memory::allocate(SIZE, true, true)
            .expect("allocation failed");

        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % TWO_MEGABYTES, 0);
        assert_eq!(
            memory.flags & ALLOC_FLAGS_HUGE_PAGES,
            ALLOC_FLAGS_HUGE_PAGES
        );
        assert_eq!(
            memory.flags & ALLOC_FLAGS_SEQUENTIAL,
            ALLOC_FLAGS_SEQUENTIAL
        );
    }

    #[test]
    fn alloc_4mb_nonsequential_is_2mb_aligned_hugepage() {
        const SIZE: usize = TWO_MEGABYTES * 2;
        let memory = Memory::allocate(SIZE, false, false)
            .expect("allocation failed");

        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % TWO_MEGABYTES, 0);
        assert_eq!(
            memory.flags & ALLOC_FLAGS_HUGE_PAGES,
            ALLOC_FLAGS_HUGE_PAGES
        );
        assert_ne!(
            memory.flags & ALLOC_FLAGS_SEQUENTIAL,
            ALLOC_FLAGS_SEQUENTIAL
        );
    }

    #[test]
    fn alloc_2mb_is_2mb_aligned_hugepage() {
        const SIZE: usize = TWO_MEGABYTES;
        let memory = Memory::allocate(SIZE, true, true)
            .expect("allocation failed");

        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % TWO_MEGABYTES, 0);
        assert_eq!(
            memory.flags & ALLOC_FLAGS_HUGE_PAGES,
            ALLOC_FLAGS_HUGE_PAGES
        );
    }

    #[test]
    fn alloc_1mb_is_64b_aligned() {
        const SIZE: usize = TWO_MEGABYTES / 2;
        let memory = Memory::allocate(SIZE, true, true)
            .expect("allocation failed");

        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % SIXTY_FOUR_BYTES, 0);
        assert_ne!(
            memory.flags & ALLOC_FLAGS_HUGE_PAGES,
            ALLOC_FLAGS_HUGE_PAGES
        );
    }

    #[test]
    fn alloc_63kb_is_64b_aligned() {
        const SIZE: usize = 63 * 1024;
        let memory = Memory::allocate(SIZE, true, true)
            .expect("allocation failed");

        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % SIXTY_FOUR_BYTES, 0);
        assert_ne!(
            memory.flags & ALLOC_FLAGS_HUGE_PAGES,
            ALLOC_FLAGS_HUGE_PAGES
        );
    }

    #[test]
    fn alloc_64kb_is_64b_aligned() {
        const SIZE: usize = 64 * 1024;
        let memory = Memory::allocate(SIZE, true, true)
            .expect("allocation failed");

        assert_ne!(memory.address, null_mut());
        assert_eq!((memory.address as usize) % SIXTY_FOUR_BYTES, 0);
        assert_ne!(
            memory.flags & ALLOC_FLAGS_HUGE_PAGES,
            ALLOC_FLAGS_HUGE_PAGES
        );
    }

    #[test]
    fn alloc_0b_is_not_allocated() {
        const SIZE: usize = 0;
        let err = Memory::allocate(SIZE, true, true)
            .expect_err("the allocation was empty");

        assert_eq!(err, AllocationError::EmptyAllocation);
    }
}