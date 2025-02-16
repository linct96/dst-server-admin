use crate::api::res::Res;
use crate::service::s_user::{login_service, AuthBody, UserLoginReq};
use axum::{http::HeaderMap, Json};
// use super::res::{Res,Result};

pub async fn login(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> Res<AuthBody> {
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
