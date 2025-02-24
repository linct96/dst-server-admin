use regex::Regex;
use reqwest::header::CONTENT_DISPOSITION;
use serde::Serialize;
use std::env::consts::OS;
use std::io::{BufRead, BufWriter, Write};
use std::path::Path;
use std::{fs, io, path};
use tempfile::Builder;
use tokio::time;

use crate::context::static_config::EnumStaticConfigKey;
use crate::context::{self, static_config};

pub fn is_dir(path: &str) -> bool {
    path::Path::new(path).is_dir()
}

pub fn create_dir(path: &str) -> bool {
    match fs::create_dir_all(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn copy_dir_all(src: &str, dst: &str) -> io::Result<()> {
    // 确保目标目录存在，如果不存在则创建
    fs::create_dir_all(&dst)?;
    let dst_path = path::Path::new(dst);
    // 遍历源目录中的所有条目
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        let entry_path = entry.path();
        let full_dst_path = dst_path.join(entry.file_name());

        if filetype.is_dir() {
            // 如果是目录，递归复制
            copy_dir_all(
                entry_path.to_str().unwrap(),
                full_dst_path.to_str().unwrap(),
            )?;
        } else {
            // 如果是文件，直接复制
            fs::copy(&entry_path, &full_dst_path)?;
        }
    }
    Ok(())
}

pub fn list_dir(path: &str) -> Result<Vec<String>, io::Error> {
    let mut result = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name().to_str().unwrap().to_string();
        result.push(file_name);
    }
    Ok(result)
}

pub fn list_dir_with_target_file(path: &str, file_name: &str) -> Result<Vec<String>, io::Error> {
    let mut result = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_type = entry.file_type().unwrap();
        let entry_path = entry.path();
        let entry_name = entry.file_name();
        // 过滤非文件夹类型
        if entry_type.is_dir() {
            if path::PathBuf::from(entry_path).join(file_name).exists() {
                result.push(entry_name.to_str().unwrap().to_string());
            }
        }
    }
    Ok(result)
}

pub fn resolve_path(path: String) -> String {
    let resolved_path = if OS == "windows" {
        path.replace("/", "\\")
    } else {
        path.replace("\\", "/")
    };
    resolved_path
}

pub fn trans_content_to_file(content: &str, suffix: &str) -> io::Result<path::PathBuf> {
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

pub async fn download_file(url: &str, save_path: &str) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let max_retries = 3;
    let retry_delay = time::Duration::from_secs(2);

    for attempt in 1..=max_retries {
        match client.get(url).send().await {
            Ok(response) => {
                let content_disposition = response
                    .headers()
                    .get(reqwest::header::CONTENT_DISPOSITION)
                    .cloned();
                let filename = match content_disposition {
                    Some(value) => {
                        let value = value.to_str().unwrap().to_string();
                        let start = value.find("filename=").map(|i| i + 9).unwrap_or(0);
                        let end = value[start..]
                            .find('"')
                            .map(|i| start + i)
                            .unwrap_or(value.len());
                        value[start..end].to_string()
                    }
                    None => path::Path::new(url)
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("downloaded_file")
                        .to_string(),
                };
                let bytes = response.bytes().await.expect("Unable to read response");
                tokio::fs::create_dir_all(save_path).await.unwrap();
                let mut file = fs::File::create(format!("{}/{}", save_path, filename))
                    .expect("Unable to create file");
                file.write_all(&bytes).expect("Unable to write data");
                return Ok(filename);
            }
            Err(e) => {
                if attempt < max_retries {
                    time::sleep(retry_delay).await;
                } else {
                    return Err(anyhow::anyhow!(e));
                }
            }
        }
    }
    Err(anyhow::anyhow!("Failed to download file"))
}

fn unzip_gz(origin_path: &path::Path, output_path: &path::Path) -> anyhow::Result<()> {
    let tar_gz_file = std::fs::File::open(origin_path)?;
    let tar_decoder = flate2::read::GzDecoder::new(tar_gz_file);
    let mut archive = tar::Archive::new(tar_decoder);

    std::fs::create_dir_all(output_path)?;
    archive.unpack(output_path)?;

    Ok(())
}

