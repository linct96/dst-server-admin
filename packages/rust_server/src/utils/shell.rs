use std::process::Command;

pub fn run_shell_command(path: &str) {
    let output = Command::new(path)
        .output()
        .expect("Failed to execute script");
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
