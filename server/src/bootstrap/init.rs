use rusqlite::{config, Connection, Result};
use tokio::time::{interval, Duration};

use crate::{
    api,
    db::db::DB,
    service::task::{SYSTEM_INFO},
    utils::file::{self, create_dir},
};

fn init_database() -> Result<()> {
    let db = DB::new().unwrap();
    db.conn.execute("create table if not exists cat_colors ( id integer primary key, name text not null unique )", ())?;
    db.conn.execute("create table if not exists cats ( id integer primary key, name text not null, color_id integer not null references cat_colors(id) )", ())?;
    Ok(())
}

fn init_config() {
    let home_dir: Option<std::path::PathBuf> = dirs::home_dir();
    match home_dir {
        None => {
            create_dir(home_dir.unwrap().join(".rust_server").to_str().unwrap());
        }
        Some(home_dir) => {
            let config_dir = home_dir.join(".rust_server");
            let config_dir_path = config_dir.to_str().unwrap();
            if !file::is_dir(config_dir_path) {
                create_dir(config_dir_path);
            }
        }
    }
}

async fn init_server() {
    tracing_subscriber::fmt::init();
    let port = "9527";
    let router = api::route::root();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("Server started on http://localhost:{}", port);
    axum::serve(listener, router).await.unwrap();
}

async fn init_periodic_task() {
    tokio::spawn(async {
        let mut sys = sysinfo::System::new_all();
        let disks = sysinfo::Disks::new_with_refreshed_list();
        let mut interval = interval(Duration::from_secs(1)); // 每 1 秒执行一次

        loop {
            sys.refresh_all();
            tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
            let mut system_info = SYSTEM_INFO.lock().await;

            let mut disk_total: u64 = 0; // 用于累加总大小
            let mut disk_used: u64 = 0; // 用于累加已使用大小
            disks.iter().for_each(|disk| {
                disk_total += disk.total_space();
                disk_used += disk.total_space() - disk.available_space();
            });
            let cpu_usage_count: f32 = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).sum();
            system_info.os_version =
                sysinfo::System::long_os_version().unwrap_or_else(|| "unknown".to_owned());
            system_info.cpu_count = sys.cpus().len() as u8;
            system_info.cpu_usage =
                (10.0 * cpu_usage_count / sys.cpus().len() as f32).round() / 10.0;
            system_info.disk_used =
                (10.0 * disk_used as f32 / (1024 * 1024 * 1024) as f32).round() / 10.0;
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
            interval.tick().await;
            // update_system_info(sys, disks).await;
        }
    });
}

pub async fn entry() {
    init_periodic_task().await;
    init_config();
    init_database().expect("Failed to initialize database");
    init_server().await;

    // println!("home_dir: {}", home_dir.unwrap().display());
    // start_server();
}
