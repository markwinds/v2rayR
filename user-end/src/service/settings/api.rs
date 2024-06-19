// src/services/api.rs

use actix_web::{Responder, web};

use crate::service::resp::ApiResponse;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/settings")
            .route("now-version", web::get().to(get_now_version_parse))
    );
}

async fn get_now_version_parse() -> impl Responder {
    let version = env!("VERSION");
    let build_time = env!("BUILD_TIME");

    let res = format!("v{} build_{}", version, build_time);
    ApiResponse::success(res)
}
