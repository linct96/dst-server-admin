use crate::api::{game, system, un_auth};

use axum::{http, Router};
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn root() -> Router {
    Router::new()
        .nest(
            "/api",
            Router::new()
                .nest("/auth", auth_router())
                .nest("/unAuth", un_auth_router()),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin("*".parse::<http::HeaderValue>().unwrap())
                        .allow_methods([http::Method::GET, http::Method::POST])
                        .allow_headers([http::header::CONTENT_TYPE]),
                ),
        )

    // Router::new()
    //     // 文件上传api
    //     // .nest_service(&CFG.web.upload_url, get_service(ServeDir::new(&CFG.web.upload_dir)))
    //     // 无需授权Api.通用模块
    //     .nest("/api/unAuth", un_auth::router_un_auth())
    //     .nest("/api/auth", auth_router())
    // 系统管理模块
    // .nest("/system", set_auth_middleware(system::system_api()))
    //  测试模块
    // .nest("/test", test_api())
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
