use crate::{api::res::ResBody, service::game::{self, GameInfo}, utils};
use axum::{
    http::HeaderMap, routing::{get, post}, Json, Router
};

pub fn router_game() -> Router {
    Router::new()
        .route("/test_fn", get(test_fn))
        .route("/get_game_info", get(get_game_info))
        .route("/get_all_saves", get(get_all_saves))
        .route("/install_steam_cmd", post(install_steam_cmd))
        .route("/update_dedicated_server", post(update_dedicated_server))
        .route("/start_dst_server", post(start_dst_server))
        .route("/stop_dst_server", post(stop_dst_server))
    // 登录
}
pub async fn test_fn() -> ResBody<bool> {
    let result = utils::shell::install_lib().await;
    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}

pub async fn get_game_info() -> ResBody<GameInfo> {
    let result = game::service_get_game_info().await;

    match result {
        Ok(data) => ResBody::success(data),
        Err(e) => ResBody::err(GameInfo::default(), e.to_string()),
    }
}

pub async fn install_steam_cmd() -> ResBody<bool> {
    let result = game::service_install_steam_cmd().await;
    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}

pub async fn update_dedicated_server() -> ResBody<bool> {
    let result = game::service_update_dedicated_server().await;
    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}

pub async fn get_all_saves() -> ResBody<Vec<game::DstSaveInfo>> {
    let result = game::service_get_all_saves().await;

    match result {
        Ok(data) => ResBody::success(data),
        Err(e) => ResBody::err(vec![], e.to_string()),
    }
}

pub async fn start_dst_server(header: HeaderMap, Json(req): Json<game::StartServerReq>) -> ResBody<bool> {
  let result = game::service_start_dst_server(req).await;

  match result {
      Ok(_) => ResBody::success(true),
      Err(e) => ResBody::err(false, e.to_string()),
  }
}

pub async fn stop_dst_server(header: HeaderMap, Json(req): Json<game::StartServerReq>) -> ResBody<bool> {
    let result = game::service_stop_dst_server(req).await;

    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}


