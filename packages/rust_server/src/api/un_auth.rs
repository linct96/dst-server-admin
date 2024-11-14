use crate::api::res::{Res, ResBody};
use crate::service::s_user::{login_service, AuthBody, UserLoginReq};
use crate::utils::system::SystemInfo;
use axum::routing::get;
use axum::Router;

pub fn router_un_auth() -> Router {
    Router::new().route("/login", get(get_system_info)) // 登录
}
pub async fn get_system_info() -> ResBody<SystemInfo> {
    let system_info = SystemInfo::get();
    ResBody::success(system_info)
}
