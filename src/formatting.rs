//! Formatting utilities for file information display.
//!
//! This module provides functions for formatting various file attributes
//! into human-readable strings, including file sizes, timestamps, and
//! permission values.

use chrono::{DateTime, Local};
use std::fs;
use std::os::unix::fs::PermissionsExt;

/// Formats a file size in bytes into a human-readable string.
///
/// Uses standard binary prefixes (1024-based) and includes one decimal place
/// for sizes larger than 1KB.
///
/// # Arguments
///
/// * `size` - The file size in bytes
///
/// # Returns
///
/// A formatted string like "1.5K", "2.3M", "1.2G", or "256B"
///
/// # Examples
///
/// ```
/// let size = format_size(1536);
/// assert_eq!(size, "1.5K");
/// ```
pub fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{}B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1}K", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1}M", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}G", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

/// Formats the modification time from file metadata into a readable string.
///
/// # Arguments
///
/// * `metadata` - The file's metadata
///
/// # Returns
///
/// A formatted timestamp string like "Jun 08 14:30" or "Unknown" if unavailable
pub fn format_time(metadata: &fs::Metadata) -> String {
    match metadata.modified() {
        Ok(time) => {
            let datetime: DateTime<Local> = time.into();
            datetime.format("%b %d %H:%M").to_string()
        }
        Err(_) => "Unknown".to_string(),
    }
}

/// Formats file permissions as an octal string.
///
/// # Arguments
///
/// * `metadata` - The file's metadata
///
/// # Returns
///
/// An octal permission string like "755", "644", etc.
pub fn format_octal_permissions(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    format!("{:o}", mode & 0o7777)
}