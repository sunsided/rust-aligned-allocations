# Changelog

All notable changes to this project will be documented in this file.
This project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

### Added

- Added the `alloc-madvise` library which dynamically aligns to 2 MB boundaries
  and signals a Huge Page request if the requested memory region is a multiple of 2 MB in size, or aligns at
  or aligns at 64 byte boundaries otherwise.
