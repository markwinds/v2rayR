use std::{env, process};
use std::fs::File;
use std::io::Write;
use std::process::Command;

use actix_web::{Responder, web};
use tokio::time::{Duration, sleep};
use uuid::Uuid;

use crate::{log_d, log_w, Logger, LogLevel};
use crate::service::resp::ApiResponse;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/settings")
            .route("stop", web::get().to(stop_program))
            .route("restart", web::get().to(restart_program))
            .route("now-version", web::get().to(get_now_version_parse))
    );
}

async fn get_now_version_parse() -> impl Responder {
    let version = env!("VERSION");
    let build_time = env!("BUILD_TIME");

    let res = format!("v{} build_{}", version, build_time);
    ApiResponse::success(res)
}

async fn stop_program() -> impl Responder {
    log_w!("stop program");
    tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        process::exit(0)
    });
    ApiResponse::success("")
}

async fn restart_program() -> impl Responder {
    log_w!("restart program");
    tokio::spawn(async {
        start_program_delay(1).await;
        process::exit(0)
    });
    ApiResponse::success("")
}

// 延时启动一个程序
async fn start_program_delay(time_s: u32) {
    let mut cmd_str;
    let args: Vec<String> = env::args().collect();
    let args_other: Vec<String> = env::args().skip(1).collect();

    #[cfg(target_os = "windows")]
    {
        cmd_str = String::from("start \"titleName\" ");
        cmd_str.push_str("\"");
        cmd_str.push_str(&*args[0]);
        cmd_str.push_str("\" ");
        cmd_str.push_str(&*args_other.join(" "));
    }

    #[cfg(target_os = "linux")]
    {
        cmd_str = args.join(" ");
        cmd_str.push_str(" &");
    }

    run_cmd_in_new_process(&cmd_str, time_s).await;
}

// 延时执行一条命令
async fn run_cmd_in_new_process(cmd_str: &str, time_s: u32) {
    let uuid = Uuid::new_v4();
    let uuid_str = uuid.to_string();
    let first_five_chars = &uuid_str[0..5];

    let script_name;
    let mut content;

    #[cfg(target_os = "windows")]
    {
        script_name = format!("runCmd{}.bat", first_five_chars);
        content = format!("ping -n {} 127.0.0.1\r\n", time_s);
        content.push_str(cmd_str);
        content.push_str("\r\n");
        content.push_str("del %0");
    }
    #[cfg(target_os = "linux")]
    {
        script_name = format!("runCmd{}.sh", first_five_chars);
        content = format!("#!/bin/bash\n");
        content.push_str(format!("sleep {}s\n", time_s));
        content.push_str(cmd_str);
        content.push_str("\n");
        content.push_str("rm $0");
    }

    log_d!("write script file[{}] content[{}]", script_name, content);


    if cfg!(target_os = "windows") {
        let content_gbk = encoding_rs::GBK.encode(&content).0.into_owned();
        content = String::from_utf8(content_gbk).expect("Conversion to GBK failed");
    }

    let mut file = File::create(&script_name).unwrap();
    file.write_all(content.as_bytes()).unwrap();

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .arg("/C")
            .arg(&script_name)
            .spawn().unwrap();
    }

    #[cfg(target_os = "linux")]
    {
        Command::new("chmod")
            .arg("+x")
            .arg(&script_name)
            .spawn().unwrap();

        Command::new(format!("./{}", &script_name))
            .spawn().unwrap();
    }
}