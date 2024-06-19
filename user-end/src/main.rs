// 统计耗时
// 日志
// panic捕捉

// 本地设置界面 + 自动检查并从github更新  github路径可配

// 写在main.rs的第一行 指示编译器生成GUI程序而不是console程序 这样编译出来的windows程序就不会弹出黑框
// #![windows_subsystem = "windows"]
#![cfg_attr(feature = "gui", windows_subsystem = "windows")]

use std::env;

use actix_web::{App, HttpServer};

use log::{Logger, LogLevel};
use middleware::req_time::ReqTime;

use crate::config::Config;
// use crate::middleware::handle_panic::handle_panic;
use crate::tray::add_tray;
use crate::utils::open_web;
use crate::web_dist::{dist, index};

mod log;
mod middleware;
mod utils;
mod web_dist;
mod config;
mod tray;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 打印堆栈信息
    env::set_var("RUST_BACKTRACE", "1");

    set_log_level!(LogLevel::WarningLevel);

    #[cfg(not(debug_assertions))]
    open_web();

    {
        let config_ins = Config::instance();
        let config = config_ins.lock().unwrap();
        log_w!("read config{:?}", config);
        set_log_level!(config.log_config.level)
    }

    add_tray().unwrap();

    HttpServer::new(|| App::new()
        // 越往下越外层
        .configure(service::init)
        // .wrap(ReqTime)
        // .wrap_fn(handle_panic)
        .service(index)
        .service(dist))
        .bind("127.0.0.1:3333")?
        .run()
        .await
}
