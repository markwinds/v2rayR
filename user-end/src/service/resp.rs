use std::fmt;

use actix_web::{error, HttpResponse, Responder};
use actix_web::http::StatusCode;
use serde::Serialize;

// json文本协议统一返回的响应结构体
#[derive(Debug, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: Serialize> {
    code: i32, // 错误码
    msg: String, // 错误码对应的说明
    zh_msg: String, // 如果发生错误，返回的中文信息，web可以以弹框方式提示
    attach: String, // 附加描述信息
    result: Option<T>, // 实际结果数据
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(result: T) -> ApiResponse<T> {
        ApiResponse {
            code: 0,
            msg: "".to_string(),
            zh_msg: "".to_string(),
            attach: "".to_string(),
            result: Some(result),
        }
    }
}

// 实现Responder接口，可以直接在处理函数中返回ApiResponse结构体
impl<T: Serialize> Responder for ApiResponse<T> {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
    }
}

// 自定义错误类型
#[derive(Debug)]
pub enum ApiError {
    ReqParamErr(String), // 携带的数据是attach信息,可以附上具体的报错信息
}

impl ApiError {
    fn to_api_resp(&self) -> ApiResponse<String> {
        match self {
            ApiError::ReqParamErr(attach) => ApiResponse {
                code: 1001,
                msg: "req param err".to_string(),
                zh_msg: "请求参数错误".to_string(),
                attach: attach.clone(),
                result: None,
            },
        }
    }

    fn to_http_respond(&self) -> HttpResponse {
        let api_resp = self.to_api_resp();
        // if api_resp.code == 0 {
        //     return HttpResponse::Ok().json(api_resp);
        // }
        HttpResponse::Ok().append_header(("Err-Code", api_resp.code)).json(api_resp)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

// 实现error::ResponseError接口 可以在处理函数返回Result时直接返回（ResponseError接口实现了Responder接口）
impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        return StatusCode::OK; // 发生错误时，http状态码还是返回200
    }

    fn error_response(&self) -> HttpResponse {
        self.to_http_respond()
    }
}

// 实现Responder这个接口，ApiError就可以直接在处理函数中返回
impl Responder for ApiError {
    type Body = actix_web::body::BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        self.to_http_respond()
    }
}