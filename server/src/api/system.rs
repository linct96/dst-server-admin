use std::env::consts::OS;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

use crate::api::res::{Res, ResBody};
use crate::config::config::{Config, PathConfig, CONFIG_PATH};
use crate::service::s_user::{login_service, AuthBody, UserLoginReq};
use crate::utils::file::{download_file, trans_content_to_path, unzip_file};
use crate::utils::system::SystemInfo;
use crate::utils::{file, shell};

use asset::STATIC_DIR;
use axum::routing::{get, post};
use axum::Router;
use axum::{http::HeaderMap, Json};
use reqwest::Client;
use serde::Serialize;
use std::io::{self, Write};
// use super::res::{Res,Result};

pub fn router_system() -> Router {
    Router::new()
        .route("/get_system_info", get(get_system_info)) // 登录
        .route("/get_game_info", get(get_game_info)) // 登录
        .route("/update_dst_server", post(update_dst_server)) // 安装、更新服务器
        .route("/start_dst_server", post(start_dst_server)) // 启动游戏服务器
        .route(
            "/update_dst_server_windows",
            post(update_dst_server_windows),
        ) // 启动游戏服务器
}
pub async fn get_system_info() -> ResBody<SystemInfo> {
    let system_info = SystemInfo::get();
    ResBody::success(system_info)
}

#[derive(Debug, Serialize, Clone)]
pub struct GameInfo {
    pub path: String,
    pub version: String,
}

pub async fn start_dst_server() -> ResBody<bool> {
    let mut sh_name = "run_cluster.sh";

    if OS == "windows" {
        sh_name = "install_windows.bat";
    }

    if let Some(file) = STATIC_DIR.get_file(sh_name) {
        // 打印文件内'
        // 构建 screen 命令
        let temp_file_path = trans_content_to_path(file.contents_utf8().unwrap());
        let mut command = Command::new("screen");
        command
            .arg("-dmS") // 以分离模式启动一个新的 screen 会话
            .arg("my_session_name") // 指定会话名称
            .arg("-c") // 使用 -c 选项指定配置文件
            .arg(temp_file_path); // 传递临时文件路径作为配置文件
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
    };

    let path_config = PathConfig::new();
    println!("path_config: {:#?}", path_config);
    game_info.path = path_config.dst_server_path.to_str().unwrap().to_string();
    let dst_version_path: PathBuf = path_config.dst_server_path.join("version.txt");
    if path_config.dst_server_path.exists() {
        if let Ok(dst_version) = fs::read_to_string(dst_version_path) {
            game_info.version = dst_version;
        }
    }

    ResBody::success(game_info)
}
pub async fn update_dst_server_windows() -> ResBody<bool> {
    let url = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
    let output_path = "./steamcmd.zip";
    let path_config = PathConfig::new();
    if Path::new(output_path).exists() {
        fs::remove_file(output_path).unwrap();
    } else {
        let res = download_file(url, output_path).await;
        if let Err(e) = res {
            return ResBody::err(false, e.to_string());
        }
        unzip_file(output_path, path_config.steam_cmd_path.to_str().unwrap()).await;
        
    }

    ResBody::success(true)
}

pub async fn update_dst_server_linux() -> ResBody<bool> {
    let mut sh_name = "install_linux.sh";

    if OS == "macos" {
        sh_name = "install_macOS.sh";
    } else if OS == "windows" {
        sh_name = "install_windows.bat";
    }

    if let Some(file) = STATIC_DIR.get_file(sh_name) {
        // 打印文件内'
        let file_path = file.path().to_str().unwrap();
        println!("path: {}", file_path);
        let content = file.contents_utf8().unwrap();
        shell::run_command(content);
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
