use crate::api::{game, system, un_auth};

use axum::{http, Router};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn api_router() -> Router {
    Router::new()
        .nest("/auth", auth_router())
        .nest("/unAuth", un_auth_router())
}

fn auth_router() -> Router {
    Router::new()
        .nest("/system", system::router_system())
        .nest("/game", game::router_game())
}

fn un_auth_router() -> Router {
    Router::new()
        // 文件上传api
        // .nest_service(&CFG.web.upload_url, get_service(ServeDir::new(&CFG.web.upload_dir)))
        // 无需授权Api.通用模块
        .nest("/login", un_auth::router_un_auth())
}

