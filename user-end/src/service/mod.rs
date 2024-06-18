use actix_web::web;

pub mod settings;
mod resp;

pub fn init(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(
        web::scope("/v2ray-r") // 所有请求的base路径
            .configure(settings::api::init)
        // 继续添加其他模块的路由
    );
}