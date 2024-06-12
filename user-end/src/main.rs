// 中间件
// 统计耗时
// 处理静态文件
// 日志

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
