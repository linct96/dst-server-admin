use crate::service::s_user::{login_service, AuthBody, UserLoginReq};
use crate::api::res::Res;

use axum::{http::HeaderMap, routing::post, Json, Router};
// use super::{res::Res, user::login};
// mod user;

pub fn entry() -> Router {
    Router::new()
        // 文件上传api
        // .nest_service(&CFG.web.upload_url, get_service(ServeDir::new(&CFG.web.upload_dir)))
        // 无需授权Api.通用模块
        .nest("/unAuth", no_auth_api())
    // 系统管理模块
    // .nest("/auth", set_auth_middleware(system::system_api()))
}


async fn lg(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> &'static str {
    "Hello, World!"
}

pub async fn lg2(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> Res<AuthBody> {
  let res = login_service(login_req, header).await;
  match res {
      Ok(x) => Res::<AuthBody>::with_data(x),
      Err(e) => Res::<AuthBody>::with_err(&e.to_string()),
  }
}

fn no_auth_api() -> Router {
    Router::new().route("/login", post(lg)) // 登录
                                               // .route("/get_captcha", get(system::get_captcha)) // 获取验证码
                                               // .route("/log_out", post(system::log_out)) // 退出登录
}
