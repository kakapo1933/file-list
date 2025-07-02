//! Example: Plugin system for extending file information
//!
//! This example shows how to create a plugin system that allows adding
//! custom file information extractors.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Trait for file information plugins
pub trait FileInfoPlugin {
    /// Name of the plugin (used for column headers)
    fn name(&self) -> &'static str;
    
    /// Extract information from file metadata
    fn extract_info(&self, path: &Path, metadata: &fs::Metadata) -> String;
    
    /// Whether this plugin should be enabled by default
    fn default_enabled(&self) -> bool { false }
}

/// Example plugin: File extension extractor
pub struct ExtensionPlugin;

impl FileInfoPlugin for ExtensionPlugin {
    fn name(&self) -> &'static str {
        "Extension"
    }
    
    fn extract_info(&self, path: &Path, _metadata: &fs::Metadata) -> String {
        path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("None")
            .to_string()
    }
    
    fn default_enabled(&self) -> bool {
        true
    }
}

/// Example plugin: File hash calculator
pub struct HashPlugin;

impl FileInfoPlugin for HashPlugin {
    fn name(&self) -> &'static str {
        "SHA256"
    }
    
    fn extract_info(&self, path: &Path, metadata: &fs::Metadata) -> String {
        if metadata.is_file() && metadata.len() < 1024 * 1024 { // Only hash files < 1MB
            // In real implementation, you would use a crate like `sha2`
            format!("sha256:{:x}", path.to_string_lossy().len()) // Dummy hash
        } else {
            "N/A".to_string()
        }
    }
}

/// Example plugin: Line count for text files
pub struct LineCountPlugin;

impl FileInfoPlugin for LineCountPlugin {
    fn name(&self) -> &'static str {
        "Lines"
    }
    
    fn extract_info(&self, path: &Path, metadata: &fs::Metadata) -> String {
        if metadata.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                match ext {
                    "txt" | "rs" | "py" | "js" | "md" => {
                        // In real implementation, count actual lines
                        format!("~{}", metadata.len() / 50) // Rough estimate
                    }
                    _ => "N/A".to_string()
                }
            } else {
                "N/A".to_string()
            }
        } else {
            "N/A".to_string()
        }
    }
}

/// Plugin registry
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn FileInfoPlugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            plugins: HashMap::new(),
        };
        
        // Register built-in plugins
        registry.register("extension", Box::new(ExtensionPlugin));
        registry.register("hash", Box::new(HashPlugin));
        registry.register("lines", Box::new(LineCountPlugin));
        
        registry
    }
    
    pub fn register(&mut self, name: &str, plugin: Box<dyn FileInfoPlugin>) {
        self.plugins.insert(name.to_string(), plugin);
    }
    
    pub fn get_enabled_plugins(&self, enabled_plugins: &[String]) -> Vec<&dyn FileInfoPlugin> {
        enabled_plugins
            .iter()
            .filter_map(|name| self.plugins.get(name).map(|p| p.as_ref()))
            .collect()
    }
    
    pub fn list_available(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
}

/// Enhanced FileInfo that supports plugins
pub struct ExtendedFileInfo {
    pub name: String,
    pub file_type: String,
    pub size: String,
    pub modified: String,
    pub plugin_data: HashMap<String, String>,
}

impl ExtendedFileInfo {
    pub fn from_path_with_plugins(
        path: &Path,
        metadata: &fs::Metadata,
        plugins: &[&dyn FileInfoPlugin],
    ) -> Self {
        let mut plugin_data = HashMap::new();
        
        for plugin in plugins {
            let info = plugin.extract_info(path, metadata);
            plugin_data.insert(plugin.name().to_string(), info);
        }
        
        Self {
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            file_type: if metadata.is_dir() { "Directory" } else { "File" }.to_string(),
            size: format!("{}B", metadata.len()),
            modified: "Recent".to_string(), // Simplified
            plugin_data,
        }
    }
}

fn main() {
    println!("Example: Plugin system for extending file information\n");
    
    // Create plugin registry
    let registry = PluginRegistry::new();
    
    println!("Available plugins:");
    for plugin_name in registry.list_available() {
        println!("  - {}", plugin_name);
    }
    
    // Example usage
    let enabled_plugins = vec!["extension".to_string(), "lines".to_string()];
    let plugins = registry.get_enabled_plugins(&enabled_plugins);
    
    println!("\nEnabled plugins: {:?}", enabled_plugins);
    
    // Simulate processing a file
    let dummy_path = Path::new("example.rs");
    let dummy_metadata = fs::metadata(".").unwrap(); // Use current dir as dummy
    
    let file_info = ExtendedFileInfo::from_path_with_plugins(
        dummy_path,
        &dummy_metadata,
        &plugins,
    );
    
    println!("\nExample output:");
    println!("Name: {}", file_info.name);
    println!("Type: {}", file_info.file_type);
    println!("Size: {}", file_info.size);
    
    for (plugin_name, data) in &file_info.plugin_data {
        println!("{}: {}", plugin_name, data);
    }
    
    println!("\nIntegration steps:");
    println!("1. Add plugin support to Config struct");
    println!("2. Add CLI arguments: --plugins extension,lines");
    println!("3. Modify FileInfo to include plugin data");
    println!("4. Update table display to show plugin columns");
    println!("5. Allow loading external plugins via dynamic libraries");
}