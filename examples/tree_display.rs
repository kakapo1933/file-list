//! Example: Tree Display Features
//!
//! This example demonstrates how to use and extend the tree display functionality.

use std::process::Command;

/// Example: Basic tree display with various options
fn demo_tree_display() {
    println!("=== Tree Display Examples ===\n");
    
    // Basic tree view
    println!("1. Basic tree view:");
    println!("   fls -t\n");
    
    // Tree with depth limit
    println!("2. Tree with depth limit (2 levels):");
    println!("   fls -t -L 2\n");
    
    // Tree with hidden files
    println!("3. Tree including hidden files:");
    println!("   fls -ta\n");
    
    // Interactive tree
    println!("4. Interactive tree with clickable files:");
    println!("   fls -ti\n");
    
    // Combined options
    println!("5. Tree with depth limit and hidden files:");
    println!("   fls -ta --depth 3\n");
}

/// Example: Custom tree symbols
fn custom_tree_symbols() {
    println!("=== Custom Tree Symbols ===\n");
    
    // Alternative tree drawing characters
    const ALT_BRANCH: &str = "+-";
    const ALT_LAST: &str = "`-";
    const ALT_VERTICAL: &str = "| ";
    const ALT_SPACE: &str = "  ";
    
    println!("ASCII-style tree (for terminals without Unicode):");
    println!(".");
    println!("+-Cargo.toml");
    println!("+-src");
    println!("| +-main.rs");
    println!("| `-lib.rs");
    println!("`-README.md\n");
}

/// Example: Programmatic depth control
fn depth_control_example() {
    println!("=== Depth Control Examples ===\n");
    
    // Different depth levels
    let depths = vec![1, 2, 3, 5];
    
    for depth in depths {
        println!("Tree with depth {}:", depth);
        let output = Command::new("fls")
            .args(&["-t", "-L", &depth.to_string()])
            .output()
            .expect("Failed to execute command");
        
        // Process output...
        println!("(Output would show {} levels deep)\n", depth);
    }
}

/// Example: Tree traversal logic
fn tree_traversal_pattern() {
    println!("=== Tree Traversal Pattern ===\n");
    
    println!("The tree display uses depth-first traversal:");
    println!("1. Process current directory");
    println!("2. For each entry:");
    println!("   a. Display with appropriate tree symbol");
    println!("   b. If directory and depth allows:");
    println!("      - Recursively process subdirectory");
    println!("   c. Update prefix for next entries\n");
}

/// Example: Integration with other flags
fn flag_combinations() {
    println!("=== Flag Combinations ===\n");
    
    let examples = vec![
        ("-t", "Basic tree view"),
        ("-ta", "Tree with hidden files"),
        ("-ti", "Interactive tree"),
        ("-t -L 2", "Tree with depth limit"),
        ("-tai", "Tree with all features"),
        ("-ta --depth 3", "Tree with hidden files, depth 3"),
    ];
    
    for (flags, description) in examples {
        println!("fls {} - {}", flags, description);
    }
}

fn main() {
    println!("File List Tree Display Examples\n");
    
    demo_tree_display();
    custom_tree_symbols();
    depth_control_example();
    tree_traversal_pattern();
    flag_combinations();
    
    println!("\nFor implementation details, see src/display/tree.rs");
}