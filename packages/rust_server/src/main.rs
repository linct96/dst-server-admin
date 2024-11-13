mod api;
mod bootstrap;
mod service;
use api::res::Res;
use axum::{http::HeaderMap, routing::get, Json, Router};
use service::s_user::{login_service, login_service2, AuthBody, UserLoginReq};
// use serde::{Deserialize, Serialize};

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}
async fn t_login2(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> &'static str {
    "Hello, World!"
}
pub async fn t_login(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> Res<AuthBody> {
    let res = login_service(login_req, header).await;
    print!("1:");
    match res {
        Ok(x) => {
            print!("login success: {}", x.exp);
            Res::<AuthBody>::with_data(x)
        },
        Err(e) => Res::<AuthBody>::with_err(&e.to_string()),
    }
}
async fn t_login3(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> Res<AuthBody> {
    let res = login_service2().await;
    print!("1:");
    match res {
        Ok(x) => {
            print!("login success: {}", x.exp);
            Res::<AuthBody>::with_data(x)
        },
        Err(e) => Res::<AuthBody>::with_err(&e.to_string()),
    }
}

#[tokio::main]
async fn main() {
    // bootstrap::init();
    // bootstrap::say();
    println!("Hello, world123!");
    // initialize tracing
    tracing_subscriber::fmt::init();
    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/login", get(t_login))
        .route("/login3", get(t_login3));
    // `POST /users` goes to `create_user`
    // .route("/users", post(create_user));
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
