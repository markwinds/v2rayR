use std::fmt;

use actix_web::{error, HttpResponse, Responder};
use actix_web::http::StatusCode;
use serde::Serialize;

// 统一返回的响应结构体
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    code: i32, // 错误码
    msg: String, // 错误码对应的说明
    attach: String, // 附加描述信息
    result: Option<T>, // 实际结果数据
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(result: T) -> HttpResponse {
        let a = ApiResponse {
            code: 0,
            msg: "".to_string(),
            attach: "".to_string(),
            result: Some(result),
        };
        HttpResponse::Ok().json(a)
    }
}

// 自定义错误类型
#[derive(Debug)]
pub enum ApiError {
    ReqParamErr(String), // 携带的数据是attach信息
}

impl ApiError {
    fn to_api_resp(&self) -> ApiResponse<String> {
        match self {
            ApiError::ReqParamErr(attach) => ApiResponse {
                code: 1001,
                msg: "req param err".to_string(),
                attach: attach.clone(),
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

// 实现Responder这个接口，ApiError就可以直接在处理函数中返回
impl Responder for ApiError {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self.to_api_resp())
    }
}