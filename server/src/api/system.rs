use std::path::PathBuf;
use std::{env, fs};

use crate::api::res::{Res, ResBody};
use crate::config::config::CONFIG_PATH;
use crate::service::s_user::{login_service, AuthBody, UserLoginReq};
use crate::utils::file::resolve_path;
use crate::utils::system::SystemInfo;
use axum::routing::get;
use axum::Router;
use axum::{http::HeaderMap, Json};
use serde::Serialize;
// use super::res::{Res,Result};

pub fn router_system() -> Router {
    Router::new()
        .route("/get_system_info", get(get_system_info)) // 登录
        .route("/get_game_info", get(get_game_info)) // 登录
        .route("/get_saves_info", get(get_system_info)) // 登录
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

// 获取游戏信息
pub async fn get_game_info() -> ResBody<GameInfo> {
    let mut game_info = GameInfo {
        path: "".to_string(),
        version: "".to_string(),
    };

    let dst_server_path = dirs::home_dir()
        .unwrap()
        .join(resolve_path(CONFIG_PATH.dst_server_path));
    let dst_server_version_path = dst_server_path.join("version.txt");
    game_info.path = dst_server_path.to_str().unwrap().to_string();
    if dst_server_version_path.exists() {
        if let Ok(dst_version) = fs::read_to_string(dst_server_version_path) {
            game_info.version = dst_version;
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
