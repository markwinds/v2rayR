use std::process::Command;

use chrono::{Datelike, Local, Timelike};

use crate::{log_d, log_e, log_w, Logger, LogLevel};
use crate::config::Config;

pub fn get_time_str_ms() -> String {
    let now = Local::now();

    // 格式化为毫秒级别时间戳字符串
    let timestamp = format!(
        "{:04}-{:02}-{:02}_{:02}:{:02}:{:02}.{:03}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.timestamp_subsec_millis()
    );

    return timestamp;
}

pub fn get_time_s_dir() -> String {
    let now = Local::now();

    // 格式化为毫秒级别时间戳字符串
    let timestamp = format!(
        "{:04}-{:02}-{:02}_{:02}-{:02}-{:02}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    );

    return timestamp;
}

// 使用浏览器打开程序的web界面
pub fn open_web() {
    let web_url = format!("http://127.0.0.1:{}", Config::instance().lock().unwrap().web_port);

    #[cfg(target_os = "windows")]
    {
        // 打开指定网页
        let res = Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("chrome")
            .arg(web_url)
            .output();

        if let Err(e) = res {
            log_w!("open web failed, err:{}",e);
        }
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(web_url)
            .output().unwrap_or_else(|err| {
            log_w!("open web failed, err:{}",err)
        })
    }
}

