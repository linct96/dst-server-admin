use crate::{api::res::ResBody, service::game, utils};
use axum::{
    routing::{get, post},
    Router,
};

pub fn router_game() -> Router {
    Router::new()
        .route("/test_fn", get(test_fn))
        .route("/install_steam_cmd", post(install_steam_cmd))
        .route("/update_dedicated_server", post(update_dedicated_server))
    // 登录
}
pub async fn test_fn() -> ResBody<bool> {
    let result = utils::shell::install_lib().await;
    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
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
