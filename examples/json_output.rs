//! Example: Adding JSON output format
//!
//! This example shows how to extend the file listing tool to support JSON output.
//! Run with: cargo run --example json_output

use serde_json::json;
use std::fs;

// You would add this to your Cargo.toml dependencies:
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

/// Example of how to add JSON output to the existing codebase
fn main() {
    println!("This example shows how to add JSON output format");
    
    // Step 1: Add a new field to Config struct
    println!("1. Add 'json_output: bool' to Config struct in config.rs");
    
    // Step 2: Add CLI argument
    println!("2. Add CLI argument in main.rs:");
    println!("   .arg(Arg::new(\"json\").short('j').long(\"json\"))");
    
    // Step 3: Create new display module
    println!("3. Create src/display/json.rs with this structure:");
    
    let example_output = json!({
        "files": [
            {
                "name": "example.txt",
                "type": "File",
                "size_bytes": 1024,
                "size_human": "1.0K",
                "permissions": {
                    "user": ["Read", "Write"],
                    "group": ["Read"],
                    "other": ["Read"]
                },
                "owner": "user",
                "group": "staff",
                "modified": "2023-06-08T14:30:00Z"
            }
        ]
    });
    
    println!("Example JSON output:");
    println!("{}", serde_json::to_string_pretty(&example_output).unwrap());
    
    // Step 4: Integration point
    println!("\n4. Add to display/mod.rs:");
    println!("   if config.json_output {{ json::display(&entries, config); }}");
}