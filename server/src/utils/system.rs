use psutil::{cpu, memory};
use serde::Serialize;
use std::{mem, thread, time::Duration};
use sysinfo::System;

#[derive(Debug, Serialize, Clone)]
pub struct SystemInfo {
    pub os_version: String,
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub memory_total: u64,
    pub memory_used: u64,
}

impl SystemInfo {
    pub fn get() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        let mut cpu_collector = cpu::CpuPercentCollector::new().unwrap();
        let memory_info = memory::virtual_memory().unwrap();
        cpu_collector.cpu_percent().expect("Failed to get CPU usage");
        thread::sleep(Duration::from_millis(50));
        let cpu_usage = cpu_collector.cpu_percent().unwrap();
        let memory_usage = memory_info.percent();
        let memory_total = memory_info.total();
        let memory_used = memory_info.used();
        let os_version: String =
            System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned());
        return Self {
            os_version,
            cpu_usage,
            memory_total,
            memory_used,
            memory_usage,
        };
    }
}
