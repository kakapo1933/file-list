//! Example: Adding custom color schemes
//!
//! This example demonstrates how to add configurable color schemes to the file listing tool.

use colored::*;

/// Example color scheme structure
#[derive(Clone)]
pub struct ColorScheme {
    pub hidden_files: fn(&str) -> ColoredString,
    pub directories: fn(&str) -> ColoredString,
    pub executables: fn(&str) -> ColoredString,
    pub regular_files: fn(&str) -> ColoredString,
}

/// Predefined color schemes
impl ColorScheme {
    /// Default color scheme (current implementation)
    pub fn default() -> Self {
        Self {
            hidden_files: |s| s.bright_black(),
            directories: |s| s.blue().bold(),
            executables: |s| s.green().bold(),
            regular_files: |s| s.normal(),
        }
    }
    
    /// High contrast color scheme for better accessibility
    pub fn high_contrast() -> Self {
        Self {
            hidden_files: |s| s.bright_black(),
            directories: |s| s.bright_blue().bold(),
            executables: |s| s.bright_green().bold(),
            regular_files: |s| s.bright_white(),
        }
    }
    
    /// Monochrome color scheme (no colors, just styles)
    pub fn monochrome() -> Self {
        Self {
            hidden_files: |s| s.dimmed(),
            directories: |s| s.bold(),
            executables: |s| s.underline(),
            regular_files: |s| s.normal(),
        }
    }
    
    /// Solarized color scheme
    pub fn solarized() -> Self {
        Self {
            hidden_files: |s| s.truecolor(88, 110, 117),      // base01
            directories: |s| s.truecolor(38, 139, 210).bold(), // blue
            executables: |s| s.truecolor(133, 153, 0).bold(),  // green
            regular_files: |s| s.truecolor(131, 148, 150),     // base0
        }
    }
}

/// Example of how to integrate custom colors into the existing codebase
fn main() {
    println!("Example: Adding custom color schemes\n");
    
    // Step 1: Extend Config struct
    println!("1. Add to Config struct:");
    println!("   pub color_scheme: String,");
    
    // Step 2: Add CLI argument
    println!("\n2. Add CLI argument:");
    println!("   .arg(Arg::new(\"colors\").long(\"colors\").value_name(\"SCHEME\"))");
    
    // Step 3: Update colors.rs
    println!("\n3. Update colors.rs to use ColorScheme:");
    println!("   pub fn get_colored_name_with_scheme(name: &str, metadata: &fs::Metadata, scheme: &ColorScheme) -> String");
    
    // Step 4: Demonstrate different schemes
    let file_name = "example.txt";
    
    println!("\n4. Example outputs with different schemes:");
    
    let schemes = [
        ("default", ColorScheme::default()),
        ("high-contrast", ColorScheme::high_contrast()),
        ("monochrome", ColorScheme::monochrome()),
        ("solarized", ColorScheme::solarized()),
    ];
    
    for (name, scheme) in &schemes {
        println!("   {}: {}", name, (scheme.regular_files)(file_name));
        println!("   {}: {}", name, (scheme.directories)("folder"));
        println!("   {}: {}", name, (scheme.executables)("script.sh"));
        println!("   {}: {}", name, (scheme.hidden_files)(".hidden"));
        println!();
    }
    
    println!("5. Integration in display modules:");
    println!("   let scheme = ColorScheme::from_name(&config.color_scheme);");
    println!("   let colored_name = get_colored_name_with_scheme(&file_name, &metadata, &scheme);");
}