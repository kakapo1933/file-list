//! Color and terminal hyperlink utilities.
//!
//! This module handles all color formatting for file names and sizes, as well as
//! generating terminal hyperlinks for interactive mode. It provides consistent
//! color schemes based on file types and sizes.

use colored::*;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use std::fs;
use std::path::Path;

use crate::file_info::is_executable;

/// Applies color formatting to a file name based on its type and attributes.
///
/// # Arguments
///
/// * `file_name` - The name of the file
/// * `metadata` - The file's metadata
///
/// # Returns
///
/// A colored string representation of the file name
///
/// # Color Scheme
///
/// - Hidden files (starting with '.'): Dimmed/gray
/// - Directories: Blue and bold
/// - Executable files: Green and bold
/// - Regular files: Normal color
pub fn get_colored_name(file_name: &str, metadata: &fs::Metadata) -> String {
    if file_name.starts_with('.') {
        format!("{}", file_name.bright_black())
    } else if metadata.is_dir() {
        format!("{}", file_name.blue().bold())
    } else if is_executable(metadata) {
        format!("{}", file_name.green().bold())
    } else {
        file_name.to_string()
    }
}

/// Applies color coding to file size strings based on the actual size in bytes.
///
/// # Arguments
///
/// * `size_str` - The formatted size string (e.g., "1.5K")
/// * `size_bytes` - The actual size in bytes for comparison
///
/// # Returns
///
/// A colored version of the size string
///
/// # Color Scheme
///
/// - Green: < 1MB (small files)
/// - Yellow: 1MB - 100MB (medium files)
/// - Magenta: 100MB - 1GB (large files)
/// - Red (bold): > 1GB (very large files)
pub fn get_colored_size(size_str: &str, size_bytes: u64) -> String {
    // Color coding for file sizes:
    // Green: < 1MB (small files)
    // Yellow: 1MB - 100MB (medium files)
    // Magenta: > 100MB (large files)
    // Red: > 1GB (very large files)
    if size_bytes >= 1024 * 1024 * 1024 {
        // >= 1GB - Red
        format!("{}", size_str.red().bold())
    } else if size_bytes >= 100 * 1024 * 1024 {
        // >= 100MB - Magenta
        format!("{}", size_str.magenta())
    } else if size_bytes >= 1024 * 1024 {
        // >= 1MB - Yellow
        format!("{}", size_str.yellow())
    } else {
        // < 1MB - Green
        format!("{}", size_str.green())
    }
}

/// Creates a clickable terminal hyperlink using OSC 8 escape sequences.
///
/// This function generates terminal hyperlinks that work in modern terminals
/// supporting OSC 8 sequences. When clicked, the link will open the file or
/// directory with the system's default application.
///
/// # Arguments
///
/// * `_file_name` - The file name (currently unused but kept for future use)
/// * `full_path` - The full path to the file or directory
/// * `colored_name` - The colored display text for the link
///
/// # Returns
///
/// A string containing OSC 8 escape sequences that create a clickable hyperlink
///
/// # Terminal Support
///
/// - iTerm2, GNOME Terminal, Windows Terminal: Full support
/// - VS Code terminal: Full support
/// - Other terminals: Graceful fallback (sequences ignored)
pub fn make_clickable_link(_file_name: &str, full_path: &Path, colored_name: &str) -> String {
    // Convert path to absolute path if needed
    let absolute_path = if full_path.is_absolute() {
        full_path.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_default()
            .join(full_path)
    };
    
    // Create file:// URL with percent encoding for special characters
    let url_path = absolute_path.to_string_lossy();
    let encoded_path: String = url_path
        .chars()
        .map(|c| {
            if c == '/' || c == ':' {
                c.to_string()
            } else {
                percent_encode(c.to_string().as_bytes(), NON_ALPHANUMERIC).to_string()
            }
        })
        .collect();
    
    let file_url = format!("file://{}", encoded_path);
    
    // OSC 8 escape sequence: \x1b]8;;URL\x1b\\TEXT\x1b]8;;\x1b\\
    format!("\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\", file_url, colored_name)
}