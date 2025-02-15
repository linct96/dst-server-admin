use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

fn copy_assets(src: &Path, dst: &Path) -> io::Result<()> {
    // 创建目标目录
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


    let source_static_dir = current_dir.clone().ancestors().nth(2).unwrap().join("web/dist");
    let target_static_dir = Path::new(&out_dir)
        .ancestors() // 向上遍历父目录
        .nth(3) // 获取 target/debug 或 target/release 目录
        .unwrap();

    println!("cargo:rerun-if-changed={}", source_assets_dir.display());
    copy_assets(&source_assets_dir, &target_assets_dir).expect("Failed to copy assets");
    println!("cargo:rerun-if-changed={}", source_static_dir.display());
    copy_assets(&source_static_dir, &target_static_dir).expect("Failed to copy assets");
}
