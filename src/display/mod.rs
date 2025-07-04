//! Display modules for different output formats.
//!
//! This module provides the main entry point for displaying directory contents
//! and delegates to specific formatters based on the configuration.

pub mod simple;
pub mod table;
pub mod tree;

use std::fs;
use colored::*;

use crate::config::Config;

/// Lists directory contents according to the provided configuration.
///
/// This is the main entry point for directory listing. It reads the directory,
/// sorts entries alphabetically, and delegates to the appropriate display module
/// based on whether long format is requested.
///
/// # Arguments
///
/// * `config` - Configuration specifying path, format, and options
///
/// # Errors
///
/// Prints an error message to stderr if the directory cannot be read.
pub fn list_directory(config: &Config) {
    let dir = match fs::read_dir(&config.path) {
        Ok(dir) => dir,
        Err(e) => {
            eprintln!("{}: {}", "Error".red().bold(), e);
            return;
        }
    };

    let mut entries: Vec<_> = dir.collect();
    entries.sort_by(|a, b| {
        let a_name = a.as_ref().unwrap().file_name();
        let b_name = b.as_ref().unwrap().file_name();
        a_name.cmp(&b_name)
    });

    if config.tree {
        tree::display(&entries, config);
    } else if config.long_format {
        table::display(&entries, config);
    } else {
        simple::display(&entries, config);
    }
}