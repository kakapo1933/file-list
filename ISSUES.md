# Known Issues and Future Improvements

This document tracks all known issues, bugs, and potential improvements for the `file-list` (fls) project.

## 🔴 Critical Issues

### 1. Platform Compatibility (Unix-only)
- **Problem**: Code uses Unix-specific imports that prevent compilation on Windows
- **Affected files**: `src/main.rs` lines 5, 225-227, 284
- **Impact**: Program won't compile on Windows
- **Solution**: Add conditional compilation with `#[cfg(unix)]` or abstract platform-specific code
```rust
use std::os::unix::fs::{MetadataExt, PermissionsExt}; // Unix only!
```

### 2. Symlink Detection Bug
- **Problem**: Symlinks are incorrectly classified because `metadata()` follows symlinks
- **Affected files**: `src/main.rs` lines 189, 291 in `get_file_type()`
- **Impact**: Symlinks shown as regular files/directories
- **Solution**: Use `entry.path().symlink_metadata()` instead of `entry.metadata()`

### 3. Panic on Directory Sorting
- **Problem**: Sorting uses `unwrap()` which can panic on errored entries
- **Affected files**: `src/main.rs` lines 74-78
- **Impact**: Program crashes instead of graceful error handling
- **Solution**: Filter out errors before sorting
```rust
entries.sort_by(|a, b| {
    let a_name = a.as_ref().unwrap().file_name(); // Can panic!
    let b_name = b.as_ref().unwrap().file_name(); // Can panic!
    a_name.cmp(&b_name)
});
```

## 🟡 Moderate Issues

### 4. Special Permission Handling Bugs
- **Problem**: setuid/setgid/sticky bit display has multiple bugs
- **Affected files**: `src/main.rs` lines 203-214
- **Issues**:
  - Line 210 uses `replace_range` incorrectly for setgid
  - Doesn't handle uppercase 'S' when execute bit is not set
- **Impact**: Special permissions display incorrectly

### 5. Inefficient Color Application
- **Problem**: Multiple string replacements in `apply_colors_to_table()`
- **Affected files**: `src/main.rs` lines 337-429
- **Impact**: Performance degradation with large directories
- **Solution**: Consider single-pass replacement or different approach

### 6. Version Number Mismatch
- **Problem**: CLI shows v0.1.0 but Cargo.toml has v0.2.0
- **Affected files**: `src/main.rs` line 33
- **Impact**: Confusing version information
- **Solution**: Use `env!("CARGO_PKG_VERSION")` instead of hardcoded version

### 7. Incorrect Permission Display for 000 Mode
- **Problem**: Files with `chmod 000` incorrectly show as having permissions
- **Impact**: Misleading permission information
- **Solution**: Fix permission calculation logic

## 🟠 Minor Issues

### 8. Dead Code
- **Problem**: `_format_permissions()` function is unused
- **Affected files**: `src/main.rs` lines 182-218
- **Solution**: Remove or use the function

### 9. No Test Coverage
- **Problem**: Zero unit or integration tests
- **Impact**: Can't ensure reliability or catch regressions
- **Solution**: Add comprehensive test suite

### 10. Clippy Warnings
- **Problems**:
  - Needless borrow at line 153
  - Single-character string patterns at line 206
- **Solution**: Run `cargo clippy` and fix warnings

### 11. Long Filename Handling
- **Problem**: Very long filenames not truncated for terminal width
- **Impact**: Display issues with extreme cases
- **Solution**: Add terminal width detection and truncation

### 12. Inconsistent Error Messages
- **Problem**: Some errors use `eprintln!`, others just return
- **Impact**: Inconsistent user experience
- **Solution**: Standardize error handling approach

## 🔵 Missing Features

### 13. No Recursive Listing
- **Feature**: `-R` flag for recursive directory traversal
- **Impact**: Can't view nested directory structures

### 14. Limited Sorting Options
- **Current**: Only alphabetical sorting
- **Missing**: Sort by size (`-S`), date (`-t`), type (`-X`)

### 15. No File Filtering
- **Missing**: Filter by pattern, type, or size
- **Example**: `fls *.rs` or `fls --type=dir`

### 16. No Summary Statistics
- **Missing**: Total file count and cumulative size
- **Example**: "15 files, 3 directories, 245.3 MB total"

### 17. No Pagination
- **Problem**: Large directories dump everything at once
- **Solution**: Add pager support or built-in pagination

## 🟢 Code Quality Improvements

### 18. Magic Numbers
- **Problem**: Hard-coded values like `0o111`, `0o7777`
- **Solution**: Use named constants
```rust
const EXECUTE_MASK: u32 = 0o111;
const PERMISSION_MASK: u32 = 0o7777;
```

### 19. Long Functions
- **Problem**: `display_table_format()` and `apply_colors_to_table()` are too long
- **Solution**: Break into smaller, testable functions

### 20. No Configuration File
- **Missing**: User preferences for colors, default flags, etc.
- **Solution**: Add `~/.config/fls/config.toml` support

## 📋 Performance Improvements

### 21. Synchronous I/O Only
- **Problem**: All file operations are blocking
- **Impact**: Slow on network filesystems
- **Solution**: Consider async I/O with tokio

### 22. No Caching
- **Problem**: Re-reads metadata for color application
- **Solution**: Cache metadata during initial read

## 🛡️ Security Considerations

### 23. Path Traversal
- **Note**: No validation on user paths (may be intended behavior)
- **Consider**: Warning for sensitive directories

### 24. Error Information Leakage
- **Problem**: Full error messages might reveal system information
- **Solution**: Sanitize error output in production builds

## Priority Order for Fixes

1. **Immediate** (Breaking bugs):
   - Fix Unix-only compilation (#1)
   - Fix symlink detection (#2)
   - Fix panic on sorting (#3)
   - Fix version number (#6)

2. **Important** (Correctness):
   - Fix special permissions (#4)
   - Add test suite (#9)
   - Fix permission display (#7)

3. **Nice to Have** (Features):
   - Add recursive listing (#13)
   - Add sorting options (#14)
   - Add filtering (#15)

4. **Long Term** (Architecture):
   - Performance improvements (#5, #21, #22)
   - Configuration support (#20)
   - Cross-platform support

## How to Contribute

When fixing an issue:
1. Create a branch named `fix/issue-NUMBER`
2. Write tests for your fix
3. Update this file to mark the issue as resolved
4. Submit a PR referencing the issue number

---
*Last updated: January 2025*
*Version: 0.2.0*