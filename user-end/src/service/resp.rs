use std::fmt;

use actix_web::{error, HttpResponse, Responder};
use actix_web::http::StatusCode;
use serde::Serialize;

// 将其他错误转成标准错误的宏
#[macro_export]
macro_rules! conv_err {
    ($e:expr) => {
        |e| {
            log_e!("cache err: {}", e);
            $e(e.to_string())
        }
    };
}

// 解包Result的宏
#[macro_export]
macro_rules! unwrap_res {
    ($result:expr) => {
        match $result {
            Ok(val) => val,
            Err(err) => {
                return err.to_http_respond();
            }
        }
    };
}

// 错误码到错误信息的宏
macro_rules! create_api_response {
    ($code:expr, $msg:expr, $zh_msg:expr, $attach:expr) => {
        ApiResponse {
            code: $code,
            msg: $msg.to_string(),
            zh_msg: $zh_msg.to_string(),
            attach: $attach.clone(),
            result: None,
        }
    };
}

// json文本协议统一返回的响应结构体
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: Serialize> {
    code: i32, // 错误码
    msg: String, // 错误码对应的说明
    zh_msg: String, // 如果发生错误，返回的中文信息，web可以以弹框方式提示
    attach: String, // 附加描述信息
    result: Option<T>, // 实际结果数据
}

impl<T: Serialize> ApiResponse<T> {
    fn success(result: T) -> ApiResponse<T> {
        ApiResponse {
            code: 0,
            msg: "".to_string(),
            zh_msg: "".to_string(),
            attach: "".to_string(),
            result: Some(result),
        }
    }

    pub fn ok(result: T) -> HttpResponse {
        HttpResponse::Ok().json(Self::success(result))
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
    // 通用错误
    ReqParamErr(String), // 携带的数据是attach信息,可以附上具体的报错信息
    CreateFileErr(String),
    ExtFileErr(String),

    // settings模块相关错误
    GithubReqErr(String),
    RenameFileErr(String),
}

impl ApiError {
    fn to_api_resp(&self) -> ApiResponse<String> {
        match self {
            // 通用错误
            ApiError::ReqParamErr(attach) => create_api_response!(1001,"req param err","请求参数错误",attach),
            ApiError::CreateFileErr(attach) => create_api_response!(1002,"create file err","创建文件失败",attach),
            ApiError::ExtFileErr(attach) => create_api_response!(1003,"extra file err","解压文件失败",attach),

            // settings模块相关错误
            ApiError::GithubReqErr(attach) => create_api_response!(1101,"github req failed","向github请求失败",attach),
            ApiError::RenameFileErr(attach) => create_api_response!(1102,"rename file failed","修改程序文件名失败",attach),
        }
    }

    pub fn to_http_respond(&self) -> HttpResponse {
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