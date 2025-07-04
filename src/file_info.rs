//! File information and metadata handling.
//!
//! This module provides structures and functions for extracting and formatting
//! file metadata, including permissions, ownership, file types, and the main
//! FileInfo struct used for table display.

use std::fs;
use std::path::Path;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use tabled::Tabled;
use users::{get_group_by_gid, get_user_by_uid};

use crate::formatting::{format_octal_permissions, format_size, format_time};

/// Represents file information for table display.
///
/// This struct contains all the formatted information needed to display a file
/// in the table format. It uses the `Tabled` derive macro to automatically
/// generate table headers and formatting.
#[derive(Tabled)]
pub struct FileInfo {
    #[tabled(rename = "Name")]
    pub name: String,
    #[tabled(rename = "Type")]
    pub file_type: String,
    #[tabled(rename = "User Permission")]
    pub user_perms: String,
    #[tabled(rename = "Group Permission")]
    pub group_perms: String,
    #[tabled(rename = "Other Permission")]
    pub other_perms: String,
    #[tabled(rename = "Octal")]
    pub octal: String,
    #[tabled(rename = "User/Group (Owner)")]
    pub owner: String,
    #[tabled(rename = "Size")]
    pub size: String,
    #[tabled(rename = "Modified")]
    pub modified: String,
}

impl FileInfo {
    /// Creates a new FileInfo instance from file metadata.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the file
    /// * `metadata` - The file's metadata from the filesystem
    ///
    /// # Returns
    ///
    /// A new FileInfo instance with all fields populated from the metadata.
    pub fn from_metadata(name: String, metadata: &fs::Metadata) -> Self {
        Self {
            name,
            file_type: get_file_type(metadata),
            user_perms: get_user_permissions(metadata),
            group_perms: get_group_permissions(metadata),
            other_perms: get_other_permissions(metadata),
            octal: format_octal_permissions(metadata),
            owner: get_owner_info(metadata),
            size: format_size(metadata.len()),
            modified: format_time(metadata),
        }
    }

    /// Creates a new FileInfo instance from a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the file
    ///
    /// # Returns
    ///
    /// A Result containing the FileInfo instance or an error if the file cannot be accessed.
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let path = path.as_ref();
        let metadata = fs::metadata(path)?;
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        
        Ok(Self::from_metadata(name, &metadata))
    }

    /// Checks if this file is a directory.
    ///
    /// # Returns
    ///
    /// `true` if the file is a directory, `false` otherwise.
    pub fn is_directory(&self) -> bool {
        self.file_type == "Directory"
    }

    /// Checks if this file is executable.
    ///
    /// # Returns
    ///
    /// `true` if the file is executable, `false` otherwise.
    pub fn is_executable(&self) -> bool {
        self.file_type == "Executable"
    }

    /// Checks if this file is hidden (starts with a dot).
    ///
    /// # Returns
    ///
    /// `true` if the file is hidden, `false` otherwise.
    pub fn is_hidden(&self) -> bool {
        self.name.starts_with('.')
    }
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            file_type: "File".to_string(),
            user_perms: "None".to_string(),
            group_perms: "None".to_string(),
            other_perms: "None".to_string(),
            octal: "000".to_string(),
            owner: "unknown/unknown".to_string(),
            size: "0B".to_string(),
            modified: "Unknown".to_string(),
        }
    }
}

/// Checks if a file is executable by examining its permission bits.
///
/// # Arguments
///
/// * `metadata` - The file's metadata
///
/// # Returns
///
/// `true` if the file has execute permissions for any user (owner, group, or other)
pub fn is_executable(metadata: &fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}

/// Determines the human-readable file type based on metadata.
///
/// # Arguments
///
/// * `metadata` - The file's metadata
///
/// # Returns
///
/// A string describing the file type: "Directory", "Symlink", "Executable", or "File"
pub fn get_file_type(metadata: &fs::Metadata) -> String {
    if metadata.is_dir() {
        "Directory".to_string()
    } else if metadata.file_type().is_symlink() {
        "Symlink".to_string()
    } else if is_executable(metadata) {
        "Executable".to_string()
    } else {
        "File".to_string()
    }
}

/// Formats a permission group (3 bits) into human-readable text.
///
/// # Arguments
///
/// * `perm` - A 3-bit permission value (0-7)
///
/// # Returns
///
/// A comma-separated string of permissions ("Read", "Write", "Execute") or "None"
fn format_permission_group(perm: u32) -> String {
    let mut result = Vec::new();

    if perm & 4 != 0 {
        result.push("Read");
    }
    if perm & 2 != 0 {
        result.push("Write");
    }
    if perm & 1 != 0 {
        result.push("Execute");
    }

    if result.is_empty() {
        "None".to_string()
    } else {
        result.join(", ")
    }
}

fn get_user_permissions(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let user_perm = (mode >> 6) & 7;
    format_permission_group(user_perm)
}

fn get_group_permissions(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let group_perm = (mode >> 3) & 7;
    format_permission_group(group_perm)
}

fn get_other_permissions(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let other_perm = mode & 7;
    format_permission_group(other_perm)
}

fn get_owner_info(metadata: &fs::Metadata) -> String {
    let uid = metadata.uid();
    let gid = metadata.gid();

    let user_name = get_user_by_uid(uid)
        .map(|user| user.name().to_string_lossy().to_string())
        .unwrap_or_else(|| uid.to_string());

    let group_name = get_group_by_gid(gid)
        .map(|group| group.name().to_string_lossy().to_string())
        .unwrap_or_else(|| gid.to_string());

    format!("{}/{}", user_name, group_name)
}