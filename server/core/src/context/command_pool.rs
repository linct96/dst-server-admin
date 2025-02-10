use once_cell::sync::Lazy;
use std::{env::consts::OS, sync::{Arc, Mutex}};
use tokio::{process::Command, sync::Semaphore};
use std::collections::HashMap;

pub static COMMAND_POOL: Lazy<Mutex<CommandPool>> = Lazy::new(|| {
    Mutex::new(CommandPool::new(5)) // 限制同时执行 5 个命令
});

pub struct CommandPool {
    semaphore: Arc<Semaphore>,
    running_commands: Arc<Mutex<HashMap<u32, String>>>,
}

impl CommandPool {
    pub fn new(max_concurrent: usize) -> Self {
        CommandPool {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            running_commands: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn execute_command(&self, command: &str) -> anyhow::Result<Option<u32>> {
        let permit = self.semaphore.acquire().await?;
        let child = match OS {
            "windows" => {
                let mut cm = Command::new("cmd");
                cm.arg("/C").arg(command);
                cm
            }
            _ => {
                let mut cm = Command::new("sh");
                cm.arg("-c").arg(command);
                cm
            }
        }
        .spawn()
        .expect("Failed to execute command");

        
        let id: Option<u32> = child.id();
        if let Some(id) = id {
            // 将命令 ID 和命令字符串存储到 HashMap 中
            self.running_commands.lock().unwrap().insert(id, command.to_string());
        }

        // 释放许可
        drop(permit);

        Ok(id)
    }

    pub fn get_running_commands(&self) -> Vec<u32> {
        let commands = self.running_commands.lock().unwrap();
        commands.keys().cloned().collect() // 返回所有正在执行的命令 ID
    }

}