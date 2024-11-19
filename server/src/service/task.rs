use std::{str::FromStr, sync::Arc};

use once_cell::sync::Lazy;
use psutil::{cpu, disk, memory};
use rust_decimal::{
    prelude::{FromPrimitive, ToPrimitive},
    Decimal,
};
use serde::Serialize;
use tokio::{
    sync::Mutex,
    time::{self, Duration},
};

#[derive(Debug, Serialize, Clone, Default)]
pub struct SystemInfo {
    pub os_version: String,
    pub cpu_count: u64,
    pub cpu_usage: f32,
    pub memory_used: f32,
    pub memory_total: f32,
    pub memory_usage: f32,
    pub disk_used: f32,
    pub disk_total: f32,
    pub disk_usage: f32,
}

pub static SYSTEM_INFO: Lazy<Arc<Mutex<SystemInfo>>> = Lazy::new(|| {
    let system_info = SystemInfo::default();
    Arc::new(Mutex::new(system_info))
});
// pub static DB: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn update_system_info() {
    // 创建一个新的 System 实例
    let mut system_info = SYSTEM_INFO.lock().await;
    let mut cpu_collector = cpu::CpuPercentCollector::new().unwrap();
    let disk_info = disk::disk_usage("/").unwrap();
    cpu_collector
        .cpu_percent()
        .expect("Failed to get CPU usage");
    time::sleep(Duration::from_secs(1)).await;
    let memory_info = memory::virtual_memory().unwrap();
    system_info.os_version = sysinfo::System::long_os_version().unwrap_or_else(|| "unknown".to_owned());
    system_info.cpu_count = cpu::cpu_count();
    system_info.cpu_usage = (cpu_collector.cpu_percent().unwrap() * 10.0).round() / 10.0;
    system_info.disk_used =
        (disk_info.used() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
    system_info.disk_total =
        (disk_info.total() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
    system_info.disk_usage = (disk_info.percent() * 10.0).round() / 10.0;
    system_info.memory_total =
        (memory_info.total() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
    system_info.memory_used =
        (memory_info.used() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
    system_info.memory_usage = (memory_info.percent() * 10.0).round() / 10.0;
}

pub async fn get_system_info() -> SystemInfo {
    let system_info = SYSTEM_INFO.lock().await;
    system_info.clone()
}
