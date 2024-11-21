use std::env::consts::OS;

use std::path::{Path, PathBuf};
use tokio::fs;

use crate::api::res::{Res, ResBody};
use crate::config::config::{Config, PathConfig, CONFIG_PATH};
use crate::service;
use crate::service::game::{DstSaveInfo, StartServerReq};
use crate::service::s_user::{login_service, AuthBody, UserLoginReq};
use crate::service::task::{SystemInfo, SYSTEM_INFO};
use crate::utils::file::{download_file, trans_content_to_file};
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
        .route("/get_all_saves", get(get_all_saves)) // 登录
        .route("/get_system_info", get(get_system_info)) // 登录
        .route("/get_game_info", get(get_game_info)) // 登录
        .route("/update_dst_server", post(update_dst_server)) // 安装、更新服务器
        .route("/start_dst_server", post(start_dst_server)) // 启动游戏服务器
        .route("/stop_dst_server", post(stop_dst_server)) // 启动游戏服务器
        .route("/test_fn", get(test_fn)) // 启动游戏服务器
}

pub async fn test_fn() -> ResBody<bool> {
    let result = shell::install_steam_cmd(true).await;
    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}
pub async fn get_system_info() -> ResBody<SystemInfo> {
    let system_info = SYSTEM_INFO.lock().await.clone();
    // let system_info = SystemInfo::get();
    ResBody::success(system_info)
}

#[derive(Debug, Serialize, Clone)]
pub struct GameInfo {
    pub path: String,
    pub version: String,
    pub server_installed: bool,
}

pub async fn get_all_saves() -> ResBody<Vec<DstSaveInfo>> {
    let result = service::game::service_get_all_saves().await;

    match result {
        Ok(data) => ResBody::success(data),
        Err(e) => ResBody::err(vec![], e.to_string()),
    }
}

pub async fn update_dst_server() -> ResBody<bool> {
    let result = service::game::service_update_dst_server().await;

    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}

pub async fn force_install_dst_server() -> ResBody<bool> {
    let result = service::game::service_force_install_dst_server().await;

    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}


pub async fn start_dst_server(header: HeaderMap, Json(req): Json<StartServerReq>) -> ResBody<bool> {
    let result = service::game::service_start_dst_server(req).await;

    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}
pub async fn stop_dst_server(header: HeaderMap, Json(req): Json<StartServerReq>) -> ResBody<bool> {
    let result = service::game::service_stop_dst_server(req).await;

    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
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
