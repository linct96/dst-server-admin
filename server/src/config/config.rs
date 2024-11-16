use std::path::{Path, PathBuf};

pub struct Config {
    pub steam_cmd_path: &'static str,
    pub dst_server_path: &'static str,
    pub dst_save_path: &'static str,
}
pub const CONFIG_PATH: Config = Config {
    steam_cmd_path: "steamCMD",
    dst_server_path: "Steam/steamapps/common/Don't Starve Together Dedicated Server",
    dst_save_path: ".klei/DoNotStarveTogether",
};
