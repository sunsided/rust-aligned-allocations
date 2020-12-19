# Aligned Allocators

This repository contains a set of memory allocators meant to be used
by .NET P/Invoke to create aligned chunks of memory in an optimal fashion.

## Build

```bash
cargo build --release
```

To strip object files in order to create a smaller library, run e.g.

```bash
strip target/release/liballoc_madvise.so
```

Note that future Cargo version will have an option of stripping debug symbols.
