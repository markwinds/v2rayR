use std::{error, thread};
use std::process::exit;

use crate::log::{Logger, LogLevel};
use crate::log_w;
use crate::utils::open_web;

// 获取鼠标悬浮在图标上时的提示字符串
fn get_tool_tip() -> String {
    let version = env!("VERSION");
    let build_time = env!("BUILD_TIME");

    format!("v2ray_r v{} build_{}", version, build_time)
}

pub fn add_tray() -> Result<(), systray::Error> {
    let mut app = systray::Application::new()?;

    app.set_icon_from_resource("IDI_ICON1")?;

    app.set_tooltip(&*get_tool_tip())?;

    app.add_menu_item("主页", |_| {
        open_web();
        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("退出", |_| {
        log_w!("exit by tray");
        exit(0);
        Ok::<_, systray::Error>(())
    })?;


    // app.add_menu_item("Print a thing", |_| {
    //     println!("Printing a thing!");
    //     Ok::<_, systray::Error>(())
    // })?;
    //
    // app.add_menu_item("Add Menu Item", |window| {
    //     window.add_menu_item("Interior item", |_| {
    //         println!("what");
    //         Ok::<_, systray::Error>(())
    //     })?;
    //     window.add_menu_separator()?;
    //     Ok::<_, systray::Error>(())
    // })?;
    //
    // app.add_menu_separator()?;
    //
    // app.add_menu_item("Quit", |window| {
    //     window.quit();
    //     Ok::<_, systray::Error>(())
    // })?;


    thread::spawn(move || {
        app.wait_for_message().unwrap();
    });
    Ok(())
}