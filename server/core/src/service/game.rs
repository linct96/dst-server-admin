use std::{
    env::consts::OS,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Ok, Result};
use asset::STATIC_DIR;

use ini::Ini;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use tokio::{fs, join};

use crate::{
    config::config::PathConfig,
    constant::{self, path::PATH_GAME},
    service::task::{ConstantOS, SYSTEM_INFO},
    utils::{file, shell},
};

pub async fn service_force_install_dst_server() -> Result<bool> {
    remove_dst_server().await?;
    service_update_dst_server().await?;
    Ok(true)
}

pub async fn service_update_dst_server() -> anyhow::Result<bool> {
    // if OS == "windows" {
    //     return update_dst_server_windows().await;
    // } else {
    //     return update_dst_server_linux().await;
    // }
    anyhow::Ok(true)
}

#[derive(Deserialize, Debug)]
pub struct StartServerReq {
    cluster: String,
    world: String,
}
pub async fn service_start_dst_server(req: StartServerReq) -> Result<bool> {
    let path_game = constant::path::PATH_GAME.lock().await.clone();
    let execute_command = match OS {
        "windows" => {
            let execute = PathBuf::from(path_game.dst_server_bin_path.clone())
                .join("dontstarve_dedicated_server_nullrenderer");
            let execute_str = execute.to_str().unwrap();
            let mut command = String::from("");
            command += &format!("cd /D {}", path_game.dst_server_bin_path);
            command += &format!(" && start dontstarve_dedicated_server_nullrenderer -console");
            command += &format!(" -cluster {} -shard {}", req.cluster, req.world);
            if Path::new(&path_game.dst_ugc_mods_path).exists() {
                command += &format!(" -ugc_directory '{}'", path_game.dst_ugc_mods_path);
            }

            command
        }
        _ => {
            let execute = PathBuf::from(path_game.dst_server_bin_path.clone())
                .join("dontstarve_dedicated_server_nullrenderer");
            let execute_str = execute.to_str().unwrap();
            let mut command = String::from("");
            command += &format!("cd \"{}\"", path_game.dst_server_bin_path);
            command += &format!(" && screen -dmS {}-{}", req.cluster, req.world);
            command += &format!(" ./dontstarve_dedicated_server_nullrenderer -console",);
            command += &format!(" -cluster {} -shard {}", req.cluster, req.world);
            if Path::new(&path_game.dst_ugc_mods_path).exists() {
                command += &format!(" -ugc_directory '{}'", path_game.dst_ugc_mods_path);
            }
            command
        }
    };
    println!("shell: {}", execute_command);
    shell::execute_command(&execute_command).await?;
    Ok(true)
}
pub async fn service_stop_dst_server(req: StartServerReq) -> Result<bool> {
    let execute_command = format!(
        "screen -S \"{}-{}\" -p 0 -X stuff \"c_shutdown(true)\\n\"",
        // "screen -S \"{}-{}\" -p 0 -X stuff $'\\003'",
        req.cluster,
        req.world
    );
    let exe = format!(
        "screen -S \"{}-{}\" -p 0 -X stuff \"c_shutdown(true)\\n\"",
        // "screen -S \"{}-{}\" -p 0 -X stuff $'\\003'",
        req.cluster,
        req.world
    );
    println!("shell: {}", execute_command);
    shell::execute_command(&execute_command).await?;
    // shell::execute_command(&execute_command).await?;

    Ok(true)
}

#[derive(Debug, Serialize, Clone)]
pub struct DstSaveInfo {
    cluster: String,
    cluster_name: String,
    cluster_description: String,
    cluster_password: String,
    game_mode: String,
    max_players: String,
    pvp: String,
    worlds: Vec<DstSaveWorldInfo>,
}
#[derive(Debug, Serialize, Clone)]
pub struct DstSaveWorldInfo {
    world: String,
}
pub async fn service_get_all_saves() -> Result<Vec<DstSaveInfo>> {
    let path_game = constant::path::PATH_GAME.lock().await.clone();
    println!("path_game: {}", &path_game.dst_save_path);
    let saves = file::list_dir_with_target_file(&path_game.dst_save_path, "cluster.ini")
        .unwrap_or_else(|_| vec![]);
    let result: Vec<DstSaveInfo> = saves
        .iter()
        .map(|save_name| {
            let current_save_path = format!("{}/{}", path_game.dst_save_path, save_name);
            let cluster_ini_path = format!("{}/cluster.ini", current_save_path);
            let worlds: Vec<String> =
                file::list_dir_with_target_file(&current_save_path, "server.ini").unwrap();
            let worlds_result: Vec<DstSaveWorldInfo> = worlds
                .iter()
                .map(|world| DstSaveWorldInfo {
                    world: world.to_string(),
                })
                .collect();
            let conf = Ini::load_from_file(cluster_ini_path).unwrap();
            let network_section = conf.section(Some("NETWORK")).unwrap();
            let game_play_section = conf.section(Some("GAMEPLAY")).unwrap();
            let cluster_name = network_section.get("cluster_name").map_or("", |p| p);
            let cluster_description = network_section.get("cluster_description").map_or("", |p| p);
            let cluster_password = network_section.get("cluster_password").map_or("", |p| p);
            let game_mode = game_play_section.get("game_mode").map_or("", |p| p);
            let max_players = game_play_section.get("max_players").map_or("", |p| p);
            let pvp = game_play_section.get("pvp").map_or("", |p| p);
            DstSaveInfo {
                cluster: save_name.to_string(),
                cluster_name: cluster_name.to_string(),
                cluster_password: cluster_password.to_string(),
                cluster_description: cluster_description.to_string(),
                game_mode: game_mode.to_string(),
                max_players: max_players.to_string(),
                pvp: pvp.to_string(),
                worlds: worlds_result,
            }
        })
        .collect();

    Ok(result)
}
// async fn update_dst_server_windows() -> Result<bool> {
//     let url = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
//     let output_path = "./steamcmd.zip";
//     let path_config = PathConfig::new();
//     let steam_cmd_path = path_config.steam_cmd_path.to_str().unwrap();
//     let steam_exe_path = format!("{}\\steamcmd.exe", steam_cmd_path);

