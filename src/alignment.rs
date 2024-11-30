const TWO_MEGABYTES: usize = 2 * 1024 * 1024;
const SIXTY_FOUR_BYTES: usize = 64;
const ZERO_BYTES: usize = 0;

/// An alignment hint to control subsequent allocations or de-allocations.
pub struct AlignmentHint {
    /// The alignment byte boundary.
    ///
    /// If a zero-byte allocation is attempted, this value will be zero.
    /// In all other cases, the value is positive and a multiple of 64.
    pub alignment: usize,

    /// Whether the use of Huge/Large Pages are suggested.
    pub use_huge_pages: bool,
}

/// Gets the optimal alignment for the number of bytes.
///
/// If the number of bytes is a multiple of 2 MB, a natural 2 MB boundary
/// is selected and a hint for using Huge/Large Pages is issued.
///
/// In any other case, an alignment of 64 byte boundaries is produced, which
/// should be optimal for both AVX-2 and AVX-512 operations.
///
/// ## Arguments
/// * `num_bytes` - The number of bytes to allocate.
///
/// ## Returns
/// An [`AlignmentHint`] outlining the suggested alignment.
pub fn get_alignment(num_bytes: usize) -> AlignmentHint {
    if num_bytes == 0 {
        AlignmentHint {
            alignment: ZERO_BYTES,
            use_huge_pages: false,
        }
    } else if (num_bytes & (TWO_MEGABYTES - 1)) == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_alignment_4mb_is_2mb_aligned_hugepage() {
        let memory = get_alignment(2 * TWO_MEGABYTES);
        assert_eq!(memory.alignment, TWO_MEGABYTES);
        assert!(memory.use_huge_pages);
    }

    #[test]
    fn get_alignment_2mb_is_2mb_aligned_hugepage() {
        let memory = get_alignment(TWO_MEGABYTES);
        assert_eq!(memory.alignment, TWO_MEGABYTES);
        assert!(memory.use_huge_pages);
    }

    #[test]
    fn get_alignment_1mb_is_64b_aligned() {
        let memory = get_alignment(TWO_MEGABYTES / 2);
        assert_eq!(memory.alignment, SIXTY_FOUR_BYTES);
        assert!(!memory.use_huge_pages);
    }

    #[test]
    fn get_alignment_63kb_is_64b_aligned() {
        let memory = get_alignment(63 * 1024);
        assert_eq!(memory.alignment, SIXTY_FOUR_BYTES);
        assert!(!memory.use_huge_pages);
    }

    #[test]
    fn get_alignment_64kb_is_64b_aligned() {
        let memory = get_alignment(64 * 1024);
        assert_eq!(memory.alignment, SIXTY_FOUR_BYTES);
        assert!(!memory.use_huge_pages);
    }

    #[test]
    fn get_alignment_0b_is_0b_aligned() {
        let memory = get_alignment(0);
        assert_eq!(memory.alignment, ZERO_BYTES);
        assert!(!memory.use_huge_pages);
    }
}
