mod api;
mod bootstrap;
mod config;
mod db;
mod service;
mod utils;
use std::{fs, path::Path};

use api::res::{Res, ResBody};
use axum::{http::HeaderMap, routing::get, Json, Router};
use include_dir::{include_dir, Dir};
use service::s_user::{login_service, login_service2, AuthBody, UserLoginReq};
use utils::system::SystemInfo;

static STATIC_DIR: Dir = include_dir!("static");

async fn t_login2(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> &'static str {
    "Hello, World!"
}
pub async fn t_login(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> Res<AuthBody> {
    let res = login_service(login_req, header).await;
    print!("1:");
    match res {
        Ok(x) => {
            print!("login success: {}", x.exp);
            Res::<AuthBody>::with_data(x)
        }
        Err(e) => Res::<AuthBody>::with_err(&e.to_string()),
    }
}
async fn t_login3() -> Res<AuthBody> {
    let res = login_service2().await;
    print!("1:");
    match res {
        Ok(x) => {
            print!("login success: {}", x.exp);
            Res::<AuthBody>::with_data(x)
        }
        Err(e) => Res::<AuthBody>::with_err(&e.to_string()),
    }
}

#[tokio::main]
async fn main() {
    // bootstrap::init().await;
    // bootstrap::say();
    println!("Hello, world123!");

    // 获取 resources 目录中的 config.json 文件
    if let Some(file) = STATIC_DIR.get_file("install_macOS.json") {
        // 打印文件内容
        println!("File contents:\n{}", file.contents_utf8().unwrap());
    } else {
        println!("File not found");
    }

    // 定义静态文件的路径
    // let file_path = Path::new("asset/install_macOS.json");

    // // 读取文件内容
    // let contents = fs::read_to_string(&file_path).expect("Failed to read file");

    // // 打印文件内容
    // println!("File contents:\n{}", contents);
}
