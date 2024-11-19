use rusqlite::{config, Connection, Result};
use tokio::time::{Duration};

use crate::{
    api,
    db::db::DB,
    service::task::update_system_info,
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
        loop {
            update_system_info().await;
            tokio::time::sleep(Duration::from_secs(1)).await
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
