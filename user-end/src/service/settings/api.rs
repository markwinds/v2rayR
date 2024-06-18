// src/services/api.rs

use std::thread;
use std::time::Duration;

use actix_web::{HttpResponse, Responder, web};

fn users() -> impl Responder {
    thread::sleep(Duration::from_secs(2));
    HttpResponse::Ok().body("Users list")
}

async fn user_detail(path: web::Path<(u32, )>) -> impl Responder {
    HttpResponse::Ok().body(format!("User detail: {}", path.0))
}

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
    HttpResponse::Ok().body(format!("User detail: {}", ""))
}