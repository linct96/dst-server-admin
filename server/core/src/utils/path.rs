use std::{env, path};

pub fn resolve_current_exe_path(path: &str) -> path::PathBuf {
    let current_dir = env::current_exe()
        .expect("无权限读取当前exe文件")
        .parent()
        .unwrap()
        .to_path_buf();
    let path = path::Path::new(path);
    let resolved_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_dir.join(path)
    };
    resolved_path
}

pub fn resolve_current_dir_path(path: &str) -> path::PathBuf {
    let current_dir = env::current_dir().expect("无权限读取当前目录");
    let path = path::Path::new(path);
    let resolved_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_dir.join(path)
    };
    resolved_path
}

pub fn resolve_out_dir_path(path: &str) -> path::PathBuf {
    let out_dir = env::var("OUT_DIR").expect("环境变量不存在：OUT_DIR");
    let path = path::Path::new(path);
    let resolved_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        path.ancestors().nth(3).unwrap().join(&path)
    };
    resolved_path
}
