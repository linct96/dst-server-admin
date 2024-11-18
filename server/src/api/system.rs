use std::env::consts::OS;
use std::io::Write;

use axum::extract::path;
use tempfile::NamedTempFile;
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;

use crate::api::res::{Res, ResBody};
use crate::config::config::{Config, PathConfig, CONFIG_PATH};
use crate::service::s_user::{login_service, AuthBody, UserLoginReq};
use crate::utils::file::{download_file, trans_content_to_file, unzip_file};
use crate::utils::shell::run_command;
use crate::utils::system::SystemInfo;
use crate::utils::{file, shell};

use asset::STATIC_DIR;
use axum::routing::{get, post};
use axum::Router;
use axum::{http::HeaderMap, Json};
use serde::Serialize;
// use super::res::{Res,Result};

pub fn router_system() -> Router {
    Router::new()
        .route("/force_install_dst_server", post(force_install_dst_server)) // 登录
        .route("/get_system_info", get(get_system_info)) // 登录
        .route("/get_game_info", get(get_game_info)) // 登录
        .route("/update_dst_server", post(update_dst_server)) // 安装、更新服务器
        .route("/start_dst_server", post(start_dst_server)) // 启动游戏服务器
}
pub async fn get_system_info() -> ResBody<SystemInfo> {
    let system_info = SystemInfo::get();
    ResBody::success(system_info)
}

#[derive(Debug, Serialize, Clone)]
pub struct GameInfo {
    pub path: String,
    pub version: String,
    pub server_installed: bool,
}

pub async fn remove_dst_server() {
    let path_config = PathConfig::new();
    let steam_app_path = path_config.steam_app_path.to_str().unwrap();
    let steam_cmd_path = path_config.steam_cmd_path.to_str().unwrap();
    if Path::new(steam_app_path).exists() {
        fs::remove_dir_all(steam_app_path).await.unwrap();
    }
    if Path::new(steam_cmd_path).exists() {
        fs::remove_dir_all(steam_cmd_path).await.unwrap();
    }
    if Path::new("steamcmd.zip").exists() {
        fs::remove_file("steamcmd.zip").await.unwrap();
    }
}

pub async fn force_install_dst_server() -> ResBody<bool> {
    remove_dst_server().await;
    println!("remove_dst_server success");
    if OS == "windows" {
        return update_dst_server_windows().await;
    } else {
        return update_dst_server_linux().await;
    }
}

pub async fn start_dst_server() -> ResBody<bool> {
    let mut sh_name = "run_cluster.sh";

    if OS == "windows" {
        sh_name = "install_windows.bat";
    }

    if let Some(file) = STATIC_DIR.get_file(sh_name) {
        // 打印文件内'
        // 构建 screen 命令
        let callback = |path: PathBuf| shell::run_command(path.to_str().unwrap(), [sh_name.to_string()].to_vec());
        trans_content_to_file(file.contents_utf8().unwrap(), ".sh");
    } else {
        println!("File not found");
    }
    ResBody::success(true)
}

// 获取游戏信息
pub async fn get_game_info() -> ResBody<GameInfo> {
    let mut game_info = GameInfo {
        path: "".to_string(),
        version: "".to_string(),
        server_installed: false,
    };

    let path_config = PathConfig::new();

    game_info.path = path_config.dst_server_path.to_str().unwrap().to_string();
    let dst_version_path: PathBuf = path_config.dst_server_path.join("version.txt");
    if path_config.dst_server_path.exists() {
        if let Ok(dst_version) = fs::read_to_string(dst_version_path).await {
            game_info.version = dst_version.replace("\n", "").replace("\r", "");
        }
    }

    ResBody::success(game_info)
}
pub async fn update_dst_server_windows() -> ResBody<bool> {
    let url = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
    let output_path = "./steamcmd.zip";
    let path_config = PathConfig::new();
    if Path::new(output_path).exists() {
        println!("exist steamcmd.zip");
        // fs::remove_file(output_path).await.unwrap();
    } else {
        let res = download_file(url, output_path).await;
        if let Err(e) = res {
            return ResBody::err(false, e.to_string());
        }
    }
    let steam_cmd_path = path_config.steam_cmd_path.to_str().unwrap();
    unzip_file(output_path, steam_cmd_path).await;

    let callback = |path: PathBuf| shell::run_command(path.to_str().unwrap(), [].to_vec());
    let script = format!(
        "{}/steamcmd.exe +login anonymous +app_update 343050 validate +quit",
        steam_cmd_path
    );
    let temp_file_path = trans_content_to_file(&script, ".sh");
    ResBody::success(true)
}

pub async fn update_dst_server_linux() -> ResBody<bool> {
    let mut sh_name = "install.sh";

    if OS == "macos" {
        sh_name = "install.sh";
    } else if OS == "windows" {
        sh_name = "install_windows.bat";
    }

    let path_config = PathConfig::new();
    let steam_cmd_path = path_config.steam_cmd_path.to_str().unwrap();
    // let args = vec![];
    let callback = |path: &str| {
        shell::run_command(path, ["1".to_string()].to_vec())
    };
    // trans_content_to_file("1", ".sh", callback);
    if let Some(file) = STATIC_DIR.get_file(sh_name) {
        // 打印文件内'
        let content = file.contents_utf8().unwrap();
        let mut temp_file = NamedTempFile::new().unwrap();
        temp_file.write_all(file.contents()).unwrap();
        let temp_file_path = trans_content_to_file(content, ".sh").unwrap();
        shell::run_command(temp_file.path().to_str().unwrap(), vec![steam_cmd_path.to_string()]);
    } else {
        println!("File not found");
    }
    ResBody::success(true)
}

pub async fn update_dst_server() -> ResBody<bool> {
    if OS == "windows" {
        return update_dst_server_windows().await;
    } else {
        return update_dst_server_linux().await;
    }
}
pub async fn get_system_info_v(
    header: HeaderMap,
    Json(login_req): Json<UserLoginReq>,
) -> Res<AuthBody> {
    // let db = DB.get_or_init(db_conn).await;
    // match service::sys_user::login(db, login_req, header).await {
    //     Ok(x) => Res::with_data(x),
    //     Err(e) => Res::with_err(&e.to_string()),
    // }
    let res = login_service(login_req, header).await;
    match res {
        Ok(x) => Res::<AuthBody>::with_data(x),
        Err(e) => Res::<AuthBody>::with_err(&e.to_string()),
    }
}
