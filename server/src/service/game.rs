use std::{env::consts::OS, io::Write, path::Path};

use anyhow::{Ok, Result};
use asset::STATIC_DIR;

use ini::Ini;
use serde::Serialize;
use tempfile::NamedTempFile;
use tokio::fs;

use crate::{
    config::config::PathConfig,
    utils::{file, shell},
};

pub async fn service_force_install_dst_server() -> Result<bool> {
    remove_dst_server().await?;
    service_update_dst_server().await?;
    Ok(true)
}

pub async fn service_update_dst_server() -> Result<bool> {
    println!("login service");
    if OS == "windows" {
        return update_dst_server_windows().await;
    } else {
        return update_dst_server_linux().await;
    }
}

pub async fn service_start_dst_server() -> Result<bool> {
    let mut sh_name = "run_cluster.sh";

    if OS == "windows" {
        sh_name = "install_windows.bat";
    }
    let shell_file = STATIC_DIR.get_file(sh_name).unwrap();
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(shell_file.contents())?;
    let path_config = PathConfig::new();
    let dst_server_path = path_config.dst_server_path.to_str().unwrap();
    println!("path_config.dst_server_path: {}", dst_server_path);
    shell::run_command(
        temp_file.path().to_str().unwrap(),
        vec![
            dst_server_path.to_string(),
            "cluster_1".to_string(),
            "master".to_string(),
        ],
    );

    Ok(true)
}

#[derive(Debug, Serialize, Clone)]
pub struct DstSaveInfo {
    save_name: String,
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
  world_name: String,
}
pub async fn service_get_all_saves() -> Result<Vec<DstSaveInfo>> {
    let path_config = PathConfig::new();

    let saves_path = path_config.dst_save_path.to_str().unwrap();
    let saves = file::list_dir_with_target_file(saves_path, "cluster.ini")?;
    let result: Vec<DstSaveInfo> = saves
        .iter()
        .map(|save_name| {
          let current_save_path = format!("{}/{}", saves_path, save_name);
          let cluster_ini_path = format!("{}/cluster.ini", current_save_path);
          let worlds: Vec<String> = file::list_dir_with_target_file(&current_save_path, "server.ini").unwrap();
          let worlds_result:Vec<DstSaveWorldInfo> = worlds.iter().map(|world_name| {
            DstSaveWorldInfo {
              world_name: world_name.to_string(),
            }
          }).collect();
          let conf = Ini::load_from_file(cluster_ini_path).unwrap();
          let network_section = conf.section(Some("NETWORK")).unwrap();
          let game_play_section = conf.section(Some("GAMEPLAY")).unwrap();
          let cluster_name = network_section.get("cluster_name").unwrap();
          let cluster_description = network_section.get("cluster_description").unwrap();
          let cluster_password = network_section.get("cluster_password").unwrap();
          let game_mode = game_play_section.get("game_mode").unwrap();
          let max_players = game_play_section.get("max_players").unwrap();
          let pvp = game_play_section.get("pvp").unwrap();
            DstSaveInfo {
                save_name: save_name.to_string(),
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
async fn update_dst_server_windows() -> Result<bool> {
    let url = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
    let output_path = "./steamcmd.zip";
    let path_config = PathConfig::new();
    if Path::new(output_path).exists() {
        println!("exist steamcmd.zip");
        // fs::remove_file(output_path).await.unwrap();
    } else {
        file::download_file(url, output_path).await?;
    }
    let steam_cmd_path = path_config.steam_cmd_path.to_str().unwrap();
    file::unzip_file(output_path, steam_cmd_path).await;

    // let callback = |path: PathBuf| shell::run_command(path.to_str().unwrap(), [].to_vec());
    // let script = format!(
    //     "{}/steamcmd.exe +login anonymous +app_update 343050 validate +quit",
    //     steam_cmd_path
    // );
    // let temp_file_path = trans_content_to_file(&script, ".sh");
    Ok(true)
}

async fn update_dst_server_linux() -> Result<bool> {
    let mut sh_name = "install.sh";

    if OS == "macos" {
        sh_name = "install.sh";
    } else if OS == "windows" {
        sh_name = "install_windows.bat";
    }

    let path_config = PathConfig::new();
    let steam_cmd_path = path_config.steam_cmd_path.to_str().unwrap();
    let shell_file = STATIC_DIR.get_file(sh_name).unwrap();
    let mut temp_file = NamedTempFile::new().unwrap();
    temp_file.write_all(shell_file.contents()).unwrap();
    shell::run_command(
        temp_file.path().to_str().unwrap(),
        vec![steam_cmd_path.to_string()],
    );
    Ok(true)
}

async fn remove_dst_server() -> Result<bool> {
    let path_config = PathConfig::new();
    let steam_app_path = path_config.steam_app_path.to_str().unwrap();
    let steam_cmd_path = path_config.steam_cmd_path.to_str().unwrap();
    if Path::new(steam_app_path).exists() {
        fs::remove_dir_all(steam_app_path).await?;
    }
    if Path::new(steam_cmd_path).exists() {
        fs::remove_dir_all(steam_cmd_path).await?;
    }
    if Path::new("steamcmd.zip").exists() {
        fs::remove_file("steamcmd.zip").await?;
    }
    Ok(true)
}
