use crate::utils::path::resolve_current_exe_path;
use once_cell::sync::Lazy;
use std::{collections::HashMap, env};

pub enum EnumPathSettingKey {
    SteamCmd,
    DstDedicatedServer,
    DstSave,
    DstBackup,
    DstMod,
}

impl EnumPathSettingKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnumPathSettingKey::SteamCmd => "steam_cmd",
            EnumPathSettingKey::DstDedicatedServer => "dst_dedicated_server",
            EnumPathSettingKey::DstSave => "dst_save",
            EnumPathSettingKey::DstBackup => "dst_backup",
            EnumPathSettingKey::DstMod => "dst_mod",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "steam_cmd" => Some(EnumPathSettingKey::SteamCmd),
            "dst_dedicated_server" => Some(EnumPathSettingKey::DstDedicatedServer),
            "dst_save" => Some(EnumPathSettingKey::DstSave),
            "dst_backup" => Some(EnumPathSettingKey::DstBackup),
            "dst_mod" => Some(EnumPathSettingKey::DstMod),
            _ => None,
        }
    }
}

pub static PATH_SETTINGS: Lazy<HashMap<String, String>> =
    Lazy::new(|| load_path_settings().expect("Failed to load settings"));
fn load_path_settings() -> anyhow::Result<HashMap<String, String>, config::ConfigError> {
    let setting_path = match env::consts::OS {
        "windows" => resolve_current_exe_path("PathWindows.toml"),
        _ => resolve_current_exe_path("Path.toml"),
    };
    let settings = config::Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name(setting_path.to_str().unwrap()))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;
    settings.try_deserialize::<HashMap<String, String>>()
}
