use std::fmt::Debug;

use axum::{
    body::Body,
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Serialize, Debug, Default)]
pub struct Res<T> {
    pub code: i32,
    pub data: Option<T>,
    pub message: String,
}
impl<T: Serialize> Res<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message: "success".to_string(),
        }
    }
    pub fn error(message: String) -> Self {
        Self {
            code: 500,
            data: None,
            message,
        }
    }
}

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
            message: self.message,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => {
                // print!("json_string: {}", v);
                v
            }
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

#[derive(Serialize, Debug, Default)]
pub struct ResBody<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> ResBody<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            data: Some(data),
            message: "success".to_string(),
        }
    }
    pub fn err(data: T, message: String) -> Self {
        Self {
            code: 500,
            data: Some(data),
            message,
        }
    }
}

impl<T> IntoResponse for ResBody<T>
where
    T: Serialize + Send + Sync + Debug + 'static,
{
    fn into_response(self) -> Response {
        let data = Self {
            code: self.code,
            data: self.data,
            message: self.message,
        };
        let json_string = match serde_json::to_string(&data) {
            Ok(v) => v,
            Err(e) => {
                let error_response = Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static(mime::TEXT_PLAIN_UTF_8.as_ref()),
                    )
                    .body(Body::from(e.to_string()));
                return error_response.unwrap();
            }
        };
        let response = json_string.into_response();
        // response.ex
        // let res_json_string: ResJsonString = ResJsonString(json_string.clone());
        // response.extensions_mut().insert(res_json_string);
        response
    }
}
