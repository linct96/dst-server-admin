use std::env::consts::OS;
use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;

use reqwest::Error;
use tempfile::Builder;

use tokio::time::sleep;
use zip::ZipArchive;

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

pub fn trans_content_to_file(content: &str, suffix: &str) ->io::Result<PathBuf>{
    let mut temp_file = Builder::new()
        .suffix(suffix)
        .rand_bytes(5) // 生成随机字符串以确保文件名唯一
        .tempfile()
        .unwrap();
    temp_file
        .write_all(content.as_bytes())
        .expect("Failed to write to temp file");
    let temp_file_path = temp_file.path().to_path_buf();
    Ok(temp_file_path)
}

pub async fn download_file(url: &str, save_path: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let max_retries = 3;
    let retry_delay = Duration::from_secs(2);

    for attempt in 1..=max_retries {
        match client.get(url).send().await {
            Ok(response) => {
                // 处理响应
                let bytes = response.bytes().await?;
                // 保存文件等操作
                // 保存文件
                let mut file = fs::File::create(save_path).expect("Unable to create file");
                file.write_all(&bytes).expect("Unable to write data");
                return Ok(());
            }
            Err(e) => {
                println!("Attempt {}/{} failed: {:?}", attempt, max_retries, e);
                if attempt < max_retries {
                    sleep(retry_delay).await;
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(())
}

pub async fn unzip_file(origin_path: &str, output_path: &str) {
    // 解压文件
    let zip_file = fs::File::open(origin_path).unwrap();
    let mut archive = ZipArchive::new(zip_file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = format!("{}/{}", output_path, file.name());

        if file.is_dir() {
            fs::create_dir_all(&out_path).unwrap();
        } else {
            if let Some(p) = out_path.rsplit_once('/') {
                fs::create_dir_all(p.0).unwrap();
            }
            let mut out_file = fs::File::create(&out_path).unwrap();
            io::copy(&mut file, &mut out_file).unwrap();
        }
    }
}
