//! Tree display module for hierarchical file listing.
//!
//! This module provides tree-like directory listing functionality, similar to the Unix `tree` command.
//! It shows files and directories in a hierarchical structure with visual tree branches.

use colored::*;
use std::fs::{self, DirEntry};
use std::io::Result as IoResult;
use std::path::Path;

use crate::colors::format_with_color;
use crate::config::Config;
use crate::file_info::FileInfo;

/// Tree drawing characters for different positions
const TREE_BRANCH: &str = "├── ";
const TREE_LAST: &str = "└── ";
const TREE_VERTICAL: &str = "│   ";
const TREE_SPACE: &str = "    ";

/// Maximum depth to prevent infinite recursion
const MAX_DEPTH: usize = 20;

/// Displays directory contents in a tree-like structure.
///
/// This function recursively traverses directories and displays them with visual tree branches.
/// It respects the configuration options for showing hidden files and interactive mode.
///
/// # Arguments
///
/// * `entries` - Vector of directory entries to display
/// * `config` - Configuration specifying display options
pub fn display(_entries: &[IoResult<DirEntry>], config: &Config) {
    let path = Path::new(&config.path);

    // Display the root directory name
    println!("{}", path.display().to_string().bright_blue().bold());

    // Start tree traversal from the root
    if let Ok(entries) = fs::read_dir(path) {
        let mut valid_entries: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|entry| {
                config.show_hidden || !entry.file_name().to_string_lossy().starts_with('.')
            })
            .collect();

        // Sort entries alphabetically
        valid_entries.sort_by(|a, b| {
            let a_name = a.file_name();
            let b_name = b.file_name();
            a_name.cmp(&b_name)
        });

        display_tree_recursive(&valid_entries, "", true, config, 0);
    }
}

/// Recursively displays directory contents in tree format.
///
/// # Arguments
///
/// * `entries` - Vector of directory entries to display
/// * `prefix` - Current indentation prefix for tree structure
/// * `is_root` - Whether this is the root level
/// * `config` - Configuration specifying display options
/// * `depth` - Current recursion depth
fn display_tree_recursive(
    entries: &[DirEntry],
    prefix: &str,
    _is_root: bool,
    config: &Config,
    depth: usize,
) {
    // Check user-specified depth limit first, then absolute maximum
    let max_allowed_depth = config.tree_depth.unwrap_or(MAX_DEPTH);
    if depth >= max_allowed_depth || depth > MAX_DEPTH {
        return;
    }

    let total_entries = entries.len();

    for (index, entry) in entries.iter().enumerate() {
        let is_last = index == total_entries - 1;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip hidden files unless explicitly requested
        if !config.show_hidden && file_name_str.starts_with('.') {
            continue;
        }

        // Determine tree symbols
        let tree_symbol = if is_last { TREE_LAST } else { TREE_BRANCH };
        let next_prefix = if is_last { TREE_SPACE } else { TREE_VERTICAL };

        // Get file info for coloring
        if let Ok(file_info) = FileInfo::from_path(entry.path()) {
            let display_name = format_file_name(&file_name_str, &file_info, config);
            println!("{}{}{}", prefix, tree_symbol, display_name);

            // Recursively display subdirectories
            if file_info.is_directory() {
                if let Ok(sub_entries) = fs::read_dir(entry.path()) {
                    let mut sub_entries: Vec<_> = sub_entries
                        .filter_map(|e| e.ok())
                        .filter(|entry| {
                            config.show_hidden
                                || !entry.file_name().to_string_lossy().starts_with('.')
                        })
                        .collect();

                    // Sort sub-entries alphabetically
                    sub_entries.sort_by(|a, b| {
                        let a_name = a.file_name();
                        let b_name = b.file_name();
                        a_name.cmp(&b_name)
                    });

                    if !sub_entries.is_empty() {
                        let new_prefix = format!("{}{}", prefix, next_prefix);
                        display_tree_recursive(&sub_entries, &new_prefix, false, config, depth + 1);
                    }
                }
            }
        } else {
            // Handle cases where file info can't be retrieved
            let display_name = format_file_name(&file_name_str, &FileInfo::default(), config);
            println!("{}{}{}", prefix, tree_symbol, display_name);
        }
    }
}

/// Formats a file name with appropriate colors and interactive features.
///
/// # Arguments
///
/// * `name` - The file name to format
/// * `file_info` - File information for determining colors
/// * `config` - Configuration for interactive mode
///
/// # Returns
///
/// A formatted string with colors and optional hyperlinks
fn format_file_name(name: &str, file_info: &FileInfo, config: &Config) -> String {
    format_with_color(name, file_info, config.interactive)
}

