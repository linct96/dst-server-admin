use crate::api::res::{Res, ResBody};
use crate::service;
use crate::service::game::StartServerReq;
use crate::service::task::{SystemInfo, SYSTEM_INFO};
use crate::utils;

use axum::routing::{get, post};
use axum::Router;
use axum::{http::HeaderMap, Json};
use serde::Serialize;

pub fn router_system() -> Router {
    Router::new()
        .route("/force_install_dst_server", post(force_install_dst_server)) // 登录
        .route("/get_system_info", get(get_system_info)) // 登录
        .route("/start_dst_server", post(start_dst_server)) // 启动游戏服务器
        .route("/stop_dst_server", post(stop_dst_server)) // 启动游戏服务器
        .route("/test_fn", get(test_fn)) // 启动游戏服务器
}

pub async fn test_fn() -> ResBody<bool> {
    let result = utils::shell::install_lib().await;
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
