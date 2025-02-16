use std::path::Path;
use std::{env, fs, io, path};

fn resolve_current_exe_path(path: &str) -> path::PathBuf {
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

fn resolve_current_dir_path(path: &str) -> path::PathBuf {
    let current_dir = env::current_dir().expect("无权限读取当前目录");
    let path = path::Path::new(path);
    let resolved_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_dir
            .join(path)
            .canonicalize()
            .expect("路径序列化错误")
    };
    resolved_path
}

fn resolve_out_dir_path(path: &str) -> path::PathBuf {
    let out_dir = env::var("OUT_DIR").expect("环境变量不存在：OUT_DIR");
    let out_dir_path = path::Path::new(&out_dir);
    let path = path::Path::new(path);
    let resolved_path = if path.is_absolute() {
        path.to_path_buf().canonicalize().expect("路径序列化错误")
    } else {
        out_dir_path.ancestors().nth(3).unwrap().join(&path)
    };
    resolved_path
}

fn copy_assets(src: &Path, dst: &Path) -> io::Result<()> {
    // 创建目标目录
    if src.is_file() {
        fs::copy(&src, &dst)?;
    }
    fs::create_dir_all(dst)?;

    // 遍历源目录中的所有条目
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        // 如果是目录，递归调用 copy_assets
        if path.is_dir() {
            copy_assets(&path, &dest_path)?;
        } else {
            // 如果是文件，复制文件
            fs::copy(&path, &dest_path)?;
        }
    }
    Ok(())
}

fn copy_file_to_dir(src: &Path, dst_dir: &Path) -> io::Result<()> {
    if !src.is_file() {
        return Err(io::Error::new(io::ErrorKind::Other, "Source is not a file"));
    }
    let dst = dst_dir.join(src.file_name().unwrap());
    fs::copy(src, &dst)?;
    Ok(())
}
/// 递归地复制目录及其内容到目标目录
fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Source is not a directory",
        ));
    }

    // 创建目标目录（如果它不存在）
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
fn copy_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if src.is_dir() {
        // 如果源是目录，则递归复制目录及其内容
        copy_dir_recursive(src, dst)
    } else if src.is_file() {
        // 如果源是文件，则复制文件到目标目录
        copy_file_to_dir(src, dst)
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Source path is neither a file nor a directory",
        ))
    }
}

fn main() {
    // 读取环境变量
    let current_dir = std::env::current_dir().unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let assets_name = "assets";
    let source_assets_dir = current_dir.clone().join("assets"); // 源目录
    let target_assets_dir = Path::new(&out_dir)
        .ancestors() // 向上遍历父目录
        .nth(3) // 获取 target/debug 或 target/release 目录
        .unwrap()
        .join("scripts");

    let source_static_dir = current_dir
        .clone()
        .ancestors()
        .nth(2)
        .unwrap()
        .join("web/dist");
    let target_static_dir = Path::new(&out_dir)
        .ancestors() // 向上遍历父目录
        .nth(3) // 获取 target/debug 或 target/release 目录
        .unwrap();

    let assets_source_path = resolve_current_dir_path("script");
    println!("assets_source_path={}", assets_source_path.display());
    let assets_target_path = resolve_out_dir_path("script");
    println!("assets_target_path={}", assets_target_path.display());
    copy_recursive(&assets_source_path, &assets_target_path).expect("Failed to copy assets");
    let static_source_path = resolve_current_dir_path("../../web/dist");
    println!("static_source_path={}", static_source_path.display());
    let static_target_path = resolve_out_dir_path("");
    println!("static_target_path={}", static_target_path.display());
    copy_recursive(&static_source_path, &static_target_path).expect("Failed to copy assets");

    let config_source_path = resolve_current_dir_path("Config.toml");
    let config_target_path = resolve_out_dir_path("");
    copy_recursive(&config_source_path, &config_target_path).expect("Failed to copy assets");

    copy_recursive(
        &resolve_current_dir_path("Path.toml"),
        &resolve_out_dir_path(""),
    )
    .expect("Failed to copy Path.toml");

    copy_recursive(
        &resolve_current_dir_path("PathWindows.toml"),
        &resolve_out_dir_path(""),
    )
    .expect("Failed to copy PathWindows.toml");
}
