# File List (`fls`)

An enhanced `ls` command-line tool written in Rust that provides human-readable file information with comprehensive permission details and visual formatting.

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
```

### Command Line Options

| Option | Short | Long | Description |
|--------|-------|------|-------------|
| `-l` | `-l` | `--long` | Display detailed information in table format with human-readable permissions |
| `-a` | `-a` | `--all` | Show hidden files (files starting with `.`) |

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

### Architecture
- **Modular design**: Separate functions for permissions, formatting, and display
- **Color-safe table rendering**: Colors applied after table layout calculation
- **Robust error handling**: Graceful degradation for permission errors
- **Cross-platform compatibility**: Works on macOS and Linux systems

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

## Comparison with Traditional `ls`

| Feature | `ls -la` | `fls -la` |
|---------|----------|-------------------|
| Permission format | `rwxr-xr-x` | `Read, Write, Execute` |
| File type | First character | Dedicated "Type" column |
| Ownership | `user group` | `user/group (Owner)` |
| Visual layout | Plain text | Professional table |
| Color coding | Basic | Enhanced with proper alignment |
| Learning curve | Requires Unix knowledge | Self-explanatory |

## License

This project is available under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests to improve the functionality or documentation.