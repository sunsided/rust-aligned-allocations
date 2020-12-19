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

/// Gets a git version reference in order to identify the library version.
const char *git_version();

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
