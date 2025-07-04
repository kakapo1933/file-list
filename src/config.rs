//! Configuration management for the file listing tool.
//!
//! This module provides a type-safe configuration structure that encapsulates all
//! command-line options and their values, replacing the previous approach of passing
//! multiple boolean parameters between functions.

/// Configuration structure that holds all command-line options and their values.
///
/// This struct provides a clean interface for passing configuration between modules
/// and ensures type safety for all options.
pub struct Config {
    /// The directory path to list (default: current directory)
    pub path: String,
    /// Whether to display detailed information in table format
    pub long_format: bool,
    /// Whether to show hidden files (files starting with '.')
    pub show_hidden: bool,
    /// Whether to enable clickable file names using terminal hyperlinks
    pub interactive: bool,
    /// Whether to display files in a tree-like structure
    pub tree: bool,
}

impl Config {
    /// Creates a new Config instance from parsed command-line arguments.
    ///
    /// # Arguments
    ///
    /// * `matches` - The parsed command-line arguments from clap
    ///
    /// # Returns
    ///
    /// A new Config instance with values extracted from the command-line arguments.
    pub fn from_matches(matches: clap::ArgMatches) -> Self {
        Self {
            path: matches.get_one::<String>("path").unwrap().clone(),
            long_format: matches.get_flag("long"),
            show_hidden: matches.get_flag("all"),
            interactive: matches.get_flag("interactive"),
            tree: matches.get_flag("tree"),
        }
    }
}