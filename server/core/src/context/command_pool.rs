use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::{env, sync::Arc};
use tokio::{
    process::Command,
    sync::{Mutex, Semaphore},
};

static COMMAND_POOL: Lazy<Arc<CommandPool>> = Lazy::new(|| {
    Arc::new(CommandPool::new(5)) // 限制同时执行 5 个命令
});

struct CommandPool {
    semaphore: Arc<Semaphore>,
    running_commands: Arc<Mutex<HashMap<String, u32>>>,
}

impl CommandPool {
    pub fn new(max_concurrent: usize) -> Self {
        CommandPool {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            running_commands: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn execute_command(&self, command: &str) -> anyhow::Result<u32> {
        let permit = self.semaphore.acquire().await?;

        let mut child = match env::consts::OS {
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
        .spawn()?;

        let pid = child
            .id()
            .ok_or_else(|| anyhow::anyhow!("Failed to get process ID"))?;

        self.running_commands
            .lock()
            .await
            .insert(pid.to_string(), pid);

        let running_commands_clone = self.running_commands.clone();

        tokio::spawn({
            let _permit = permit; // 捕获 permit，确保它在任务中保持有效
            async move {
                let _ = child.wait().await; // 等待命令完成
                running_commands_clone.lock().await.remove(&pid.to_string()); // 清理
            }
        });

        Ok(pid)
    }
    pub async fn get_running_commands(&self) -> HashMap<String, u32> {
        self.running_commands.lock().await.clone()
    }
}
