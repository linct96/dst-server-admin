mod api;
mod bootstrap;
mod config;
mod db;
mod service;
mod utils;
mod constant;
use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use api::res::{Res, ResBody};
use axum::{http::HeaderMap, routing::get, Json, Router};

use asset::STATIC_DIR;
use service::s_user::{login_service, login_service2, AuthBody, UserLoginReq};
use tempfile::{Builder, NamedTempFile};
use utils::shell::{get_screen_task, run_bash_command, run_cmd_command};

async fn t_login2(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> &'static str {
    "Hello, World!"
}
pub async fn t_login(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> Res<AuthBody> {
    let res = login_service(login_req, header).await;
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

pub fn create_temp_file(content: &str) -> io::Result<PathBuf> {
    // 使用 Builder 模式创建持久化的临时文件
    let mut temp_file = Builder::new()
        .prefix("temp_file_")
        .rand_bytes(5)
        .tempfile_in("temp")?;

    // 将内容写入临时文件
    temp_file.write_all(content.as_bytes())?;

    // 获取临时文件的路径并返回
    let temp_file_path = temp_file.path().to_path_buf();
    // let contents = fs::read_to_string(&temp_file_path).expect("Failed to read file");
    // 打印文件内容
    // println!("File contents:\n{}", contents);
    Ok(temp_file_path)
}

#[tokio::main]
async fn main() {
    // let temp_file_path = create_temp_file("Hello, Rust!").unwrap();
    // println!("Temp file path: {:?}", temp_file_path);
    // 读取文件内容
    // let contents = fs::read_to_string(&temp_file_path).expect("Failed to read file");
    // 打印文件内容
    // println!("File contents:\n{}", contents);
    println!("Hello, world12!");
    
    bootstrap::init().await;

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
    // let file_name = "install_linux.sh";
    // // let file_name = "test.sh";
    // if let Some(file) = STATIC_DIR.get_file(file_name) {
    //     // 打印文件内'
    //     let file_path = file.path().to_str().unwrap();
    //     println!("path: {}", file_path);
    //     let content = file.contents_utf8().unwrap();
    //     let current_exe_path = std::env::current_exe().expect("Failed to get current executable path");
    //     // env::current_exe().unwrap().parent().unwrap().display().to_string();
    //     // println!("current_exe: {}", current_exe_path.to_str().unwrap());
    //     // println!("File contents:\n{}", file.contents_utf8().unwrap());
    //     run_bash_command(content);
    // } else {
    //     println!("File not found");
    // }

    // 定义静态文件的路径
    // let file_path = Path::new("asset/install_macOS.json");

    // // 读取文件内容
    // let contents = fs::read_to_string(&file_path).expect("Failed to read file");

    // // 打印文件内容
    // println!("File contents:\n{}", contents);
}
