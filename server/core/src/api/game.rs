use crate::{api::res::ResBody, context::command_pool::{EnumCommand, COMMAND_POOL}, service::game::{self, GameInfo}, utils};
use asset::STATIC_DIR;
use axum::{
    http::HeaderMap, routing::{get, post}, Json, Router
};
use std::{collections::HashMap, env, fs::{self, File}, io::Read, string};

pub fn router_game() -> Router {
    Router::new()
        .route("/test_fn", get(get_running_commands))
        .route("/get_game_info", get(get_game_info))
        .route("/get_all_saves", get(get_all_saves))
        .route("/install_steam_cmd", post(install_steam_cmd))
        .route("/install_dedicated_server", post(install_dedicated_server))
        .route("/update_dedicated_server", post(update_dedicated_server))
        .route("/start_dst_server", post(start_dst_server))
        .route("/stop_dst_server", post(stop_dst_server))
        .route("/get_running_commands", post(get_running_commands))
    // 登录
}
pub async fn test_fn() -> ResBody<String> {
    // let file = STATIC_DIR.get_file("install_cmd.sh");
    // let script_content = fs::read_to_string(file).expect("Failed to read script file");
    let exe_path = env::current_exe().unwrap();
    let debug_dir = exe_path.parent().expect("无法获取可执行文件目录");
    let script_path = debug_dir.join("assets/t.bat");

    let mut file = File::open(script_path).expect("无法打开脚本文件");
    let mut script_content = String::new();
    file.read_to_string(&mut script_content).expect("无法读取脚本文件");
    println!("BAT 文件内容:\n{}", script_content);

    let result = utils::shell::execute_command(&script_content).await;

    match result {
        Ok(_) => ResBody::success(script_content),
        Err(e) => ResBody::err(script_content, e.to_string()),
    }
}

pub async fn get_running_commands() -> ResBody<HashMap<EnumCommand, u32>> {
    let command_pool = &*COMMAND_POOL;
    let commands = command_pool.get_running_commands().await;

    ResBody::success(commands)
    
}

pub async fn get_game_info() -> ResBody<GameInfo> {
    let result = game::service_get_game_info().await;

    match result {
        Ok(data) => ResBody::success(data),
        Err(e) => ResBody::err(GameInfo::default(), e.to_string()),
    }
}

pub async fn install_steam_cmd() -> ResBody<bool> {
    let result = game::service_install_steam_cmd().await;
    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}

pub async fn install_dedicated_server(header: HeaderMap, Json(req): Json<game::InstallDedicatedServerReq>) -> ResBody<u32> {
    let result = game::service_install_dedicated_server(req).await;
    match result {
        Ok(pid) => ResBody::success(pid),
        Err(e) => ResBody::err(0, e.to_string()),
    }
}


pub async fn update_dedicated_server() -> ResBody<u32> {
    let result = game::service_update_dedicated_server().await;
    match result {
        Ok(pid) => ResBody::success(pid),
        Err(e) => ResBody::err(0, e.to_string()),
    }
}

pub async fn get_all_saves() -> ResBody<Vec<game::DstSaveInfo>> {
    let result = game::service_get_all_saves().await;

    match result {
        Ok(data) => ResBody::success(data),
        Err(e) => ResBody::err(vec![], e.to_string()),
    }
}

pub async fn start_dst_server(header: HeaderMap, Json(req): Json<game::StartServerReq>) -> ResBody<bool> {
  let result = game::service_start_dst_server(req).await;

  match result {
      Ok(_) => ResBody::success(true),
      Err(e) => ResBody::err(false, e.to_string()),
  }
}

pub async fn stop_dst_server(header: HeaderMap, Json(req): Json<game::StartServerReq>) -> ResBody<bool> {
    let result = game::service_stop_dst_server(req).await;

    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}


