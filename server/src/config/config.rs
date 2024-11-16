use std::path::{Path, PathBuf};

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
        let is_windows = cfg!(target_os = "windows");
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
