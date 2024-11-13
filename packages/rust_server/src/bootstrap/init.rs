use axum::{routing::get, Router};

fn say() {
    println!("Hello, world!");
}

fn start_server() {
    tracing_subscriber::fmt::init();
    // build our application with a route
}



pub fn entry() {
    start_server();
}