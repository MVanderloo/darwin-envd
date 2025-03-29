use std::path::{Path, PathBuf};

pub fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

pub fn contract_path(path: &Path) -> String {
    if let Ok(home) = std::env::var("HOME") {
        let home_path = PathBuf::from(&home);
        if let Ok(path_str) = path.to_path_buf().canonicalize() {
            if let Ok(rel_path) = path_str.strip_prefix(&home_path) {
                return format!("~/{}", rel_path.display());
            }
        }
    }
    path.display().to_string()
}
