use std::{env::consts::OS, path::PathBuf};

use crate::utils::file::resolve_path;

pub struct Config {
    pub steam_cmd_path: &'static str,
    pub dst_server_path: &'static str,
    pub dst_save_path: &'static str,
}

const steam_cmd_path: &'static str = "steamCMD";
const dst_server_path: &'static str =
    "Steam/steamapps/common/Don't Starve Together Dedicated Server";
const dst_save_path: &'static str = ".klei/DoNotStarveTogether";
impl Config {
    pub fn new() -> Self {
        let is_windows = OS == "windows";
        Self {
            steam_cmd_path,
            dst_server_path,
            dst_save_path,
        }
    }
}

pub const CONFIG_PATH: Config = Config {
    steam_cmd_path: "steamCMD",
    dst_server_path: "Steam/steamapps/common/Don't Starve Together Dedicated Server",
    dst_save_path: ".klei/DoNotStarveTogether",
};

const STEAM_CMD_PATH: &'static str = "steamcmd";
const DST_SERVER_PATH: &'static str =
    "Steam/steamapps/common/Don't Starve Together Dedicated Server";
const DST_SAVE_PATH: &'static str = "Steam/steamapps/common/Don't Starve Together Dedicated Server";

#[derive(Debug)]
pub struct PathConfig {
    pub steam_cmd_path: PathBuf,
    pub dst_server_path: PathBuf,
    pub dst_save_path: PathBuf,
}
impl PathConfig {
    pub fn new() -> Self {
        
        let home_dir = dirs::home_dir().unwrap();
        let base_steam_cmd_path = home_dir.join(resolve_path(STEAM_CMD_PATH));
        let mut base_dst_server_path = home_dir.join(resolve_path(DST_SERVER_PATH));
        let mut base_dst_save_path = home_dir.join(DST_SAVE_PATH);

        if OS == "macos" {
            println!("macos");
            base_dst_server_path = home_dir
                .join("Library/Application Support/dontstarve_dedicated_server_nullrenderer.app/Contents/MacOS")
                .join(DST_SERVER_PATH);

            base_dst_save_path = home_dir.join("Documents/Klei/DoNotStarveTogether");
            println!("{:?}", base_dst_server_path);
        }
        Self {
            steam_cmd_path: base_steam_cmd_path,
            dst_server_path: base_dst_server_path,
            dst_save_path: base_dst_save_path,
        }
    }
}
