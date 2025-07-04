# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2024-01-07

### Added
- Tree depth control with `-L/--depth` flag (similar to Unix `tree -L`)
- Configurable depth limits from 1-50 levels
- Comprehensive input validation for depth values
- Tree display example file (`examples/tree_display.rs`)

### Changed
- Enhanced tree traversal to respect user-specified depth limits
- Updated documentation to include depth control examples
- Improved DEVELOPER.md with tree implementation details

### Technical Details
- Depth validation enforced at CLI parsing level using clap's value parser
- Default behavior remains unlimited depth (up to MAX_DEPTH safety limit)
- Tree symbols and color coding work seamlessly with depth limits

## [0.4.0] - 2024-01-07

### Added
- Hierarchical tree view with `-t/--tree` flag
- Unicode tree drawing characters (├──, └──, │)
- Recursive directory traversal with safety limits
- Tree view support for all existing flags:
  - Hidden files with `-ta`
  - Interactive mode with `-ti`
  - Color coding for file types

### Changed
- Enhanced help text to include tree functionality
- Updated module structure to include tree display

## [0.3.1] - 2024-01-06

### Fixed
- Version number correction in Cargo.toml

## [0.3.0] - 2024-01-06

### Added
- Interactive mode with clickable file names (`-i/--interactive`)
- OSC 8 terminal hyperlink support
- Cross-platform file/directory opening

### Changed
- Improved color coding for file sizes
- Enhanced documentation

## [0.2.0] - 2024-01-05

### Added
- Table format display with `-l/--long` flag
- Human-readable permissions
- Professional Unicode table borders
- Color-coded file sizes

### Changed
- Refactored to modular architecture
- Introduced Config struct for cleaner code

## [0.1.0] - 2024-01-04

### Added
- Basic file listing functionality
- Color-coded output for file types
- Hidden file support with `-a/--all` flag
- Initial project structure