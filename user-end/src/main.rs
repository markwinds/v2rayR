// 中间件
// 统计耗时
// 日志

// 启动浏览器
// 使用data目录
// github ci配置

// 发布不同平台   tag
// 给exe签名

// 写在main.rs的第一行 指示编译器生成GUI程序而不是console程序 这样编译出来的windows程序就不会弹出黑框
#![windows_subsystem = "windows"]


use actix_web::{App, HttpServer};

use log::{Logger, LogLevel};
use middleware::req_time::ReqTime;

use crate::config::Config;
use crate::tray::add_tray;
use crate::web_dist::{dist, index};

mod log;
mod middleware;
mod utils;
mod web_dist;
mod config;
mod tray;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    add_tray().unwrap();


    set_log_level!(LogLevel::WarningLevel);

    {
        let config_ins = Config::instance();
        let config = config_ins.lock().unwrap();
        println!("read config{:?}", config);
        set_log_level!(config.log_config.level)
    }

    log_d!("debug");
    log_i!("info");
    log_w!("warn");
    log_e!("error");

    HttpServer::new(|| App::new().wrap(ReqTime).service(index).service(dist))
        .bind("0.0.0.0:3333")?
        .run()
        .await
}
