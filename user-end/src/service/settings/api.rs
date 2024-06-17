// src/services/api.rs

use std::thread;
use std::time::Duration;

use actix_web::{HttpResponse, Responder, web};

async fn users() -> impl Responder {
    thread::sleep(Duration::from_secs(2));
    HttpResponse::Ok().body("Users list")
}

async fn user_detail(path: web::Path<(u32, )>) -> impl Responder {
    HttpResponse::Ok().body(format!("User detail: {}", path.0))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/settings")
            .route("/save-and-reset", web::get().to(users))
            .route("/users/{id}", web::get().to(user_detail)),
    );
}
