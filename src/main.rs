use chrono::{DateTime, Local};
use clap::{Arg, Command};
use colored::*;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use tabled::{settings::Style, Table, Tabled};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Tabled)]
struct FileInfo {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    file_type: String,
    #[tabled(rename = "User Permission")]
    user_perms: String,
    #[tabled(rename = "Group Permission")]
    group_perms: String,
    #[tabled(rename = "Other Permission")]
    other_perms: String,
    #[tabled(rename = "Octal")]
    octal: String,
    #[tabled(rename = "User/Group (Owner)")]
    owner: String,
    #[tabled(rename = "Size")]
    size: String,
    #[tabled(rename = "Modified")]
    modified: String,
}

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
        .get_matches();

    let path = matches.get_one::<String>("path").unwrap();
    let long_format = matches.get_flag("long");
    let show_hidden = matches.get_flag("all");

    list_directory(path, long_format, show_hidden);
}

fn list_directory(path: &str, long_format: bool, show_hidden: bool) {
    let dir = match fs::read_dir(path) {
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

    if long_format {
        display_table_format(&entries, show_hidden);
    } else {
        display_simple_format(&entries, show_hidden);
    }
}

fn display_simple_format(entries: &[Result<fs::DirEntry, std::io::Error>], show_hidden: bool) {
    for entry in entries {
        let Ok(entry) = entry else { continue };

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !show_hidden && file_name_str.starts_with('.') {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => {
                println!("{}", file_name_str);
                continue;
            }
        };

        let colored_name = get_colored_name(&file_name_str, &metadata);
        println!("{}", colored_name);
    }
}

fn display_table_format(entries: &[Result<fs::DirEntry, std::io::Error>], show_hidden: bool) {
    let mut file_infos = Vec::new();

    for entry in entries {
        let Ok(entry) = entry else { continue };

        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !show_hidden && file_name_str.starts_with('.') {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        let file_info = FileInfo {
            name: file_name_str.to_string(), // Use plain text for proper alignment
            file_type: get_file_type(&metadata),
            user_perms: get_user_permissions(&metadata),
            group_perms: get_group_permissions(&metadata),
            other_perms: get_other_permissions(&metadata),
            octal: format_octal_permissions(&metadata),
            owner: get_owner_info(&metadata),
            size: format_size(metadata.len()),
            modified: format_time(&metadata),
        };

        file_infos.push(file_info);
    }

    if !file_infos.is_empty() {
        let table = Table::new(file_infos).with(Style::modern()).to_string();

        // Apply colors after table is formatted
        // Why colors are applied after table creation:
        // The tabled crate calculates column widths based on string length.
        // ANSI escape codes (like \x1b[34m for blue) are invisible but count as characters,
        // which would break column alignment if included in the FileInfo struct.
        // Solution: Create table with plain text first, then replace filenames with colored versions.
        let colored_output = apply_colors_to_table(&table, &entries, show_hidden);
        println!("{}", colored_output);
    }
}

fn get_colored_name(file_name: &str, metadata: &fs::Metadata) -> ColoredString {
    if file_name.starts_with('.') {
        file_name.bright_black()
    } else if metadata.is_dir() {
        file_name.blue().bold()
    } else if is_executable(metadata) {
        file_name.green().bold()
    } else {
        file_name.normal()
    }
}

fn get_colored_name_string(file_name: &str, metadata: &fs::Metadata) -> String {
    if file_name.starts_with('.') {
        format!("{}", file_name.bright_black())
    } else if metadata.is_dir() {
        format!("{}", file_name.blue().bold())
    } else if is_executable(metadata) {
        format!("{}", file_name.green().bold())
    } else {
        file_name.to_string()
    }
}

fn _format_permissions(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut perms = String::new();

    if metadata.is_dir() {
        perms.push('d');
    } else if metadata.file_type().is_symlink() {
        perms.push('l');
    } else {
        perms.push('-');
    }

    let owner = (mode >> 6) & 7;
    let group = (mode >> 3) & 7;
    let other = mode & 7;

    for &perm in &[owner, group, other] {
        perms.push(if perm & 4 != 0 { 'r' } else { '-' });
        perms.push(if perm & 2 != 0 { 'w' } else { '-' });
        perms.push(if perm & 1 != 0 { 'x' } else { '-' });
    }

    let special_bits = (mode >> 9) & 7;
    if special_bits & 4 != 0 {
        perms = perms.replacen("x", "s", 1);
    }
    if special_bits & 2 != 0 {
        let pos = perms.len() - 4;
        perms.replace_range(pos..pos + 1, "s");
    }
    if special_bits & 1 != 0 {
        let pos = perms.len() - 1;
        perms.replace_range(pos..pos + 1, "t");
    }

    perms
}

fn format_octal_permissions(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    format!("{:o}", mode & 0o7777)
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

fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{}B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1}K", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1}M", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1}G", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

fn get_colored_size_string(size_str: &str, size_bytes: u64) -> String {
    // Color coding for file sizes:
    // Green: < 1MB (small files)
    // Yellow: 1MB - 100MB (medium files)
    // Magenta: > 100MB (large files)
    // Red: > 1GB (very large files)
    if size_bytes >= 1024 * 1024 * 1024 {
        // >= 1GB - Red
        format!("{}", size_str.red().bold())
    } else if size_bytes >= 100 * 1024 * 1024 {
        // >= 100MB - Magenta
        format!("{}", size_str.magenta())
    } else if size_bytes >= 1024 * 1024 {
        // >= 1MB - Yellow
        format!("{}", size_str.yellow())
    } else {
        // < 1MB - Green
        format!("{}", size_str.green())
    }
}

fn format_time(metadata: &fs::Metadata) -> String {
    match metadata.modified() {
        Ok(time) => {
            let datetime: DateTime<Local> = time.into();
            datetime.format("%b %d %H:%M").to_string()
        }
        Err(_) => "Unknown".to_string(),
    }
}

fn is_executable(metadata: &fs::Metadata) -> bool {
    metadata.permissions().mode() & 0o111 != 0
}

fn get_file_type(metadata: &fs::Metadata) -> String {
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

fn apply_colors_to_table(
    table: &str,
    entries: &[Result<fs::DirEntry, std::io::Error>],
    show_hidden: bool,
) -> String {
    let mut result = table.to_string();

    // Collect all file names and sizes, sort by length (longest first) to avoid partial replacements
    let mut file_entries = Vec::new();
    let mut size_entries = Vec::new();

    for entry in entries {
        let Ok(entry) = entry else { continue };
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if !show_hidden && file_name_str.starts_with('.') {
            continue;
        }

        if let Ok(metadata) = entry.metadata() {
            let colored_name = get_colored_name_string(&file_name_str, &metadata);
            file_entries.push((file_name_str.to_string(), colored_name));

            // Also collect size information for coloring
            let size = metadata.len();
            let size_str = format_size(size);
            let colored_size = get_colored_size_string(&size_str, size);
            size_entries.push((size_str, colored_size));
        }
    }

    // Sort by filename length (longest first) to avoid partial matches
    file_entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    // Sort sizes by length too for proper replacement
    size_entries.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    // Apply filename color replacements
    for (file_name, colored_name) in file_entries {
        let lines: Vec<&str> = result.split('\n').collect();
        let mut new_lines = Vec::new();

        for line in lines {
            // Only replace if it's the actual filename in the first column with exact boundary
            let filename_pattern = format!("│ {} ", file_name); // Add space to ensure exact match
            if line.contains(&filename_pattern) {
                // Replace only the filename part
                let new_line = line.replace(&filename_pattern, &format!("│ {} ", colored_name));
                new_lines.push(new_line);
            } else {
                new_lines.push(line.to_string());
            }
        }

        result = new_lines.join("\n");
    }

    // Apply size color replacements
    for (size_str, colored_size) in size_entries {
        let lines: Vec<&str> = result.split('\n').collect();
        let mut new_lines = Vec::new();

        for line in lines {
            // Look for size pattern with proper boundaries (spaces or │)
            if line.contains(&size_str) {
                // Replace size ensuring we don't replace partial matches
                let size_pattern = format!(" {} ", size_str);
                let colored_pattern = format!(" {} ", colored_size);
                if line.contains(&size_pattern) {
                    let new_line = line.replace(&size_pattern, &colored_pattern);
                    new_lines.push(new_line);
                } else {
                    // Check for size at end of cell (before │)
                    let size_pattern_end = format!(" {} │", size_str);
                    let colored_pattern_end = format!(" {} │", colored_size);
                    if line.contains(&size_pattern_end) {
                        let new_line = line.replace(&size_pattern_end, &colored_pattern_end);
                        new_lines.push(new_line);
                    } else {
                        new_lines.push(line.to_string());
                    }
                }
            } else {
                new_lines.push(line.to_string());
            }
        }

        result = new_lines.join("\n");
    }

    result
}
