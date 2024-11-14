use axum::routing::get;
use axum::Router;
use axum::{http::HeaderMap, Json};
use crate::service::s_user::{login_service, AuthBody, UserLoginReq};
use crate::api::res::{Res,ResBody};
use crate::utils::system::SystemInfo;
// use super::res::{Res,Result};

pub fn router_system() -> Router {
    Router::new()
        .route("/get_system_info", get(get_system_info)) // 登录
}
pub async fn get_system_info() -> ResBody<SystemInfo> {
    let system_info = SystemInfo::get();
    ResBody::success(system_info)
}
pub async fn get_system_info_v(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> Res<AuthBody> {
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