fn unzip_zip(origin_path: &path::Path, output_path: &path::Path) -> anyhow::Result<()> {
    let zip_file = std::fs::File::open(origin_path)?;
    let mut archive = zip::ZipArchive::new(zip_file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let out_path = format!(
            "{}/{}",
            output_path.to_str().unwrap().to_string(),
            file.name()
        );

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
    Ok(())
}
pub fn unzip_file(origin_path: &str, output_path: &str) -> anyhow::Result<()> {
    std::fs::create_dir_all(output_path)?;
    let origin_path = path::Path::new(origin_path);
    let output_path = path::Path::new(output_path);

    let extension = origin_path.extension().unwrap().to_str().unwrap();

    match extension {
        "gz" => unzip_gz(origin_path, output_path)?,
        _ => unzip_zip(origin_path, output_path)?,
    }
    anyhow::Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub struct SetupMods {
    pub mods_collection: Vec<u64>,
    pub mods: Vec<u64>,
}
pub fn get_mod_setup(path: &str) -> io::Result<SetupMods> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    let mod_regex = Regex::new(r#"^\s*ServerModSetup\("(\d+)"\)\s*$"#).unwrap();
    let mod_collection_regex =
        Regex::new(r#"^\s*ServerModCollectionSetup\("(\d+)"\)\s*$"#).unwrap();
    let mut mods: Vec<_> = Vec::new();
    let mut mods_collection = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // 查找匹配的数字
        if let Some(captures) = mod_regex.captures(&line) {
            if let Some(number) = captures.get(1) {
                if let Ok(num) = number.as_str().parse::<u64>() {
                    mods.push(num);
                }
            }
        }
        if let Some(captures) = mod_collection_regex.captures(&line) {
            if let Some(number) = captures.get(1) {
                if let Ok(num) = number.as_str().parse::<u64>() {
                    mods_collection.push(num);
                }
            }
        }
    }

    Ok(SetupMods {
        mods_collection,
        mods,
    })
}

pub fn add_mod_setup(path: &str, mods: Vec<u64>) -> io::Result<()> {
    let current_mods = get_mod_setup(path)?;
    let file = fs::OpenOptions::new().append(true).open(path)?;
    let mut writer = io::BufWriter::new(file);

    for id in mods {
        if current_mods.mods.contains(&id) || current_mods.mods_collection.contains(&id) {
            continue;
        }
        writer.write_all(format!("\nServerModSetup(\"{}\")", id).as_bytes())?;
    }
    writer.flush()?;
    Ok(())
}

pub fn delete_mod_setup(path: &str, mods: Vec<u64>) -> io::Result<()> {
    // 读取文件内容
    let mut lines = io::BufReader::new(fs::File::open(path)?).lines();
    let mut buffer = String::new();

    // 处理每一行
    while let Some(line) = lines.next() {
        let line = line?;
        let mut found = false;
        for id in &mods {
            if line.contains(&format!("ServerModSetup(\"{}\")", id)) {
                found = true;
                break;
            }
        }
        if !found {
            buffer.push_str(&line);
            buffer.push('\n');
        }
    }

    // 覆盖写入文件
    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true) // 确保清空文件
        .open(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(buffer.as_bytes())?;
    writer.flush()?;
    Ok(())
}

pub fn get_save_worlds(save_name: String) -> io::Result<Vec<String>> {
    let static_config = context::static_config::get();
    let path_dst_save = static_config
        .get(EnumStaticConfigKey::DstSave.as_str())
        .unwrap();
    let target_save_path = Path::new(path_dst_save).join(save_name);
    if !target_save_path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "save not exists"));
    }
    let mut result = Vec::new();
    for entry in fs::read_dir(target_save_path)? {
        let entry = entry?;
        let file_name = entry.file_name().to_str().unwrap().to_string();
        if entry.path().is_dir() {
            if entry.path().join("server.ini").exists() {
                result.push(file_name);
            }

            continue;
        }
    }
    Ok(result)
}
