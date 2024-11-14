mod api;
mod bootstrap;
mod config;
mod db;
mod service;
mod utils;
use std::{fs, path::Path};

use api::res::{Res, ResBody};
use axum::{http::HeaderMap, routing::get, Json, Router};

use service::s_user::{login_service, login_service2, AuthBody, UserLoginReq};
use asset::STATIC_DIR;
use utils::shell::run_cmd_command;


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

    println!("Hello, world123!");

    // 获取 resources 目录中的 config.json 文件
    // if let Some(file) = STATIC_DIR.get_file("install_macOS2.json") {
    //     // 打印文件内容
    //     println!("File contents:\n{}", file.contents_utf8().unwrap());
    // } else {
    //     println!("File not found");
    // }

    // if let Some(file) = STATIC_DIR.get_file("install_macOS2.json") {
    //     // 打印文件内容
        
    //     println!("File contents:\n{}", file.contents_utf8().unwrap());
    // } else {
    //     println!("File not found");
    // }
    if let Some(file) = STATIC_DIR.get_file("install.bat") {
        // 打印文件内'
        let file_path = file.path().to_str().unwrap();
        println!("path: {}",file_path);
        // println!("File contents:\n{}", file.contents_utf8().unwrap());
        run_cmd_command( file_path);
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
