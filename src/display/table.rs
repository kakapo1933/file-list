//! Table format display implementation.
//!
//! This module provides the detailed table display format that shows comprehensive
//! file information including permissions, ownership, size, and modification time.
//! It handles color application after table generation to maintain proper alignment.

use std::fs;
use std::path::Path;
use tabled::{settings::Style, Table};

use crate::colors::{get_colored_name, get_colored_size, make_clickable_link};
use crate::config::Config;
use crate::file_info::FileInfo;
use crate::formatting::format_size;

/// Displays directory entries in detailed table format.
///
/// This function creates a professional table with columns for file name, type,
/// permissions, ownership, size, and modification time. Colors and hyperlinks
/// are applied after table generation to maintain proper column alignment.
///
/// # Arguments
///
/// * `entries` - Iterator over directory entries
/// * `config` - Configuration specifying display options
///
/// # Features
///
/// - Professional table formatting with Unicode borders
/// - Human-readable permission descriptions
/// - Color-coded file names and sizes
/// - Optional clickable hyperlinks in interactive mode
/// - Hidden file filtering based on configuration
/// - Proper column alignment regardless of color codes
pub fn display(entries: &[Result<fs::DirEntry, std::io::Error>], config: &Config) {
    let mut file_infos = Vec::new();

    for entry in entries {
        let Ok(entry) = entry else { continue };

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !config.show_hidden && file_name_str.starts_with('.') {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        let file_info = FileInfo::from_metadata(file_name_str.to_string(), &metadata);
        file_infos.push(file_info);
    }

    if !file_infos.is_empty() {
        let table = Table::new(file_infos).with(Style::modern()).to_string();

        // Apply colors after table is formatted
        let colored_output = apply_colors_to_table(&table, entries, config);
        println!("{}", colored_output);
    }
}

fn apply_colors_to_table(
    table: &str,
    entries: &[Result<fs::DirEntry, std::io::Error>],
    config: &Config,
) -> String {
    let mut result = table.to_string();

    // Collect all file names and sizes, sort by length (longest first) to avoid partial replacements
    let mut file_entries = Vec::new();
    let mut size_entries = Vec::new();

    for entry in entries {
        let Ok(entry) = entry else { continue };
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !config.show_hidden && file_name_str.starts_with('.') {
            continue;
        }

        if let Ok(metadata) = entry.metadata() {
            let colored_name = get_colored_name(&file_name_str, &metadata);
            if config.interactive {
                let full_path = Path::new(&config.path).join(&file_name);
                let clickable_name = make_clickable_link(&file_name_str, &full_path, &colored_name);
                file_entries.push((file_name_str.to_string(), clickable_name));
            } else {
                file_entries.push((file_name_str.to_string(), colored_name));
            }

            // Also collect size information for coloring
            let size = metadata.len();
            let size_str = format_size(size);
            let colored_size = get_colored_size(&size_str, size);
            size_entries.push((size_str, colored_size));
        }
    }

    // Sort by filename length (longest first) to avoid partial matches
    file_entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));
    size_entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    // Apply replacements
    result = apply_file_name_colors(result, file_entries);
    result = apply_size_colors(result, size_entries);

    result
}

fn apply_file_name_colors(mut result: String, file_entries: Vec<(String, String)>) -> String {
    for (file_name, colored_name) in file_entries {
        let lines: Vec<&str> = result.split('\n').collect();
        let mut new_lines = Vec::new();

        for line in lines {
            // Only replace if it's the actual filename in the first column with exact boundary
            let filename_pattern = format!("│ {} ", file_name);
            if line.contains(&filename_pattern) {
                let new_line = line.replace(&filename_pattern, &format!("│ {} ", colored_name));
                new_lines.push(new_line);
            } else {
                new_lines.push(line.to_string());
            }
        }

        result = new_lines.join("\n");
    }
    result
}

fn apply_size_colors(mut result: String, size_entries: Vec<(String, String)>) -> String {
    for (size_str, colored_size) in size_entries {
        let lines: Vec<&str> = result.split('\n').collect();
        let mut new_lines = Vec::new();

        for line in lines {
            if line.contains(&size_str) {
                // Replace size ensuring we don't replace partial matches
                let size_pattern = format!(" {} ", size_str);
                let colored_pattern = format!(" {} ", colored_size);
                if line.contains(&size_pattern) {
                    let new_line = line.replace(&size_pattern, &colored_pattern);
                    new_lines.push(new_line);
                } else {
                    // Check for size at end of cell (before │)
                    let size_pattern_end = format!(" {} │", size_str);
                    let colored_pattern_end = format!(" {} │", colored_size);
                    if line.contains(&size_pattern_end) {
                        let new_line = line.replace(&size_pattern_end, &colored_pattern_end);
                        new_lines.push(new_line);
                    } else {
                        new_lines.push(line.to_string());
                    }
                }
            } else {
                new_lines.push(line.to_string());
            }
        }

        result = new_lines.join("\n");
    }
    result
}