// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{self, copy, BufRead, Cursor, Read, Write};
use std::process::{Command, Stdio};
use std::{fs, path};
use std::error::Error;

use axum::routing::{get, post};
use tauri::api::path::home_dir;
use tauri::api::shell;
use tauri::Manager;

const ROOT_DIR: &str = "~/";
const STEAM_CMD_DIR: &str = "steam_cmd";
const DOWNLOAD_URL_WINDOWS: &str = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
const DOWNLOAD_URL_LINUX: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz";
const DOWNLOAD_URL_MACOS: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_osx.tar.gz";

async fn download_steam_cmd_windows() -> Result<(), ()> {
    let res = reqwest::get(DOWNLOAD_URL_WINDOWS)
        .await
        .expect("Failed to download steam_cmd");
    let content = res.bytes().await.expect("Failed to read steam_cmd content");
    let mut zip_file = fs::File::create("temp.zip").expect("Failed to create steam_cmd file");
    zip_file
        .write_all(&content)
        .expect("Failed to write steam_cmd file");

    let inner_file = fs::File::open("temp.zip").expect("Failed to open steam_cmd file");
    let mut archive = zip::ZipArchive::new(inner_file).expect("Failed to open steam_cmd archive");
    for i in 0..archive.len() {
        let mut file = archive.by_index(i).expect("Failed to get steam_cmd file");
        println!("File name: {}", file.name());
        let mut output_file =
            fs::File::create(file.name()).expect("Failed to create steam_cmd file");
        io::copy(&mut file, &mut output_file).expect("Failed to copy steam_cmd file");
    }
    fs::remove_file("temp.zip").expect("Failed to remove steam_cmd file");
    Ok(())
}

fn check_steam_CMD_folder() {
    let dir_path = path::Path::new("./steamcmd");

    // if dir_path.exists() && dir_path.is_dir() {
    //     println!("文件夹存在");
    // } else {
    //     println!("文件夹不存在");
    //     // 创建文件夹
    //     fs::create_dir_all(dir_path).expect("无法创建文件夹");
    //     println!("文件夹已创建");
    // }
}
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn get_platform() {
    if cfg!(target_os = "windows") {
        println!("Windows detected");
    } else if cfg!(target_os = "macos") {
        println!("MacOS detected");
    } else if cfg!(target_os = "linux") {
        println!("Linux detected");
    } else {
        println!("Unknown platform");
    }
}

async fn download_steam_cmd_macos() -> Result<(), ()> {
    let steam_cmd_dir_path_buf = home_dir()
        .expect("Failed to get home directory")
        .join(STEAM_CMD_DIR);
    let steam_cmd_dir_path = path::Path::new(&steam_cmd_dir_path_buf);
    println!("SteamCMD folder: {}", steam_cmd_dir_path.display());
    if steam_cmd_dir_path.exists() && steam_cmd_dir_path.is_dir() {
        println!("SteamCMD is already installed");
    } else {
        println!("Installing SteamCMD...");
        fs::create_dir_all(steam_cmd_dir_path).expect("无法创建文件夹");
        println!("SteamCMD folder created");
        let mut cmd = Command::new("sh");
        let shell_command = format!(
            "curl -sqL {} | tar zxf - -C {}",
            DOWNLOAD_URL_MACOS,
            steam_cmd_dir_path.display()
        );
        cmd.arg("-c").arg(shell_command);
        let output = cmd.output().expect("failed to execute process");

        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        if output.status.success() {
            println!("SteamCMD installed successfully");
            let shell_command = format!("chmod +x {}/steamcmd.sh", steam_cmd_dir_path.display());
            cmd.arg("-c").arg(shell_command);
        } else {
            println!("Failed to install SteamCMD");
        }
    }
    Ok(())
}

fn ensure_dst_server_is_installed() {
    let dir_path: &path::Path = path::Path::new("../../dst_dedicated_server");
    if dir_path.exists() && dir_path.is_dir() {
        println!("dst is already installed");
    } else {
        println!("Installing dst...");
        fs::create_dir_all(dir_path).expect("无法创建文件夹");
        println!("dst folder created");
        let mut cmd = Command::new("sh");
        let shell_command = format!(
            "../../steamCMD/steamcmd.sh +force_install_dir {} +login anonymous +app_update 343050 validate +quit",
            dir_path.display(),
        );
        let output = cmd
            .arg("-c")
            .arg(shell_command)
            .output()
            .expect("failed to execute process");
        if output.status.success() {
            println!("dst installed successfully");
        } else {
            println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        }
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));

        // let child_stdout: std::process::Child = cmd.stdout(Stdio::piped()).spawn().unwrap();
        // let mut child_cmd = cmd.arg("-c")
        //     .arg(shell_command)
        //     .stdout(Stdio::piped())
        //     .spawn()
        //     .unwrap();
        // let child_stdout = child_cmd.stdout.take().unwrap();
        // let mut child_reader = std::io::BufReader::new(child_stdout);
        // let mut stdout_str = String::new();
        // while let Ok(_) = child_reader.read_line(&mut stdout_str) {
        //     // 进程退出后结束循环
        //     if let Ok(Some(_)) = child_cmd.try_wait() {
        //         break;
        //     }
        //     println!("{}", stdout_str);
        // }
    }
}

#[tauri::command]
async fn test_function() {
    // get_platform();
    // ensure_steam_CMD_is_installed();
    // ensure_dst_server_is_installed();
    // download_steam_cmd_windows().await.unwrap();
    download_steam_cmd_macos().await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let app: axum::Router = axum::Router::new()
        // `GET /` goes to `root`
        .route("/", get(root));
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn verify_local_server() -> Result<(), Box<dyn Error>> {
    Ok(())
}


fn setup_server<'a>(app: &'a mut tauri::App) -> Result<(), Box<dyn Error>> {
    // download_steam_cmd_macos().await.unwrap();
    tauri::async_runtime::spawn(async move {
        start_server().await.expect("Failed to start server");
    });
    let handle = app.handle();
    tauri::async_runtime::spawn(async move {
        // also added move here
        let verify_result = verify_local_server().await;
        match verify_result {
            Ok(_) => {
                println!("Local Server is running");
            }
            Err(err) => {
                handle.emit_all("local-server-down", ()).unwrap(); // changed this to handle.
                println!("Local Server is not running");
                println!("{}", err);
            }
        }
    });
    Ok(())
}
fn main() {
    #[allow(unused_mut)]
    let builder = tauri::Builder::default();
    builder
        .setup(setup_server)
        .invoke_handler(tauri::generate_handler![test_function, greet,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
