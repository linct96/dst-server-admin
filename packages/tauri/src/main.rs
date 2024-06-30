// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn root() {
    return println!("Hello, World! You've been greeted from Rust!");
}

#[tauri::command]
async fn server() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = axum::Router::new();
    app.route("/", axum::routing::get(root));

    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // let app = Router::new()
    //     // `GET /` goes to `root`
    //     .route("/", get(root))
    //     // `POST /users` goes to `create_user`
    //     .route("/users", post(create_user));

    // // run our app with hyper, listening globally on port 3000
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();
}
