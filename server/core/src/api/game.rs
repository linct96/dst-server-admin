use crate::{
    api::res::ResBody,
    context::{
        self,
        command_pool::{EnumCommand, COMMAND_POOL},
        static_config,
    },
    service::game::{self, CreateSaveReq, GameInfo},
    utils::{self, file::SetupMods},
};
use futures::stream::{self, Stream};

use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

use asset::STATIC_DIR;
use axum::response::sse::{Event, Sse};
use axum::{
    extract::Path,
    http::HeaderMap,
    response::sse,
    routing::{get, post},
    Json, Router,
};
use std::{convert::Infallible, path::PathBuf, time::Duration};

use std::{
    any,
    collections::HashMap,
    env,
    fs::{self, File},
    io::Read,
    string,
};

use super::res::Res;

pub fn router_game() -> Router {
    Router::new()
        .route("/test_fn", get(test_fn))
        .route("/get_game_info", get(get_game_info))
        .route("/get_all_saves", get(get_all_saves))
        .route("/get_all_mods", get(get_all_mods))
        .route("/add_mods", post(add_mods))
        .route("/delete_mods", post(delete_mods))
        .route("/edit_save", post(edit_save))
        .route("/install_dedicated_server", post(install_dedicated_server))
        .route("/update_dedicated_server", post(update_dedicated_server))
        .route("/get_running_commands", post(get_running_commands))
        .route("/install_steam_cmd", post(install_steam_cmd))
        .route("/start_dst_server", post(start_dst_server))
        .route("/stop_dst_server", post(stop_dst_server))
        // .route("/get_process_output/:pid", post(get_process_output))
        .route("/sse_handler", get(sse_handler))
    // 登录
}
pub async fn test_fn() -> ResBody<bool> {
    // let file = STATIC_DIR.get_file("install_cmd.sh");
    // let script_content = fs::read_to_string(file).expect("Failed to read script file");
    let ov = context::static_config::get();
    println!("static_config: {:#?}", ov);
    ResBody::success(true)
}

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    println!("connected");
    let command_pool = &*COMMAND_POOL;
    let commands = command_pool.get_running_commands().await;
    let pid = commands
        .get(&EnumCommand::UpdateDedicatedServer.as_str().to_string())
        .unwrap();
    let process_output_stream = command_pool.get_process_output(*pid).await.unwrap();
    // A `Stream` that repeats an event every second
    //
    // You can also create streams from tokio channels using the wrappers in
    // https://docs.rs/tokio-stream
    let st = stream::repeat_with(|| Event::default().data("hi!"));
    // let s = st.map(Ok);

    let r = tokio_stream::iter([Event::default().data("hi!")]);

    Sse::new(process_output_stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

// pub async fn get_process_output(Path(pid): Path<u32>) -> impl axum::response::IntoResponse {
//     let command_pool = &*COMMAND_POOL;
//     let result = command_pool.get_process_output(pid).await;

//     match result {
//         Ok(stream) => {
//             let out = stream.map(|item| match item {
//                 Ok(line) => sse::Event::default().data(line),
//                 Err(e) => sse::Event::default().data(format!("Error: {}\n", e)),
//             });

//             // let mut sse_stream = sse::Sse::new(out);

//             ()
//         }
//         Err(_) => (),
//     }
// }
pub async fn get_running_commands() -> ResBody<HashMap<String, u32>> {
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

pub async fn install_dedicated_server(
    header: HeaderMap,
    Json(req): Json<game::InstallDedicatedServerReq>,
) -> ResBody<u32> {
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

pub async fn get_all_saves() -> Res<Vec<game::DstSaveInfo>> {
    let result = game::service_get_all_saves().await;

    match result {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e.to_string()),
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ResSetupMods {
    mods_collection: Vec<String>,
    mods: Vec<String>,
}
pub async fn get_all_mods() -> Res<ResSetupMods> {
    let result = game::service_get_all_mods().await;
    let result = result.map(|data| ResSetupMods {
        mods_collection: data.mods_collection.iter().map(|m| m.to_string()).collect(),
        mods: data.mods.iter().map(|m| m.to_string()).collect(),
    });

    match result {
        Ok(data) => Res::success(data),
        Err(e) => Res::error(e.to_string()),
    }
}

#[derive(Deserialize, Debug)]
pub struct AddModsReq {
    mods: Vec<String>,
}
pub async fn add_mods(_: HeaderMap, Json(req): Json<AddModsReq>) -> Res<bool> {
    let mods = req
        .mods
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let result = game::service_add_mods(mods).await;

    match result {
        Ok(()) => Res::success(true),
        Err(e) => Res::error(e.to_string()),
    }
}

#[derive(Deserialize, Debug)]
pub struct DeleteModsReq {
    mods: Vec<String>,
}
pub async fn delete_mods(_: HeaderMap, Json(req): Json<AddModsReq>) -> Res<bool> {
    let mods = req
        .mods
        .into_iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let result = game::service_delete_mods(mods).await;

    match result {
        Ok(()) => Res::success(true),
        Err(e) => Res::error(e.to_string()),
    }
}

pub async fn edit_save(_: HeaderMap, Json(req): Json<CreateSaveReq>) -> Res<bool> {
    let result = game::service_edit_save(req).await;

    match result {
        Ok(()) => Res::success(true),
        Err(e) => Res::error(e.to_string()),
    }
}

pub async fn start_dst_server(
    header: HeaderMap,
    Json(req): Json<game::StartServerReq>,
) -> ResBody<u32> {
    let result = game::service_start_dst_server(req).await;

    match result {
        Ok(pid) => ResBody::success(pid),
        Err(e) => ResBody::err(0, e.to_string()),
    }
}

pub async fn stop_dst_server(
    header: HeaderMap,
    Json(req): Json<game::StartServerReq>,
) -> ResBody<bool> {
    let result = game::service_stop_dst_server(req).await;

    match result {
        Ok(_) => ResBody::success(true),
        Err(e) => ResBody::err(false, e.to_string()),
    }
}

pub async fn get_current_running_save_info() -> ResBody<bool> {
    let static_config = context::static_config::get();
    let save_name = static_config.get("current_save");
    ResBody::success(true)
}
