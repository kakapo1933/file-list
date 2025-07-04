# File List (`fls`)

An enhanced `ls` command-line tool written in Rust that provides human-readable file information with comprehensive permission details, visual formatting, and hierarchical tree view.

## Features

### 🎨 **Visual Enhancements**
- **Color-coded output** for better visual distinction:
  - 🔸 Hidden files appear dimmed/gray
  - 🔵 Directories appear in blue and bold
  - 🟢 Executable files appear in green and bold
  - ⚫ Regular files appear in normal color
- **Color-coded file sizes** for quick size assessment:
  - 🟢 Green: < 1MB (small files)
  - 🟡 Yellow: 1MB - 100MB (medium files)  
  - 🟣 Magenta: 100MB - 1GB (large files)
  - 🔴 Red (bold): > 1GB (very large files)
- **Professional table formatting** with Unicode borders
- **Perfect column alignment** regardless of filename length or special characters
- **Hierarchical tree view** with Unicode tree drawing characters (├──, └──, │)

### 🖱️ **Interactive Features**
- **Clickable file names** with OSC 8 terminal hyperlink support
- **One-click file opening** with system default applications
- **One-click folder navigation** in file manager
- **Cross-platform compatibility** (macOS, Linux)
- **Terminal support detection** for modern terminals (iTerm2, GNOME Terminal, Windows Terminal, VS Code)

### 📊 **Human-Readable Permissions**
- **Separate permission columns** for User, Group, and Other
- **Plain English descriptions**: "Read, Write, Execute" instead of "rwx"
- **File type identification**: Directory, File, Executable, Symlink
- **Clear ownership display**: "username/groupname (Owner)"
- **Octal notation** for technical reference

### 🔧 **Enhanced Information Display**
- **Human-readable file sizes** (B, K, M, G format)
- **Formatted timestamps** for modification dates
- **Real user and group names** (not just IDs)
- **Comprehensive file type detection**

### 🌲 **Tree View Features**
- **Hierarchical directory visualization** similar to Unix `tree` command
- **Unicode tree drawing characters** for clear structural representation
- **Recursive subdirectory traversal** with depth limiting for safety
- **Color-coded file types** in tree structure (directories, executables, hidden files)
- **Interactive tree mode** with clickable file and directory links
- **Hidden file support** in tree view with `-ta` flag combination
- **Graceful error handling** for permission-denied directories

### 🚀 **Robust Edge Case Handling**
- **Complex filenames**: Spaces, Unicode, special characters, very long names
- **All file types**: Regular files, directories, executables, symlinks, hidden files
- **Permission combinations**: From read-only to complex permission sets
- **Error handling**: Graceful handling of permission errors and invalid paths

## Installation

```bash
# Clone or navigate to the project directory
cd file-list

# Build the project
cargo build --release

# Install globally (optional)
cargo install --path .
```

## Usage

### Basic Commands

```bash
# Basic listing (current directory)
fls

# List specific directory
fls /path/to/directory

# Detailed table format with human-readable permissions
fls -l

# Show hidden files
fls -a

# Combine options (detailed view + hidden files)
fls -la /path/to/directory

# Interactive mode (clickable file names)
fls -i

# Interactive mode with table format
fls -li

# Tree view of directory structure
fls -t

# Tree view with hidden files
fls -ta

# Interactive tree view with clickable files
fls -ti

# All options combined
fls -lai /path/to/directory
```

### Command Line Options

| Option | Short | Long | Description |
|--------|-------|------|-------------|
| `-l` | `-l` | `--long` | Display detailed information in table format with human-readable permissions |
| `-a` | `-a` | `--all` | Show hidden files (files starting with `.`) |
| `-i` | `-i` | `--interactive` | Enable clickable file names (requires terminal with OSC 8 support) |
| `-t` | `-t` | `--tree` | Display files in a tree-like hierarchical structure |

## Examples

### Basic Output (Simple Format)
```
.gitignore
Cargo.lock
Cargo.toml
src
target
```

### Enhanced Table Format (`-l`)
```
┌──────────────┬────────────┬──────────────────────┬──────────────────┬──────────────────┬───────┬────────────────────┬────────┬──────────────┐
│ Name         │ Type       │ User Permission      │ Group Permission │ Other Permission │ Octal │ User/Group (Owner) │ Size   │ Modified     │
├──────────────┼────────────┼──────────────────────┼──────────────────┼──────────────────┼───────┼────────────────────┼────────┼──────────────┤
│ .gitignore   │ File       │ Read, Write          │ Read             │ Read             │ 644   │ user/staff         │ 8B     │ Jun 06 01:40 │
├──────────────┼────────────┼──────────────────────┼──────────────────┼──────────────────┼───────┼────────────────────┼────────┼──────────────┤
│ Cargo.lock   │ File       │ Read, Write          │ Read             │ Read             │ 644   │ user/staff         │ 16.1K  │ Jun 08 22:28 │
├──────────────┼────────────┼──────────────────────┼──────────────────┼──────────────────┼───────┼────────────────────┼────────┼──────────────┤
│ src          │ Directory  │ Read, Write, Execute │ Read, Execute    │ Read, Execute    │ 755   │ user/staff         │ 96B    │ Jun 09 02:29 │
└──────────────┴────────────┴──────────────────────┴──────────────────┴──────────────────┴───────┴────────────────────┴────────┴──────────────┘
```

Note: File sizes are color-coded in the terminal output - green for small files (<1MB), yellow for medium (1MB-100MB), magenta for large (100MB-1GB), and red for very large (>1GB).

