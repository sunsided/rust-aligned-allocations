# Aligned Allocators

A memory allocator for creating large aligned chunks of memory in an optimal fashion. This library is meant 
to be used standalone or via FFI with the original use case being .NET P/Invoke.

## Build

```bash
cargo build --release
```

To strip object files in order to create a smaller library, run e.g.

```bash
strip target/release/liballoc_madvise.so
```

Note that future Cargo version will have an option of stripping debug symbols.
