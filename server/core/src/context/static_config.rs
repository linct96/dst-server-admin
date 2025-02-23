use config::{Config, File};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use once_cell::sync::Lazy;
use std::env;
use std::sync::mpsc::channel;
use std::{collections::HashMap, path::Path, sync::RwLock, time::Duration};

use crate::utils::path::resolve_current_exe_path;

pub enum EnumStaticConfigKey {
    SteamCmd,
    DstDedicatedServer,
    DstSave,
    DstBackup,
    DstMod,
}

impl EnumStaticConfigKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            EnumStaticConfigKey::SteamCmd => "steam_cmd",
            EnumStaticConfigKey::DstDedicatedServer => "dst_dedicated_server",
            EnumStaticConfigKey::DstSave => "dst_save",
            EnumStaticConfigKey::DstBackup => "dst_backup",
            EnumStaticConfigKey::DstMod => "dst_mod",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "steam_cmd" => Some(EnumStaticConfigKey::SteamCmd),
            "dst_dedicated_server" => Some(EnumStaticConfigKey::DstDedicatedServer),
            "dst_save" => Some(EnumStaticConfigKey::DstSave),
            "dst_backup" => Some(EnumStaticConfigKey::DstBackup),
            "dst_mod" => Some(EnumStaticConfigKey::DstMod),
            _ => None,
        }
    }
}

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

fn get_path_config() -> Vec<String> {
    let setting_path = match env::consts::OS {
        "windows" => resolve_current_exe_path("PathWindows.toml"),
        _ => resolve_current_exe_path("Path.toml"),
    };
    let mut paths = Vec::new();
    let setting_global = resolve_current_exe_path("Global.toml")
        .to_str()
        .unwrap()
        .to_string();
    let setting_path = setting_path.to_str().unwrap().to_string();

    paths.push(setting_global);
    paths.push(setting_path);
    paths
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
    for path in setting_path {
        watcher
            .watch(Path::new(path.as_str()), RecursiveMode::NonRecursive)
            .unwrap();
    }
    // watcher
    //     .watch(
    //         Path::new(setting_path.as_str()),
    //         RecursiveMode::NonRecursive,
    //     )
    //     .unwrap();

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

    let mut config = Config::builder();
    for p in setting_path {
        config = config.add_source(File::with_name(p.as_str()));
    }
    config.build().unwrap()
}

pub fn get() -> HashMap<String, String> {
    STATIC_CONFIG
        .read()
        .unwrap()
        .clone()
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
}
