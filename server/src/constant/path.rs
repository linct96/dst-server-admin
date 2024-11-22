use std::{
    env::consts::OS,
    path::{Path, PathBuf},
    sync::Arc,
};

use once_cell::sync::Lazy;
use tokio::sync::Mutex;

use crate::utils::file::resolve_path;

#[derive(Debug, Clone)]
pub struct PathGame {
    pub steam_cmd_path: String,
    pub steam_app_path: String,
    pub dst_save_path: String,
    pub dst_server_path: String,
    pub dst_server_bin_path: String,
    pub dst_ugc_mods_path: String,
}
pub static PATH_GAME: Lazy<Arc<Mutex<PathGame>>> = Lazy::new(|| {
    let steam_cmd_path = match OS {
        _ => dirs::home_dir().unwrap().join("steamcmd"),
    }
    .to_str()
    .unwrap()
    .to_string();

    let steam_app_path = match OS {
        "macos" => dirs::home_dir().unwrap().join(resolve_path(
            "Library/Application Support/Steam".to_string(),
        )),
        _ => dirs::home_dir().unwrap().join("Steam"),
    }
    .to_str()
    .unwrap()
    .to_string();

    let dst_save_path = match OS {
        "macos" => dirs::home_dir()
            .unwrap()
            .join(resolve_path(".klei/DoNotStarveTogether".to_string())),
        _ => dirs::home_dir().unwrap().join(resolve_path(
            "Documents/Klei/DoNotStarveTogether".to_string(),
        )),
    }
    .to_str()
    .unwrap()
    .to_string();

    let dst_server_path = match OS {
        "windows" => PathBuf::from(&steam_cmd_path).join(resolve_path(
            "steamapps/common/Don't Starve Together Dedicated Server".to_string(),
        )),
        _ => PathBuf::from(&steam_app_path).join(resolve_path(
            "steamapps/common/Don't Starve Together Dedicated Server".to_string(),
        )),
    }
    .to_str()
    .unwrap()
    .to_string();

    let dst_server_bin_path = match OS {
        "macos" => PathBuf::from(&dst_server_path).join(resolve_path(
            "dontstarve_dedicated_server_nullrenderer.app/Contents/MacOS".to_string(),
        )),
        _ => PathBuf::from(&dst_server_path).join(resolve_path("bin".to_string())),
    }
    .to_str()
    .unwrap()
    .to_string();

    let dst_ugc_mods_path = match OS {
        _ => PathBuf::from(&dst_server_path).join(resolve_path("ugc_mods".to_string())),
    }
    .to_str()
    .unwrap()
    .to_string();

    Arc::new(Mutex::new(PathGame {
        steam_cmd_path,
        steam_app_path,
        dst_save_path,
        dst_server_path,
        dst_server_bin_path,
        dst_ugc_mods_path,
    }))
});
