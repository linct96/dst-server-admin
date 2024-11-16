use std::env::consts::OS;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

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
