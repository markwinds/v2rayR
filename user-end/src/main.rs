// 中间件
// 统计耗时
// 处理静态文件
// 日志

// 启动浏览器
// tray图标

// 写在main.rs的第一行 指示编译器生成GUI程序而不是console程序 这样编译出来的windows程序就不会弹出黑框
#![windows_subsystem = "windows"]

use std::path::Path;

use actix_web::{App, HttpServer};
use tray_item::{IconSource, TrayItem};

use log::{Logger, LogLevel};
use middleware::req_time::ReqTime;

use crate::config::Config;
use crate::web_dist::{dist, index};

mod log;
mod middleware;
mod utils;
mod web_dist;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // 创建一个 TrayItem 对象，并设置图标和标题
    let mut tray = TrayItem::new("Tray Example", IconSource::RawIcon()).unwrap();

    // 创建菜单项
    tray.add_menu_item("Show Message", || {
        println!("Menu item clicked!");
    }).unwrap();

    // // 创建退出菜单项
    // let (tx, rx) = mpsc::channel();
    // tray.add_menu_item("Exit", move || {
    //     tx.send(()).unwrap();
    // }).unwrap();
    //
    // // 创建一个线程来处理退出事件
    // thread::spawn(move || {
    //     rx.recv().unwrap();
    //     std::process::exit(0);
    // });


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