### Tree View Format (`-t`)
```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── examples
│   ├── custom_colors.rs
│   ├── json_output.rs
│   └── plugin_system.rs
├── src
│   ├── colors.rs
│   ├── config.rs
│   ├── display
│   │   ├── mod.rs
│   │   ├── simple.rs
│   │   ├── table.rs
│   │   └── tree.rs
│   ├── file_info.rs
│   ├── formatting.rs
│   └── main.rs
└── target
    └── release
        └── fls
```

### Tree View with Hidden Files (`-ta`)
```
project
├── .gitignore
├── .cargo
│   └── config.toml
├── src
│   ├── .hidden_temp
│   ├── main.rs
│   └── lib.rs
└── README.md
```

### Interactive Tree View (`-ti`)
The tree view supports interactive mode where file and directory names become clickable hyperlinks that open with your system's default applications.

### Permission Examples Explained

| Permission Display | Meaning |
|-------------------|---------|
| `Read, Write, Execute` | Full access - can view, modify, and run |
| `Read, Execute` | Can view and enter (for directories) or run (for files) |
| `Read, Write` | Can view and modify, but not execute |
| `Read` | Read-only access |
| `None` | No access permissions |

## Technical Details

### Dependencies
- **clap**: Command-line argument parsing with derive macros
- **colored**: Terminal color output and text styling
- **chrono**: Date and time formatting for file timestamps
- **tabled**: Professional table formatting and display
- **users**: User and group name resolution from system IDs
- **open**: Cross-platform file and directory opening
- **percent-encoding**: URL encoding for file paths in hyperlinks

### Architecture
- **Modular design**: Clean separation of concerns across multiple modules
- **Configuration management**: Type-safe configuration struct replacing multiple parameters
- **Display abstraction**: Separate modules for simple and table formatting
- **Color management**: Centralized color logic with terminal hyperlink support
- **File operations**: Dedicated module for file metadata and permission handling
- **Formatting utilities**: Reusable functions for size, time, and permission formatting
- **Color-safe table rendering**: Colors applied after table layout calculation
- **Robust error handling**: Graceful degradation for permission errors
- **Cross-platform compatibility**: Works on macOS and Linux systems

#### Module Structure
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

### Performance
- **Efficient sorting**: Files sorted alphabetically for consistent output
- **Optimized table rendering**: Single-pass color application
- **Memory efficient**: Minimal allocations for large directories

## Advanced Features

### File Type Detection
- **Directories**: Identified by metadata and shown in blue
- **Executables**: Detected by permission bits and shown in green
- **Hidden files**: Files starting with '.' shown dimmed
- **Symlinks**: Properly identified with 'l' in traditional format

### Permission Analysis
- **Octal representation**: Shows numeric permission format (e.g., 755, 644)
- **Special permissions**: Handles setuid, setgid, and sticky bits
- **Real ownership**: Displays actual usernames and group names
- **Clear categorization**: Separate columns for user, group, and other permissions

### Interactive Features
- **OSC 8 hyperlinks**: Uses standard terminal escape sequences for clickable links
- **File URL generation**: Creates proper `file://` URLs with percent-encoding for special characters
- **Absolute path resolution**: Handles both relative and absolute paths correctly
- **Cross-platform opening**: Uses system default applications for file/folder opening
- **Terminal compatibility**: Works with modern terminals supporting OSC 8 sequences

#### Supported Terminals
- **iTerm2** (macOS) - Full support
- **GNOME Terminal** (Linux) - Full support  
- **Windows Terminal** - Full support
- **VS Code integrated terminal** - Full support
- **Terminal.app** (macOS) - Limited support
- **Other terminals** - Graceful fallback (hyperlinks ignored, colors preserved)

## Comparison with Traditional `ls` and `tree`

| Feature | `ls -la` | `tree` | `fls -la` | `fls -t` | `fls -tai` |
|---------|----------|--------|-----------|----------|------------|
| Permission format | `rwxr-xr-x` | ❌ None | `Read, Write, Execute` | ❌ None | ❌ None |
| File type | First character | Basic | Dedicated "Type" column | Color-coded | Color-coded |
| Ownership | `user group` | ❌ None | `user/group (Owner)` | ❌ None | ❌ None |
| Visual layout | Plain text | Tree structure | Professional table | Tree structure | Tree structure |
| Color coding | Basic | Basic | Enhanced with alignment | Enhanced colors | Enhanced colors |
| Interactive files | ❌ None | ❌ None | ❌ None | ❌ None | ✅ Clickable hyperlinks |
| File opening | Manual copy/paste | Manual copy/paste | Manual copy/paste | Manual copy/paste | ✅ One-click opening |
| Hierarchical view | ❌ None | ✅ Tree only | ❌ None | ✅ Tree with colors | ✅ Tree with colors |
| Hidden files | With `-a` | With `-a` | With `-a` | With `-a` | With `-a` |
| Learning curve | Requires Unix knowledge | Simple | Self-explanatory | Intuitive | Intuitive |

## License

This project is available under the MIT License.

## Extending the Tool

The modular architecture makes it easy to extend `fls` with new features. See the `examples/` directory for detailed examples:

- **`json_output.rs`**: Adding JSON output format
- **`custom_colors.rs`**: Implementing configurable color schemes  
- **`plugin_system.rs`**: Creating a plugin system for custom file information

For detailed development information, see [`DEVELOPER.md`](DEVELOPER.md).

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests to improve the functionality or documentation.