// 中间件
// 统计耗时
// 处理静态文件
// 日志

mod log;
mod middleware;
mod utils;
mod web_dist;

use crate::web_dist::{dist, index};
use actix_web::{App, HttpServer};
use log::{LogLevel, Logger};
use middleware::req_time::ReqTime;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    set_log_level!(LogLevel::WarningLevel);

    logE!("Formatted message: number = {}, string = {}", 42, "hello");
    logI!("good");

    HttpServer::new(|| App::new().wrap(ReqTime).service(index).service(dist))
        .bind("0.0.0.0:3333")?
        .run()
        .await
}
