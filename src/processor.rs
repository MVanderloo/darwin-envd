use std::fs;
use std::path::Path;

use crate::path_utils::contract_path;
use crate::shell::Shell;
use crate::variable_expansion::process_value;

pub fn process_directory(dir_path: &Path, shell: &Shell, extension: &str, debug_level: u8) {
    if debug_level >= 1 {
        eprintln!("Reading directory: {}", contract_path(dir_path));
    }

    let mut conf_files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let file_path = entry.path();
            if file_path.is_file() && file_path.to_string_lossy().ends_with(extension) {
                conf_files.push(file_path);
            }
        }
    }

    conf_files.sort();

    for file_path in conf_files {
        if debug_level >= 1 {
            // Use relative path to the directory for debugging output
            if let Ok(relative_path) = file_path.strip_prefix(dir_path) {
                eprintln!("  {}", relative_path.display());
            } else {
                eprintln!(
                    "  {}",
                    file_path.file_name().unwrap_or_default().to_string_lossy()
                );
            }
        }

        if let Ok(content) = fs::read_to_string(&file_path) {
            for line in content.lines() {
                let line = line.trim();

                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                if let Some(pos) = line.find('=') {
                    let key = line[..pos].trim();
                    let raw_value = line[pos + 1..].trim();

                    if !key.is_empty() {
                        let processed_value = process_value(raw_value, shell, debug_level);
                        let export_cmd = shell.format_export(key, &processed_value);

                        if debug_level >= 3 {
                            eprintln!("    Original: {}", line);
                            eprintln!("    Processed: {}", processed_value);
                            eprintln!("    Translated: {}", export_cmd);
                        } else if debug_level >= 2 {
                            eprintln!("    {}", line);
                        }

                        println!("{}", export_cmd);
                    }
                }
            }
        } else if debug_level >= 1 {
            if let Ok(relative_path) = file_path.strip_prefix(dir_path) {
                eprintln!("Error reading file: {}", relative_path.display());
            } else {
                eprintln!(
                    "Error reading file: {}",
                    file_path.file_name().unwrap_or_default().to_string_lossy()
                );
            }
        }
    }
}
