use axum::response::sse::{self, Event};
use futures::{FutureExt, Stream, StreamExt};
use std::{
    marker::PhantomPinned, pin::Pin, task::{Context, Poll}, time::Duration
};
use tokio::{
    io::{AsyncBufReadExt, AsyncRead, BufReader},
    process::{Child, ChildStdout, Command},
    sync::{Mutex, Semaphore},
};

use once_cell::sync::Lazy;
use serde::Serialize;

use std::collections::HashMap;
use std::{env, sync::Arc};

use tokio_util::codec::{FramedRead, LinesCodec, LinesCodecError};

struct ProcessOutput {
    reader: BufReader<ChildStdout>,
    child: Child, // 持有Child防止进程被提前回收
}

impl ProcessOutput {
    async fn new(pid: u32) -> tokio::io::Result<Self> {
        // 启动一个进程并获取其标准输出
        let mut child = match env::consts::OS {
            "windows" => {
                let mut cm = Command::new("cmd");
                cm.arg("-p").arg(pid.to_string()).arg("-o").arg("cmd=");
                cm
            }
            _ => {
                let mut cm = Command::new("sh");
                cm.arg("-p").arg(pid.to_string()).arg("-o").arg("cmd=");
                cm
            }
        }
        .stdout(std::process::Stdio::piped())
        .spawn()?;

        let stdout = child.stdout.take().ok_or_else(|| {
            tokio::io::Error::new(tokio::io::ErrorKind::Other, "Failed to capture stdout")
        })?;
        let reader = BufReader::new(stdout);

        Ok(Self { child, reader })
    }
}

impl Stream for ProcessOutput {
    type Item = Result<Event, tokio::io::Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut line = String::new();
        // let poll = self.reader.read_line(&mut line);
        // match self.reader.read_line(&mut line).poll_unpin(cx) {
        //     Poll::Ready(Ok(0)) => {
        //         // 如果读取到 0 字节，表示流结束
        //         Poll::Ready(None)
        //     }
        //     Poll::Ready(Ok(_)) => {
        //         // 成功读取一行数据
        //         let event = Event::default()
        //             .data(line.trim_end().to_string());
        //         Poll::Ready(Some(Ok(event)))
        //     }
        //     Poll::Ready(Err(e)) => {
        //         // 发生错误
        //         Poll::Ready(Some(Err(e)))
        //     }
        //     Poll::Pending => {
        //         // 还没有数据可读，返回 Pending
        //         Poll::Pending
        //     }
        // }
        let read_line = self.reader.read_line(&mut line);
        let poll_result = Box::pin(read_line).poll_unpin(cx);

        // let mut poll = self.reader.read_line(&mut line);
        // let result = Box::pin(self.reader.read_line(&mut line)).poll_unpin(cx);
        match poll_result {
            Poll::Ready(Ok(0)) => Poll::Ready(None),
            Poll::Ready(Ok(size)) =>{
                Poll::Ready(Some(Ok(Event::default().data("hello process out".to_string()))))
            },
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
        // Poll::Ready(Some(Ok(Event::default().data("hi!".to_string()))))
        // Poll::Pending
    }
}

#[derive(Serialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum EnumCommand {
    Start,
    Stop,
    InstallDedicatedServer,
    UpdateDedicatedServer,
}

impl EnumCommand {
    fn as_str(&self) -> &'static str {
        match self {
            EnumCommand::Start => "start",
            EnumCommand::Stop => "stop",
            EnumCommand::InstallDedicatedServer => "install_dedicated_server",
            EnumCommand::UpdateDedicatedServer => "update_dedicated_server",
        }
    }
}

pub static COMMAND_POOL: Lazy<Arc<CommandPool>> = Lazy::new(|| {
    Arc::new(CommandPool::new(5)) // 限制同时执行 5 个命令
});

pub struct CommandPool {
    semaphore: Arc<Semaphore>,
    running_commands: Arc<Mutex<HashMap<EnumCommand, u32>>>,
}

impl CommandPool {
    pub fn new(max_concurrent: usize) -> Self {
        CommandPool {
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            running_commands: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn execute_command(&self, key: EnumCommand, command: &str) -> anyhow::Result<u32> {
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

        self.running_commands.lock().await.insert(key.clone(), pid);

        let running_commands_clone = self.running_commands.clone();

        tokio::spawn({
            let _permit = permit; // 捕获 permit，确保它在任务中保持有效

            async move {
                let _ = child.wait().await; // 等待命令完成
                running_commands_clone.lock().await.remove(&key); // 清理
            }
        });

        anyhow::Ok(pid)
    }
    pub async fn get_running_commands(&self) -> HashMap<EnumCommand, u32> {
        self.running_commands.lock().await.clone()
    }
    pub async fn get_process_output(
        &self,
        pid: u32,
    ) -> anyhow::Result<impl Stream<Item = anyhow::Result<sse::Event, tokio::io::Error>>> {
        let process_output = ProcessOutput::new(pid).await?;
        anyhow::Ok(process_output)
    }
}
