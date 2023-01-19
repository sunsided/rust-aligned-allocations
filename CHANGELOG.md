# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Added `AsRef` and `AsMut` for primitive type slice mappings into `Memory`.

## 0.2.0 - 2023-01-19

### Added

- Added the `alloc-madvise` library which dynamically aligns to 2 MB boundaries
  and signals a Huge Page request if the requested memory region is a multiple of 2 MB in size, or aligns at
  or aligns at 64 byte boundaries otherwise.
- Added a C binding for `alloc-madvise`, providing `allocate_block()` and `free_block()`.
- Added support for a `sequential` hint to `madvise` which should help accessing
  vector elements in order.
- Added Git version information to `alloc-madvise`, providing `git_version()`.
