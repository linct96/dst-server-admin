// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::File;
use std::io::{self, copy, BufRead, Cursor};
use std::process::{Command, Stdio};
use std::{fs, path};

use tauri::api::shell;

const STEAMCMD_DIR: &str = "./steamcmd";
const DOWNLOAD_URL_WINDOWS: &str = "https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip";
const DOWNLOAD_URL_LINUX: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_linux.tar.gz";
const DOWNLOAD_URL_MACOS: &str =
    "https://steamcdn-a.akamaihd.net/client/installer/steamcmd_osx.tar.gz";

#[tokio::main]
async fn download_steam_cmd_windows() -> Result<(), String> {
    let tmp_dir = tempfile::Builder::new()
        .prefix("example")
        .tempdir()
        .or(Err("Failed to create temp dir"))?;

    let res = reqwest::get(DOWNLOAD_URL_WINDOWS)
        .await
        .or(Err("Failed to download steamcmd"))?;
    
    // let mut dest = {
    //     let fname = res
    //         .url()
    //         .path_segments()
    //         .and_then(|segments| segments.last())
    //         .and_then(|name| if name.is_empty() { None } else { Some(name) })
    //         .unwrap_or("tmp.bin");

    //     println!("file to download: '{}'", fname);
    //     let fname = tmp_dir.path().join(fname);
    //     println!("will be located under: '{:?}'", fname);
    //     File::create(fname)?;
    // };
    let content = res
        .text()
        .await
        .or(Err("Failed to read steamcmd content"))?;
    let mut file = std::fs::File::create("steamcmd.zip").or(Err("Failed to create steamcmd file"))?;
    copy(&mut content.as_bytes(), &mut file).or(Err("Failed to write steamcmd file"))?;

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

#[tauri::command]
fn ensure_steam_CMD_is_installed() {
    // if cfg!(target_os = "windows") {
    //     let mut cmd = Command::new("powershell.exe");
    //     cmd.arg("-Command")
    //         .arg("Start-Process")
    //         .arg("-FilePath")
    //         .arg("https://steamcdn-a.akamaihd.net/client/installer/steamcmd.zip")
    //         .arg("-Wait")
    //         .arg("-Verb")
    //         .arg("runAs");
    //     let output = cmd.output().expect("failed to execute process");
    //     println!("{:?}", output);
    // } else {
    //     // install steamcmd on linux
    //     let mut cmd = Command::new("sh");
    //     cmd.arg("-c")
    //         .arg("curl -sqL ' https://steamcdn-a.akamaihd.net/client/installer/steamcmd_osx.tar.gz ' | tar zxvf -");
    //     let output = cmd.output().expect("failed to execute process");
    //     println!("{:?}", output);
    //     // println!("SteamCMD is only available for Windows");
    // }
    let dir_path: &path::Path = path::Path::new("../../steamCMD");
    if dir_path.exists() && dir_path.is_dir() {
        println!("SteamCMD is already installed");
    } else {
        println!("Installing SteamCMD...");
        fs::create_dir_all(dir_path).expect("无法创建文件夹");
        println!("SteamCMD folder created");
        let mut cmd = Command::new("sh");
        let shell_command = format!(
            "curl -sqL {} | tar zxf - -C {}",
            DOWNLOAD_URL_MACOS,
            dir_path.display()
        );
        cmd.arg("-c").arg(shell_command);
        let output = cmd.output().expect("failed to execute process");
        if output.status.success() {
            println!("SteamCMD installed successfully");
            let shell_command = format!("chmod +x {}/steamcmd.sh", dir_path.display());
            cmd.arg("-c").arg(shell_command);
        } else {
            println!("Failed to install SteamCMD");
        }
    }
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
fn test_function() {
    // get_platform();
    // ensure_steam_CMD_is_installed();
    // ensure_dst_server_is_installed();
    download_steam_cmd_windows();
}
fn main() {
    #[allow(unused_mut)]
    let builder = tauri::Builder::default();
    builder
        .invoke_handler(tauri::generate_handler![
            test_function,
            greet,
            ensure_steam_CMD_is_installed
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[tauri::command]
// async fn server() {
//     // initialize tracing
//     tracing_subscriber::fmt::init();

//     // build our application with a route
//     // let app = axum::Router::new();
//     // app.route("/", axum::routing::get(root));

//     // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     // let app = Router::new()
//     //     // `GET /` goes to `root`
//     //     .route("/", get(root))
//     //     // `POST /users` goes to `create_user`
//     //     .route("/users", post(create_user));

//     // // run our app with hyper, listening globally on port 3000
//     // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
//     // axum::serve(listener, app).await.unwrap();
// }
