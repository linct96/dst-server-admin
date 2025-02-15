use std::path::{Path, PathBuf};
use std::{env, fs, io, sync};

use axum::http;
use axum::response::Response;
use axum::routing::get_service;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use tower_http::services::ServeDir;

use crate::api::router::api_router;

fn resolve_path(path: &str) -> Result<PathBuf, std::io::Error> {
    let current_dir = env::current_dir()?;
    let path = Path::new(path);
    let resolved_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_dir.join(path)
    };
    Ok(resolved_path)
}

fn resolve_exe_path(path: &str) -> Result<PathBuf, io::Error> {
    let current_dir = env::current_exe()?.parent().unwrap().to_path_buf();
    let path = Path::new(path);
    let resolved_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        current_dir.join(path)
    };
    Ok(resolved_path)
}

async fn serve_index_html(path: &str) -> Response {
    let index_path = Path::new(path);
    match fs::read_to_string(index_path) {
        Ok(content) => {
            let mut response = (StatusCode::OK, content).into_response();
            response.headers_mut().insert(
                "Content-Type",
                http::HeaderValue::from_static("text/html; charset=utf-8"),
            );
            response
        }
        Err(_) => (StatusCode::NOT_FOUND, "index.html not found").into_response(),
    }
}

pub fn app_router() -> Result<Router, io::Error> {
    let index_path = resolve_exe_path("index.html")?;
    let fallback_path = index_path.clone();
    let static_dir_path = resolve_exe_path("assets")?;
    let static_service = get_service(ServeDir::new(static_dir_path.to_str().unwrap()));
    println!("static_dir_path: {:?}", static_dir_path);
    let app = Router::new()
        .nest("/api", api_router())
        .nest_service("/assets", static_service)
        .route(
            "/",
            get(|| async move { serve_index_html(index_path.to_str().unwrap()).await }),
        )
        .fallback(get(|| async move {
            serve_index_html(fallback_path.to_str().unwrap()).await
        }));

    Ok(app)
}
