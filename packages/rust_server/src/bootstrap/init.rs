use std::env;

use axum::{routing::get, Router};
use rusqlite::{Connection, Result};

use crate::utils::file;

// use crate::utils::file

fn say() {
    println!("Hello, world!");
}

fn start_server() {
    tracing_subscriber::fmt::init();
    // build our application with a route
}

fn init_database() -> Result<()> {
    let conn = Connection::open("cats.db");
    match conn {
        Ok(conn) => {
            println!("Connected to database");
            return Ok(());
        }
        Err(e) => {
            println!("Error connecting to database: {}", e);
            return Err(e);
        }
    }
    // conn.execute(
    //     "create table if not exists cat_colors (
    //          id integer primary key,
    //          name text not null unique
    //      )",
    //     (),
    // )?;
    // conn.execute(
    //     "create table if not exists cats (
    //          id integer primary key,
    //          name text not null,
    //          color_id integer not null references cat_colors(id)
    //      )",
    //     (),
    // )?;
    // Ok(())
}

pub fn entry() {
    let _ = init_database();
    let home_dir: Option<std::path::PathBuf> = dirs::home_dir();
    match home_dir {
        None => println!("No home directory found"),
        Some(path) => {
            println!("Home directory: {}", path.display());
            return;
        }
    }
    let is_dir = file::is_dir("./data");
    println!("is_dir: {}", is_dir);
    let config_dir_name = "rust_server";

    // println!("home_dir: {}", home_dir.unwrap().display());
    // start_server();
}
