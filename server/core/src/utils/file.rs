use reqwest::header::CONTENT_DISPOSITION;
use std::env::consts::OS;
use std::io::Write;
use std::{fs, io, path};
use tempfile::Builder;
use tokio::time;

pub fn is_dir(path: &str) -> bool {
    path::Path::new(path).is_dir()
}

pub fn create_dir(path: &str) -> bool {
    match fs::create_dir_all(path) {
        Ok(_) => true,
        Err(_) => false,
    }
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
                let content_disposition = response.headers().get(reqwest::header::CONTENT_DISPOSITION).cloned();
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
    Ok(())
}
