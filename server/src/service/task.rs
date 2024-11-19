use std::sync::Arc;

use once_cell::sync::Lazy;

use rust_decimal::prelude::ToPrimitive;
use serde::Serialize;
use tokio::sync::Mutex;

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

pub async fn update_system_info() {
    // 创建一个新的 System 实例
    let mut sys = sysinfo::System::new_all();
    let disks = sysinfo::Disks::new_with_refreshed_list();
    sys.refresh_all();
    tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;

    let mut system_info = SYSTEM_INFO.lock().await;
    // let mut cpu_collector = cpu::CpuPercentCollector::new().unwrap();
    // let disk_info = disk::disk_usage("/").unwrap();
    // cpu_collector
    //     .cpu_percent()
    //     .expect("Failed to get CPU usage");
    // time::sleep(Duration::from_secs(1)).await;
    // let memory_info = memory::virtual_memory().unwrap();
    let mut disk_total: u64 = 0; // 用于累加总大小
    let mut disk_used: u64 = 0; // 用于累加已使用大小
    disks.iter().for_each(|disk| {
        disk_total += disk.total_space();
        disk_used += disk.available_space();
    });
    let cpu_usage_count: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();
    system_info.os_version =
        sysinfo::System::long_os_version().unwrap_or_else(|| "unknown".to_owned());
    system_info.cpu_count = sys.cpus().len().to_u64().unwrap();
    system_info.cpu_usage = (10.0 * cpu_usage_count / sys.cpus().len() as f32).round() / 10.0;
    system_info.disk_used = (10.0 * disk_used as f32 / (1024 * 1024 * 1024) as f32).round() / 10.0;
    system_info.disk_total =
        (10.0 * disk_total as f32 / (1024 * 1024 * 1024) as f32).round() / 10.0;
    system_info.disk_usage =
        (system_info.disk_used / system_info.disk_total * 1000.0).round() / 10.0;
    system_info.memory_total =
        (sys.total_memory() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
    system_info.memory_used =
        (sys.used_memory() as f32 / (1024 * 1024 * 1024) as f32 * 10.0).round() / 10.0;
    system_info.memory_usage =
        (system_info.memory_used / system_info.memory_total * 1000.0).round() / 10.0;
}

pub async fn get_system_info() -> SystemInfo {
    let system_info = SYSTEM_INFO.lock().await;
    system_info.clone()
}
