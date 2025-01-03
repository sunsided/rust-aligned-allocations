# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2024-11-30

[0.5.0]: https://github.com/sunsided/rust-aligned-allocations/releases/tag/v0.5.0

### Added

- Provided `From<Memory>` and `From<AllocationError>` implementations to
  replace the previous `Into` impls.
- Explicitly state `allow(unsafe_code)` for the library now.
- Added `Memory::to_ptr` returning an `Option<NonNull<c_void>>`.

### Changed

- The `Memory::as_ptr` and `Memory::as_ptr_mut` functions are now
  deprecated in favor of `Memory::to_ptr_const` and `Memory::to_ptr_mut`,
  as well as `Memory::to_ptr`.
- The MSRV was raised to 1.74 due to `cbindgen` requirements.

### Internal

- Upgraded `cbindgen` from 0.26.0 to 0.27.0.
- Rework `get_alignment_hint` into `AlignmentHint::new`.

## 0.4.0 - 2024-06-19

[0.4.0]: https://github.com/sunsided/rust-aligned-allocations/releases/tag/v0.4.0

### Internal

- Updated bindgen build dependencies.

## 0.3.0 - 2023-01-19

[0.3.0]: https://github.com/sunsided/rust-aligned-allocations/releases/tag/v0.3.0

### Added

- Added `AsRef` and `AsMut` for primitive type slice mappings into `Memory`.
- Added `ffi` create feature (enabled by default) to enable or disable
  FFI binding generation.

## 0.2.0 - 2023-01-19

[0.2.0]: https://github.com/sunsided/rust-aligned-allocations/releases/tag/v0.2.0

### Added

- Added the `alloc-madvise` library which dynamically aligns to 2 MB boundaries
  and signals a Huge Page request if the requested memory region is a multiple of 2 MB in size, or aligns at
  or aligns at 64 byte boundaries otherwise.
- Added a C binding for `alloc-madvise`, providing `allocate_block()` and `free_block()`.
- Added support for a `sequential` hint to `madvise` which should help accessing
  vector elements in order.
- Added Git version information to `alloc-madvise`, providing `git_version()`.
