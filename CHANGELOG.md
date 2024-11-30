# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Provided `From<Memory>` and `From<AllocationError>` implementations to
  replace the previous `Into` impls.

### Internal

- Upgraded `cbindgen` from 0.26.0 to 0.27.0.

## 0.4.0 - 2024-06-19

### Internal

- Updated bindgen build dependencies.

## 0.3.0 - 2023-01-19

### Added

- Added `AsRef` and `AsMut` for primitive type slice mappings into `Memory`.
- Added `ffi` create feature (enabled by default) to enable or disable
  FFI binding generation.

## 0.2.0 - 2023-01-19

### Added

- Added the `alloc-madvise` library which dynamically aligns to 2 MB boundaries
  and signals a Huge Page request if the requested memory region is a multiple of 2 MB in size, or aligns at
  or aligns at 64 byte boundaries otherwise.
- Added a C binding for `alloc-madvise`, providing `allocate_block()` and `free_block()`.
- Added support for a `sequential` hint to `madvise` which should help accessing
  vector elements in order.
- Added Git version information to `alloc-madvise`, providing `git_version()`.
