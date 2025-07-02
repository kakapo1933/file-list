//! # File List (`fls`)
//!
//! An enhanced `ls` command-line tool written in Rust that provides human-readable file
//! information with comprehensive permission details, visual formatting, and optional
//! interactive features for clickable file names.
//!
//! ## Features
//!
//! - Color-coded output for different file types
//! - Human-readable permission descriptions
//! - Professional table formatting
//! - Interactive clickable file names (with `-i` flag)
//! - Cross-platform compatibility
//!
//! ## Usage
//!
//! ```bash
//! # Basic listing
//! fls
//!
//! # Detailed table format
//! fls -l
//!
//! # Show hidden files
//! fls -a
//!
//! # Interactive mode with clickable files
//! fls -i
//!
//! # All options combined
//! fls -lai /path/to/directory
//! ```

mod colors;
mod config;
mod display;
mod file_info;
mod formatting;

use clap::{Arg, Command};
use config::Config;

fn main() {
    let matches = Command::new("fls")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Enhanced ls command with detailed permissions and table display")
        .arg(
            Arg::new("path")
                .help("Directory path to list")
                .default_value(".")
                .index(1),
        )
        .arg(
            Arg::new("long")
                .short('l')
                .long("long")
                .help("Display detailed information in table format")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .help("Show hidden files")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .help("Enable clickable file names (requires terminal with OSC 8 support)")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let config = Config::from_matches(matches);
    display::list_directory(&config);
}