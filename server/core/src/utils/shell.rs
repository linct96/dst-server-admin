use colored::Colorize;
use regex::Regex;
use std::{
    default,
    env::consts::OS,
    fs,
    io::{BufRead, BufReader},
    path::Path,
    process::{Command, Stdio},
    thread,
};
use tokio::process::Command as TokioCommand;
use crate::{config::config::PathConfig, service::task::SYSTEM_INFO};
use crate::{service::task::ConstantOS, utils::file};

pub async fn execute_command(command: &str) -> anyhow::Result<Option<u32>> {
    // 创建一个 Command 对象，指定要执行的 shell 命令
    let mut child = match OS {
        "windows" => {
            let mut cm = TokioCommand::new("cmd");
            cm.arg("/C").arg(command);
            cm
        }
        _ => {
            let mut cm = TokioCommand::new("sh");
            cm.arg("-c").arg(command);
            cm
        }
    }
    .spawn()
    .expect("Failed to execute command");
    // let mut child = command.spawn().expect("Failed to execute command");

    // 等待命令执行完成
    let id: Option<u32> = child.id();
    let status = child.wait().await.expect("Failed to wait on child");

    if status.success() {
        println!("{}", "脚本执行成功".green());
    } else {
        println!("{}", "脚本执行失败".red());
        Err(anyhow::anyhow!("脚本执行失败"))?;
    }
    Ok(id)
}
pub async fn install_lib() -> anyhow::Result<()> {
    let arch = std::env::consts::ARCH;
    let system_info = SYSTEM_INFO.lock().await.clone();
    let contains_ignore_case =
        |haystack: &str, needle: &str| haystack.to_lowercase().contains(&needle.to_lowercase());
    let is_arch_64 = contains_ignore_case(&arch, "64");
    if contains_ignore_case(&system_info.os_version, "macos") {
        execute_command("ls")
            .await
            .expect("Failed to execute command");
    }
    if contains_ignore_case(&system_info.os_version, "ubuntu") {
        println!("ubuntu");
        execute_command("sudo apt-get -y install lib32gcc1")
            .await
            .expect("Failed to execute command");
        execute_command("sudo apt-get -y install screen")
            .await
            .expect("Failed to execute command");
    }
    Ok(())
}

pub fn run_command_directly(content: &str) {
    if OS == "windows" {
        run_cmd_command(content);
    } else {
        run_bash_command_directly(content);
    }
}

pub fn run_bash_command_directly(content: &str) {
    let mut child_process = Command::new("bash")
        .arg("-c")
        .arg(content)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    println!("Child process id: {}", child_process.id());

    let stdout = child_process.stdout.take().unwrap();
    let stderr = child_process.stderr.take().unwrap();

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let stdout_handle = std::thread::spawn(move || {
        for line in stdout_reader.lines() {
            println!("stdout: {}", line.expect("Failed to read line from stdout"));
        }
    });

    let stderr_handle = std::thread::spawn(move || {
        for line in stderr_reader.lines() {
            println!("stderr: {}", line.expect("Failed to read line from stderr"));
        }
    });

    let status = child_process.wait().expect("Failed to wait on child");

    stdout_handle.join().expect("Failed to join stdout thread");
    stderr_handle.join().expect("Failed to join stderr thread");
}
pub fn run_command(path: &str, args: Vec<String>) {
    if OS == "windows" {
        run_cmd_command(path);
    } else {
        run_bash_command(path, args);
    }
}
pub fn run_bash_command(path: &str, args: Vec<String>) {
    let mut command = Command::new("bash");
    command.arg(path);
    for arg in args {
        command.arg(arg);
    }

    command.stderr(Stdio::piped()).stdout(Stdio::piped());

    let mut child_process = command.spawn().unwrap();
    println!("Child process id: {}", child_process.id());

    let stdout = child_process.stdout.take().unwrap();
    let stderr = child_process.stderr.take().unwrap();

    let stdout_reader = BufReader::new(stdout);
    let stderr_reader = BufReader::new(stderr);

    let stdout_handle = std::thread::spawn(move || {
        for line in stdout_reader.lines() {
            println!("stdout: {}", line.expect("Failed to read line from stdout"));
        }
    });

    let stderr_handle = std::thread::spawn(move || {
        for line in stderr_reader.lines() {
            println!("stderr: {}", line.expect("Failed to read line from stderr"));
        }
    });

    let status = child_process.wait().expect("Failed to wait on child");

    stdout_handle.join().expect("Failed to join stdout thread");
    stderr_handle.join().expect("Failed to join stderr thread");

    println!("Script exited with status: {:?}", status);
}

pub fn run_cmd_command(bat: &str) {
    let mut child_process = Command::new("cmd")
        .args(&["/C", bat])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute command");
    let id = child_process.id();
    println!("Child process id: {}", id);
    // 获取子进程的 stdout 和 stderr
    let stdout = child_process.stdout.take().expect("Failed to open stdout");
    let stderr = child_process.stderr.take().expect("Failed to open stderr");
    // 创建线程来读取和打印 stdout
    let stdout_thread = thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("stdout: {}", line.expect("Failed to read line"));
        }
    });

    // 创建线程来读取和打印 stderr
    let stderr_thread = thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            println!("stderr: {}", line.expect("Failed to read line"));
        }
    });

    // 等待子进程完成
    let status = child_process.wait().expect("Failed to wait on child");
    println!("Batch script exited with status: {:?}", status);
    // 等待线程完成
    stdout_thread.join().expect("stdout thread panicked");
    stderr_thread.join().expect("stderr thread panicked");
}

pub fn run_command_test() {

    // # cd "/root/Steam/steamapps/common/Don't Starve Together Dedicated Server/bin"
    // # run_shared=(./dontstarve_dedicated_server_nullrenderer)
    // # run_shared+=(-console_enabled)
    // # run_shared+=(-cluster "ddd")
    // # run_shared+=(-ugc_directory "/root/Steam/steamapps/common/Don't Starve Together Dedicated Server/ugc_mods")
    // # run_shared+=(-region sing)
    // # run_shared+=(-monitor_parent_process $)
    // # run_shared+=(-shard "Forest1")
    // # "${run_shared[@]}"
}

#[derive(Debug, Clone)]
pub struct ScreenTask {
    pub pid: i32,
    pub name: String,
    pub is_attached: bool,
}
pub fn get_screen_task() -> Vec<ScreenTask> {
    let mut command = Command::new("screen");
    command.arg("-ls");
    let output = command.output().expect("Failed to execute command");
    let output_str = String::from_utf8_lossy(&output.stdout);
    let mut screen_tasks: Vec<ScreenTask> = Vec::new();
    let rgx = Regex::new(r"\s*(\d+)\.(\S+)\s*\((Detached|Attached)\)\s*").unwrap();
    for line in output_str.lines() {
        if let Some(captures) = rgx.captures(line) {
            let pid: i32 = captures[1].parse().unwrap();
            let name = captures[2].to_string();
            let is_attached = &captures[3] == "Attached";
            screen_tasks.push(ScreenTask {
                pid,
                name,
                is_attached,
            });
        }
    }

    screen_tasks
}
