use std::fmt::Debug;

use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct Res<T> {
    pub code: Option<i32>,
    pub data: Option<T>,
    pub msg: Option<String>,
}
/// 填入到extensions中的数据
#[derive(Debug, Clone)]
pub struct ResJsonString(pub String);

impl<T> IntoResponse for Res<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        let data = Self {
            code: self.code,
            data: self.data,
            msg: self.msg,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => {
                print!("json_string: {}", v);
                v
            },
            Err(e) => {
                print!("Error: {}", e);
                return Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )
                    .body(Body::from(e.to_string()))
                    .unwrap();
            }
        };
        let res_json_string: ResJsonString = ResJsonString(json_string.clone());
        let mut response = json_string.into_response();
        response.extensions_mut().insert(res_json_string);
        response
    }
}

impl<T: Serialize> Res<T> {
    pub fn with_data(data: T) -> Self {
        Self {
            code: Some(200),
            data: Some(data),
            msg: Some("success".to_string()),
        }
    }
    pub fn with_err(err: &str) -> Self {
        Self {
            code: Some(500),
            data: None,
            msg: Some(err.to_string()),
        }
    }
}

#[derive(Serialize, Debug, Default)]
pub struct Result<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: String,
}

impl<T: Serialize> Result<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            msg: "success".to_string(),
        }
    }
}
