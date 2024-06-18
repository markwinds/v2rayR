// src/services/api.rs

use actix_web::{HttpResponse, Responder, web};

use crate::service::resp::ApiError::ERR1;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/settings")
            .route("test", web::get().to(test_1))
            .route("now-version", web::get().to(get_now_version_parse))
        // .route("save-and-reset", web::get().to(users))
        // .route("users/{id}", web::get().to(user_detail)),
    );
}

async fn get_now_version_parse() -> impl Responder {
    let version = env!("VERSION");
    let build_time = env!("BUILD_TIME");

    HttpResponse::Ok().body(version.to_owned() + build_time)
}

async fn test_1() -> impl Responder {
    // Ok::<HttpResponse, E>(ApiResponse::success("okkkkkkkk"))
    // HttpResponse::Ok().body(format!("User detail: {}", ""))

    ERR1
}
