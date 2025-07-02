//! Simple format display implementation.
//!
//! This module provides the simple (non-table) display format that shows
//! file names in a vertical list, similar to basic `ls` output but with
//! colors and optional interactive features.

use std::fs;
use std::path::Path;

use crate::colors::{get_colored_name, make_clickable_link};
use crate::config::Config;

/// Displays directory entries in simple format (one file per line).
///
/// This function outputs file names in a simple vertical list with color coding
/// based on file type. If interactive mode is enabled, file names become
/// clickable hyperlinks.
///
/// # Arguments
///
/// * `entries` - Iterator over directory entries
/// * `config` - Configuration specifying display options
///
/// # Features
///
/// - Color-coded file names based on type
/// - Optional clickable hyperlinks in interactive mode
/// - Hidden file filtering based on configuration
/// - Graceful error handling for unreadable files
pub fn display(entries: &[Result<fs::DirEntry, std::io::Error>], config: &Config) {
    for entry in entries {
        let Ok(entry) = entry else { continue };

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !config.show_hidden && file_name_str.starts_with('.') {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => {
                println!("{}", file_name_str);
                continue;
            }
        };

        let colored_name = get_colored_name(&file_name_str, &metadata);
        
        if config.interactive {
            let full_path = Path::new(&config.path).join(&file_name);
            let clickable_name = make_clickable_link(&file_name_str, &full_path, &colored_name);
            println!("{}", clickable_name);
        } else {
            println!("{}", colored_name);
        }
    }
}