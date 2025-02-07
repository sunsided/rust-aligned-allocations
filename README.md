# Aligned Allocators (`alloc-madvise`)

[![Crates.io](https://img.shields.io/crates/v/alloc-madvise)](https://crates.io/crates/timelag)
[![Crates.io](https://img.shields.io/crates/l/alloc-madvise)](https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/sunsided/rust-aligned-allocations/rust.yml)](https://github.com/sunsided/rust-aligned-allocations/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/alloc-madvise)](https://docs.rs/alloc-madvise/)
[![Safety Dance][safety-image]][safety-link]
[![codecov](https://codecov.io/gh/sunsided/rust-aligned-allocations/graph/badge.svg?token=6CR3SYT3HT)](https://codecov.io/gh/sunsided/rust-aligned-allocations)

A memory allocator for creating large aligned chunks of memory in an optimal fashion. This library is meant 
to be used standalone or via FFI with the original use case being .NET P/Invoke.

Memory is dynamically aligned to the most efficient boundaries based on size,
ensuring that AVX workloads can use use aligned loads and stores. Huge Page support
is automatically enabled based on the allocation size. If the flag for sequential access is provided,
memory will be m-advised for fast scans rather than random accesses; clearing out
the memory is optional.

```rust
fn main() {
    const TWO_MEGABYTES: usize = 2 * 1024 * 1024;
    const SIZE: usize = TWO_MEGABYTES * 2;
    const SEQUENTIAL: bool = true;
    const CLEAR: bool = true;

    // Allocate 4 MiB of aligned, zeroed-out, sequential read memory.
    // The memory will be automatically freed when it leaves scope.
    let memory = Memory::allocate(SIZE, SEQUENTIAL, CLEAR)
        .expect("allocation failed");

    assert_ne!(memory.address, std::ptr::null_mut());
    assert_eq!((memory.address as usize) % TWO_MEGABYTES, 0);

    // Get a reference to a mutable slice.
    let data: &mut [f32] = memory.as_mut();
    data[0] = 1.234;
    data[1] = 5.678;

    // Get a reference to an immutable slice.
    let reference: &[f32] = memory.as_ref();
    assert_eq!(reference[0], 1.234);
    assert_eq!(reference[1], 5.678);
    assert_eq!(reference[2], 0.0);
    assert_eq!(reference.len(), memory.len() / std::mem::size_of::<f32>());
}
```

## Build

```bash
cargo build --release
```

To strip object files in order to create a smaller library, run e.g.

```bash
strip target/release/liballoc_madvise.so
```

Note that future Cargo version will have an option of stripping debug symbols.

## C/C++ FFI

For the FFI, the library is built in both dylib and staticlib flavors.
Building the crate auto-generates a header file containing the declarations:

```cpp
#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace ffi {

/// Information about the allocated memory.
struct Memory {
  /// The allocation status: 0 if valid.
  uint32_t status;
  /// Allocation flags. Used internally when calling free.
  uint32_t flags;
  /// The number of allocated bytes. Used internally when calling free.
  uint32_t num_bytes;
  /// The address of the allocated memory.
  void *address;
};

extern "C" {

/// Gets a version reference in order to identify the library version.
const char *version();

/// Allocates memory of the specified number of bytes.
///
/// The optimal alignment will be determined by the number of bytes provided.
/// If the amount of bytes is a multiple of 2MB, Huge/Large Page support is enabled.
Memory allocate_block(uint32_t num_bytes, bool sequential, bool clear);

/// Frees memory of the specified number of bytes.
///
/// The memory instance is required to be created by `allocate`.
void free_block(Memory memory);

} // extern "C"

} // namespace ffi
```


[safety-image]: https://img.shields.io/badge/unsafe-required-yellow.svg

[safety-link]: https://github.com/rust-secure-code/safety-dance/