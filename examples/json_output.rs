//! Example: Adding JSON output format
//!
//! This example shows how to extend the file listing tool to support JSON output.
//! Run with: cargo run --example json_output
//!
//! Note: This is a demonstration example that shows the structure without requiring
//! external dependencies. To implement actual JSON output, you would add:
//! - serde = { version = "1.0", features = ["derive"] }
//! - serde_json = "1.0"
//! to your Cargo.toml dependencies.

/// Example of how to add JSON output to the existing codebase
fn main() {
    println!("This example shows how to add JSON output format");

    // Step 1: Add dependencies to Cargo.toml
    println!("1. Add to Cargo.toml dependencies:");
    println!("   serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("   serde_json = \"1.0\"");

    // Step 2: Add a new field to Config struct
    println!("\n2. Add 'json_output: bool' to Config struct in config.rs");

    // Step 3: Add CLI argument
    println!("\n3. Add CLI argument in main.rs:");
    println!("   .arg(Arg::new(\"json\").short('j').long(\"json\"))");

    // Step 4: Create new display module
    println!("\n4. Create src/display/json.rs with this structure:");

    // Example JSON structure (as a string since we don't have serde_json)
    let example_output = r#"{
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
}"#;

    println!("Example JSON output:");
    println!("{}", example_output);

    // Step 5: Integration point
    println!("\n5. Add to display/mod.rs:");
    println!("   if config.json_output {{ json::display(&entries, config); }}");

    // Step 6: Implementation example
    println!("\n6. Example implementation in src/display/json.rs:");
    println!("   use serde_json::{{json, Value}};");
    println!("   use crate::{{config::Config, file_info::FileInfo}};");
    println!("   ");
    println!(
        "   pub fn display(entries: &[Result<fs::DirEntry, std::io::Error>], config: &Config) {{"
    );
    println!("       let files: Vec<Value> = entries.iter()");
    println!("           .filter_map(|entry| entry.as_ref().ok())");
    println!("           .map(|entry| {{");
    println!("               // Convert FileInfo to JSON");
    println!("               json!({{");
    println!("                   \"name\": entry.file_name(),");
    println!("                   \"type\": get_file_type(&metadata),");
    println!("                   // ... other fields");
    println!("               }})");
    println!("           }})");
    println!("           .collect();");
    println!("       ");
    println!("       let output = json!({{ \"files\": files }});");
    println!("       println!(\"{{}}\", serde_json::to_string_pretty(&output).unwrap());");
    println!("   }}");
}