//     if !Path::new(&steam_exe_path).exists() {
//         file::download_file(url, output_path).await?;
//         file::unzip_file(output_path, steam_cmd_path).expect("Failed to unzip file");
//         fs::remove_file(output_path).await.unwrap();
//     }

//     let script = format!(
//         "{} +login anonymous +app_update 343050 validate +quit",
//         steam_exe_path
//     );
//     println!("script: {}", script);
//     let mut temp_file = NamedTempFile::new().unwrap();
//     temp_file.write_all(script.as_bytes()).unwrap();

//     shell::run_command(&script, vec![]);

//     Ok(true)
// }

// async fn update_dst_server_linux() -> Result<bool> {
//     let mut sh_name = "install.sh";

//     if OS == "macos" {
//         sh_name = "install.sh";
//     } else if OS == "windows" {
//         sh_name = "install_windows.bat";
//     }

//     let path_config = PathConfig::new();
//     let steam_cmd_path = path_config.steam_cmd_path.to_str().unwrap();
//     let shell_file = STATIC_DIR.get_file(sh_name).unwrap();
//     let mut temp_file = NamedTempFile::new().unwrap();
//     temp_file.write_all(shell_file.contents()).unwrap();
//     shell::run_command(
//         temp_file.path().to_str().unwrap(),
//         vec![steam_cmd_path.to_string()],
//     );
//     Ok(true)
// }

async fn remove_dst_server() -> Result<bool> {
    let path_game = constant::path::PATH_GAME.lock().await.clone();
    if Path::new(&path_game.steam_app_path).exists() {
        fs::remove_dir_all(&path_game.steam_app_path).await?;
    }
    if Path::new(&path_game.steam_cmd_path).exists() {
        fs::remove_dir_all(&path_game.steam_cmd_path).await?;
    }
    if Path::new("steamcmd.zip").exists() {
        fs::remove_file("steamcmd.zip").await?;
    }
    Ok(true)
}

pub async fn service_install_steam_cmd() -> anyhow::Result<bool> {
    let system_info = SYSTEM_INFO.lock().await.clone();
    let path_game = constant::path::PATH_GAME.lock().await.clone();
    let download_file_path = Path::new("./download");
    let download_file_path_str = download_file_path.to_str().unwrap();
    let download_url = match system_info.os {
        ConstantOS::WINDOWS => "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip",
        ConstantOS::MACOS => "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_osx.tar.gz",
        _ => "http://media.st.dl.bscstorage.net/client/installer/steamcmd_linux.tar.gz",
    };
    let executor_path_buf = match system_info.os {
        ConstantOS::WINDOWS => Path::new(&path_game.steam_cmd_path)
            .to_path_buf()
            .join("steamcmd.exe"),
        _ => Path::new(&path_game.steam_cmd_path)
            .to_path_buf()
            .join("steamcmd.sh"),
    };

    if executor_path_buf.exists() {
        return anyhow::Ok(true);
    }

    // if Path::new(&path_game.steam_cmd_path).exists() {
    //     fs::remove_dir_all(&path_game.steam_cmd_path).await?;
    // }
    let file_name = file::download_file(download_url, download_file_path_str).await?;
    file::unzip_file(
        &format!("{}/{}", download_file_path_str, file_name),
        &path_game.steam_cmd_path,
    )?;
    anyhow::Ok(true)
}

pub async fn service_update_dedicated_server() -> anyhow::Result<bool> {
    let path_game = PATH_GAME.lock().await.clone();
    let execute_command = match OS {
        "windows" => {
            let execute = PathBuf::from(path_game.steam_cmd_path.clone()).join("steamcmd.exe");
            let execute_str = execute.to_str().unwrap();
            let mut command = String::from("");
            command += &format!(
                "{} +login anonymous +app_update 343050 validate +quit",
                execute_str
            );
            command
        }
        _ => {
            let execute = PathBuf::from(path_game.steam_cmd_path.clone()).join("steamcmd.sh");
            let execute_str = execute.to_str().unwrap();
            let mut command = String::from("");
            command += &format!("cd {}", path_game.steam_cmd_path);
            command += " && chmod +x steamcmd.sh";
            command += " && ./steamcmd.sh +login anonymous +app_update 343050 validate +quit";
            command
        }
    };
    println!("execute_command: {}", execute_command);
    shell::execute_command(&execute_command).await?;
    anyhow::Ok(true)
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct GameInfo {
    pub path: String,
    pub version: String,
    pub server_installed: bool,
    pub steam_cmd_installed: bool,
}
pub async fn service_get_game_info() -> anyhow::Result<GameInfo> {
    let mut game_info = GameInfo::default();
    let path_game = constant::path::PATH_GAME.lock().await.clone();

    game_info.path = path_game.dst_server_path;
    let dst_version_path = format!("{}/version.txt", &game_info.path);
    if Path::new(&game_info.path).exists() {
        let dst_version = fs::read_to_string(dst_version_path).await?;
        game_info.version = dst_version.replace("\n", "").replace("\r", "");
    }

    anyhow::Ok(game_info)
}
