// 中间件
// 统计耗时
// 处理静态文件
// 日志

// 写在main.rs的第一行 指示编译器生成GUI程序而不是console程序 这样编译出来的windows程序就不会弹出黑框
# ![windows_subsystem = "windows"]

// 根据不同的平台选择编译
// #[cfg(target_os = "windows")]
// {
// windows_specific_function();
// }
//
// #[cfg(target_os = "linux")]
// {
// linux_specific_function();
// }
//
// #[cfg(target_os = "macos")]
// {
// macos_specific_function();
// }

// 根据debug和release选择编译
// #[cfg(debug_assertions)]
// {
// println!("Running in debug mode");
// }
//
// #[cfg(not(debug_assertions))]
// {
// println!("Running in release mode");
// }

use actix_web::{App, HttpServer};

use log::{Logger, LogLevel};
use middleware::req_time::ReqTime;

use crate::config::Config;
// use crate::config::Config;
use crate::web_dist::{dist, index};


mod log;
mod middleware;
mod utils;
mod web_dist;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

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
