use crate::api::{system, un_auth};

use axum::Router;

pub fn root() -> Router {
    
    Router::new()
        // 文件上传api
        // .nest_service(&CFG.web.upload_url, get_service(ServeDir::new(&CFG.web.upload_dir)))
        // 无需授权Api.通用模块
        .nest("/unAuth", un_auth::router_un_auth())
        .nest("/system", system::router_system())
    // 系统管理模块
    // .nest("/system", set_auth_middleware(system::system_api()))
    //  测试模块
    // .nest("/test", test_api())
}
