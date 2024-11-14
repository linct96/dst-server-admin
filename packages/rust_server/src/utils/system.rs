use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Serialize, Clone)]
pub struct SystemInfo {
    pub os_version: String,
    pub cpu_usage: f32,
    pub memory_usage: f64,
}

impl SystemInfo {
    pub fn get() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let cpu_usage = system.global_cpu_usage() / 100.0;
        let memory_usage = system.used_memory() as f64 / system.total_memory() as f64;

        let os_version: String =
            System::long_os_version().unwrap_or_else(|| "<unknown>".to_owned());
        return Self {
            os_version,
            cpu_usage,
            memory_usage,
        };
    }
}
