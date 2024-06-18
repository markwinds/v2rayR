use std::fmt;

use actix_web::{error, HttpResponse};
use actix_web::http::StatusCode;
use serde::Serialize;

// 统一返回的响应结构体
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    code: i32,
    msg: String,
    result: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(result: T) -> HttpResponse {
        let a = ApiResponse {
            code: 0,
            msg: "".to_string(),
            result: Some(result),
        };
        HttpResponse::Ok().json(a)
    }
}

// 自定义错误类型
#[derive(Debug)]
enum ApiError {
    Ok,
    ERR1,
    ERR2,
}

impl ApiError {
    fn to_api_resp(&self) -> ApiResponse<String> {
        match self {
            ApiError::Ok => ApiResponse {
                code: 1001,
                msg: "ddd".to_string(),
                result: None,
            },
            ApiError::ERR1 => ApiResponse {
                code: 1002,
                msg: "ddddddd".to_string(),
                result: None,
            },
            ApiError::ERR2 => ApiResponse {
                code: 1003,
                msg: "ddddddddddd".to_string(),
                result: None,
            },
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        return StatusCode::OK; // 发生错误时，http状态码还是返回200
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(self.to_api_resp())
    }
}