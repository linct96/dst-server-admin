use std::env::consts::OS;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use tempfile::NamedTempFile;

use crate::config::config::Config;

pub const CONFIG_DIR_NAME: &str = ".dst-server";

pub fn is_dir(path: &str) -> bool {
    Path::new(path).is_dir()
}

pub fn create_dir(path: &str) -> bool {
    match fs::create_dir_all(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn resolve_path(path: &str) -> PathBuf {
    let path = if OS == "windows" {
        path.replace("/", "\\")
    } else {
        path.replace("\\", "/")
    };
    PathBuf::from(path)
}

pub fn trans_content_to_path(content: &str)-> PathBuf {
    let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
    temp_file.write_all(content.as_bytes()).expect("Failed to write to temp file");
    let temp_file_path = temp_file.path().to_path_buf();
    return temp_file_path;
}
