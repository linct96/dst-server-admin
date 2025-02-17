use crate::utils::path::resolve_current_exe_path;
use config::{Config, File};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use std::sync::mpsc::channel;
use std::{
    collections::HashMap,
    env,
    path::Path,
    sync::{self, OnceLock, RwLock},
    time::Duration,
};

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

fn get_path_config() -> String {
    let setting_path = match env::consts::OS {
        "windows" => resolve_current_exe_path("PathWindows.toml"),
        _ => resolve_current_exe_path("Path.toml"),
    };
    setting_path.to_str().unwrap().to_string()
}

// fn settings() -> &'static RwLock<Config> {
//     static CONFIG: sync::OnceLock<RwLock<Config>> = OnceLock::new();
//     CONFIG.get_or_init(|| {
//         let settings = load();
//         RwLock::new(settings)
//     })
// }

// pub fn get2() -> HashMap<String, String> {
//     settings()
//         .read()
//         .unwrap()
//         .clone()
//         .try_deserialize::<HashMap<String, String>>()
//         .unwrap()
// }

static STATIC_CONFIG: Lazy<RwLock<Config>> = Lazy::new(|| {
    let settings = load_static_config();
    RwLock::new(settings)
});

fn use_static_config() -> &'static RwLock<Config> {
    &STATIC_CONFIG
}

fn refresh_static_config() {
    *use_static_config().write().unwrap() = load_static_config();
}

pub fn watch() -> ! {
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        notify::Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    let setting_path = get_path_config();
    watcher
        .watch(
            Path::new(setting_path.as_str()),
            RecursiveMode::NonRecursive,
        )
        .unwrap();

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(Ok(Event {
                kind: notify::event::EventKind::Modify(_),
                ..
            })) => {
                refresh_static_config();
            }

            Err(e) => println!("watch error: {e:?}"),

            _ => {
                // Ignore event
            }
        }
    }
}

fn load_static_config() -> Config {
    let setting_path = get_path_config();
    Config::builder()
        .add_source(File::with_name(setting_path.as_str()))
        .build()
        .unwrap()
}

pub fn get() -> HashMap<String, String> {
    STATIC_CONFIG
        .read()
        .unwrap()
        .clone()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
}
