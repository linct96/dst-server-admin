use anyhow::Result;
use axum::http::HeaderMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserLoginReq {
    ///  用户名
    pub user_name: String,
    ///  用户密码
    pub user_password: String,
    pub code: String,
    pub uuid: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    token: String,
    token_type: String,
    pub exp: i64,
    exp_in: i64,
}
impl AuthBody {
    fn new(access_token: String, exp: i64, exp_in: i64, token_id: String) -> Self {
        Self {
            token: access_token + &token_id,
            token_type: "Bearer".to_string(),
            exp,
            exp_in,
        }
    }
}

pub async fn login_service(login_req: UserLoginReq, header: HeaderMap) -> Result<AuthBody> {
    println!("login service");
    let auth_body = AuthBody {
        token: "token".to_string(),
        token_type: "Bearer".to_string(),
        exp: 1637612800,
        exp_in: 3600,
    };
    Ok(auth_body)
}

pub async fn login_service2() -> Result<AuthBody> {
    println!("login service");
    let auth_body = AuthBody {
        token: "token".to_string(),
        token_type: "Bearer".to_string(),
        exp: 1637612800,
        exp_in: 3600,
    };
    Ok(auth_body)
}
