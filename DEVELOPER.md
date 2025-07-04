# Developer Documentation

## Code Organization

This project follows a modular architecture with clear separation of concerns. Here's how the codebase is organized:

### Module Structure

```
src/
├── main.rs           # CLI entry point and argument parsing
├── config.rs         # Configuration struct and CLI option management  
├── file_info.rs      # File metadata, permissions, and FileInfo struct
├── formatting.rs     # Size, time, and permission formatting utilities
├── colors.rs         # Color logic and terminal hyperlink generation
└── display/
    ├── mod.rs        # Common display logic and entry point
    ├── simple.rs     # Simple format display implementation
    ├── table.rs      # Table format display with color application
    └── tree.rs       # Tree format display with recursive traversal
```

### Key Design Principles

1. **Single Responsibility**: Each module handles one specific aspect of functionality
2. **Type Safety**: Configuration is handled through a dedicated struct rather than multiple parameters
3. **Separation of Concerns**: Display logic is separated from formatting and color logic
4. **Testability**: Functions are small, focused, and easily testable
5. **Extensibility**: New display formats can be added easily by implementing new modules

### Module Dependencies

```
main.rs
├── config.rs
└── display/mod.rs
    ├── display/simple.rs
    │   └── colors.rs
    │       └── file_info.rs
    ├── display/table.rs
    │   ├── colors.rs
    │   ├── file_info.rs
    │   │   └── formatting.rs
    │   └── formatting.rs
    └── display/tree.rs
        ├── colors.rs
        └── file_info.rs
```

## Data Flow

1. **CLI Parsing**: `main.rs` parses command-line arguments using clap
2. **Configuration**: Arguments are converted to a `Config` struct (including `tree_depth`)
3. **Directory Reading**: `display::list_directory()` reads and sorts directory entries
4. **Display Routing**: Based on flags (`-t` for tree, `-l` for table), appropriate display module is called
5. **Format Selection**: Tree, table, or simple display is chosen based on config
6. **Information Extraction**: File metadata is extracted and formatted
7. **Color Application**: Colors and hyperlinks are applied based on file types
8. **Output Generation**: Final formatted output is printed to stdout

## Adding New Features

### Adding a New Display Format

1. Create a new module in `src/display/` (e.g., `json.rs`)
2. Implement a `display()` function that takes entries and config
3. Add the module to `src/display/mod.rs`
4. Add a new CLI flag in `main.rs`
5. Add the flag to the `Config` struct
6. Update `list_directory()` to handle the new format

### Adding New File Information

1. Add new fields to the `FileInfo` struct in `file_info.rs`
2. Update the `from_metadata()` method to populate the new fields
3. Add any necessary formatting functions to `formatting.rs`
4. Update table display to include the new columns

### Tree Display Implementation

The tree display module (`src/display/tree.rs`) implements hierarchical directory visualization:

1. **Recursive Traversal**: Uses `display_tree_recursive()` for depth-first traversal
2. **Depth Control**: Respects user-specified depth limits (`-L/--depth` flag)
3. **Unicode Drawing**: Uses box-drawing characters (├──, └──, │) for tree structure
4. **Safety Limits**: Enforces MAX_DEPTH (20) to prevent stack overflow
5. **Hidden Files**: Respects `--all` flag configuration
6. **Color Integration**: Uses `format_with_color()` for consistent file coloring
7. **Interactive Support**: Works with `-i` flag for clickable file names

Key implementation details:
- Depth validation: 1-50 levels (enforced at CLI parsing)
- Default behavior: Unlimited depth (up to MAX_DEPTH)
- Tree symbols adapt based on position (last item gets └── instead of ├──)
- Recursive calls track depth to enforce limits

### Adding New Color Schemes

1. Add new color functions to `colors.rs`
2. Update existing display modules to use the new colors
3. Consider adding CLI options to select color schemes

## Testing

### Unit Tests

Each module should have comprehensive unit tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(1024), "1.0K");
        assert_eq!(format_size(1536), "1.5K");
        assert_eq!(format_size(500), "500B");
    }
}
```

### Integration Tests

Create integration tests in `tests/` directory to test complete workflows:

```rust
// tests/integration_test.rs
use std::process::Command;

#[test]
fn test_basic_listing() {
    let output = Command::new("cargo")
        .args(&["run", "--", "tests/fixtures"])
        .output()
        .expect("Failed to execute command");
    
    assert!(output.status.success());
    // Add assertions about output content
}
```

## Performance Considerations

### Memory Usage

- Use iterators where possible to avoid loading all data into memory
- Consider streaming output for very large directories
- Profile memory usage with tools like `heaptrack` or `valgrind`

### CPU Usage

- File metadata operations are I/O bound, not CPU bound
- Color string generation is the main CPU cost
- Consider caching color strings for repeated file types

### Benchmarking

Use Criterion for benchmarking:

```rust
// benches/formatting.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use file_list::formatting::format_size;

fn benchmark_format_size(c: &mut Criterion) {
    c.bench_function("format_size", |b| {
        b.iter(|| format_size(black_box(1024 * 1024)))
    });
}

criterion_group!(benches, benchmark_format_size);
criterion_main!(benches);
```

## Error Handling

### Current Approach

- File system errors are handled gracefully
- Permission errors result in skipped files, not crashes
- Invalid paths are reported to stderr

### Best Practices

- Use `Result<T, E>` for operations that can fail
- Provide meaningful error messages
- Log errors appropriately (consider using `log` crate)
- Don't panic on recoverable errors

## Code Style

### Rust Conventions

- Follow standard Rust naming conventions
- Use `cargo fmt` for consistent formatting
- Use `cargo clippy` for additional linting
- Write comprehensive documentation with examples

### Documentation

- All public functions must have documentation
- Include examples in documentation where helpful
- Use `//!` for module-level documentation
- Use `///` for function/struct documentation

## Contributing Guidelines

1. **Fork and Clone**: Fork the repository and clone your fork
2. **Create Branch**: Create a feature branch for your changes
3. **Write Tests**: Add tests for new functionality
4. **Run Tests**: Ensure all tests pass with `cargo test`
5. **Check Linting**: Run `cargo clippy` and fix any warnings
6. **Format Code**: Run `cargo fmt` to format your code
7. **Update Documentation**: Update relevant documentation
8. **Submit PR**: Submit a pull request with a clear description

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md` with new features and fixes
3. Create git tag: `git tag v0.x.x`
4. Push tag: `git push origin v0.x.x`
5. Publish to crates.io: `cargo publish`
6. Create GitHub release with release notes