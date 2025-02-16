use crate::api::{res::ResBody, user};
use axum::routing::get;
use axum::Router;

pub fn router_un_auth() -> Router {
    Router::new().route("/login", get(user::login)) // 登录
}
pub async fn get_system_info() -> ResBody<bool> {
    ResBody::success(true)
}
