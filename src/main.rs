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
//! # Tree view of directory structure
//! fls -t
//!
//! # Tree view with hidden files
//! fls -ta
//!
//! # Interactive tree view
//! fls -ti
//!
//! # Tree view with depth limit
//! fls -t -L 2
//!
//! # Tree view with depth limit and hidden files
//! fls -ta --depth 3
//!
//! # All options combined
//! fls -lai /path/to/directory
//! ```

mod colors;
mod config;
mod display;
mod file_info;
mod formatting;

use clap::Parser;
use config::Config;

#[derive(Parser)]
#[command(name = "fls")]
#[command(version)]
#[command(about = "Enhanced ls command with detailed permissions, table display, and tree view")]
struct Args {
    /// Directory path to list
    #[arg(default_value = ".")]
    path: String,

    /// Show hidden files
    #[arg(short = 'a', long = "all")]
    all: bool,

    /// Show detailed information in table format
    #[arg(short = 'l', long = "long")]
    long: bool,

    /// Show clickable file names (requires terminal with OSC 8 support)
    #[arg(short = 'i', long = "interactive")]
    interactive: bool,

    /// Display files in a tree-like structure
    #[arg(short = 't', long = "tree")]
    tree: bool,

    /// Limit tree depth to specified number of levels (like tree -L)
    #[arg(short = 'L', long = "depth", value_name = "DEPTH", value_parser = clap::value_parser!(u8).range(1..=50))]
    depth: Option<u8>,
}

fn main() {
    let args = Args::parse();

    let config = Config {
        path: args.path,
        long_format: args.long,
        show_hidden: args.all,
        interactive: args.interactive,
        tree: args.tree,
        tree_depth: args.depth.map(|d| d as usize),
    };

    display::list_directory(&config);
}
