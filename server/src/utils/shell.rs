use std::{
    env::consts::OS,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub fn run_command(content: &str) {
    if OS == "windows" {
        run_cmd_command(content);
    } else {
        run_bash_command(content);
    }
}
pub fn run_bash_command(content: &str) {
    let mut child_process = Command::new("bash")
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-c")
        .arg(content)
        .spawn()
        .expect("Failed to execute command");
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

pub fn run_cmd_command(bat_file_path: &str) {
    let output = Command::new("cmd")
        .arg("/C")
        .arg(bat_file_path)
        .output()
        .expect("Failed to execute command");
    // 检查脚本是否成功执行
    if output.status.success() {
        // 打印脚本的输出
        println!(
            "Script output:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    } else {
        // 打印脚本的错误输出
        println!(
            "Script failed with error:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
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
