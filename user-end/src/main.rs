
// 中间件
// 统计耗时
// 处理静态文件
// 日志

mod web_dist;

use actix_web::{App, HttpServer};
use crate::web_dist::{dist, index};

#[actix_web::main]
async fn main() ->std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(dist))
        .bind("127.0.0.1:8000")
    ?.run()
        .await
}