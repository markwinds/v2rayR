use std::{env, fs, process};
use std::cmp::Ordering;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, exit};

use actix_web::{HttpResponse, Responder, web};
use tokio::time::{Duration, sleep};
use uuid::Uuid;

use crate::{conv_err, log_d, log_e, log_w, Logger, LogLevel, unwrap_res};
use crate::service::resp::{ApiError, ApiResponse};
use crate::utils::extract_tar_gz;

const GITHUB_OWNER: &str = "markwinds";
const GITHUB_REPO: &str = "v2rayR";

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/settings")
            .route("stop", web::get().to(stop_program))
            .route("restart", web::get().to(restart_program))
            .route("check-and-update", web::get().to(check_and_update))
            .route("save-and-restart", web::post().to(save_and_reset))
            .route("restore-default-param", web::post().to(restore_default_param))
            .route("latest-version", web::get().to(get_latest_version))
            .route("now-version", web::get().to(get_now_version_parse))
    );
}

async fn save_and_reset() -> impl Responder {
    ""
}

async fn restore_default_param() -> impl Responder {
    ""
}

async fn get_latest_version() -> impl Responder {
    let latest_version = unwrap_res!(get_latest_release(GITHUB_OWNER, GITHUB_REPO).await);
    ApiResponse::ok(latest_version)
}

async fn get_now_version_parse() -> impl Responder {
    let version = env!("VERSION");
    let build_time = env!("BUILD_TIME");

    let res = format!("v{} build_{}", version, build_time);
    ApiResponse::ok(res)
}

async fn stop_program() -> impl Responder {
    log_w!("stop program");
    tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        process::exit(0)
    });
    ApiResponse::ok("")
}

async fn restart_program() -> impl Responder {
    log_w!("restart program");
    tokio::spawn(async {
        start_program_delay(1).await;
        process::exit(0)
    });
    ApiResponse::ok("")
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

// 检查是否有最新版本发布 并尝试更新
async fn check_and_update() -> impl Responder {
    let github_owner = GITHUB_OWNER;
    let github_repo = GITHUB_REPO;

    let latest_version = unwrap_res!(get_latest_release(github_owner, github_repo).await);
    let now_version = env!("VERSION");

    match cmp_version(now_version, &latest_version).await {
        Ordering::Less => {
            unwrap_res!(download_and_replace(github_owner,github_repo,&latest_version).await);
            log_w!("update from version:{} to {}", now_version, latest_version);
        }
        _ => {}
    }
    ApiResponse::ok("ok")
}

// 获取最新版本
async fn get_latest_release(owner: &str, repo: &str) -> Result<String, ApiError> {
    let url = format!("https://api.github.com/repos/{}/{}/releases/latest", owner, repo);
    let resp = reqwest::get(&url).await.map_err(conv_err!(ApiError::GithubReqErr))?;
    log_d!("resp:{:?}",resp);
    let json_resp = resp.json::<serde_json::Value>().await.map_err(conv_err!(ApiError::GithubReqErr))?;

    Ok(remove_leading_v(json_resp["tag_name"].as_str().unwrap()).await.to_string())
}

// 去除首字母v
async fn remove_leading_v(input: &str) -> &str {
    if let Some(first_char) = input.chars().next() {
        if first_char == 'v' || first_char == 'V' {
            return &input[1..];
        }
    }
    input
}

async fn download_latest_version(owner: &str, repo: &str, version: &str, file_name: &str) -> Result<(), ApiError> {
    let url = format!("https://github.com/{}/{}/releases/download/{}/your_binary_name", owner, repo, version);

    let response = reqwest::get(&url).await.map_err(conv_err!(ApiError::GithubReqErr))?;
    let mut file = File::create(file_name).map_err(conv_err!(ApiError::CreateFileErr))?;
    let content = response.bytes().await.map_err(conv_err!(ApiError::CreateFileErr))?;
    file.write_all(&content).map_err(conv_err!(ApiError::CreateFileErr))?;
    Ok(())
}

// 下载并替换程序
async fn download_and_replace(owner: &str, repo: &str, version: &str) -> Result<(), ApiError> {
    let file_name = "v2ray_r.tar.gz";
    download_latest_version(owner, repo, version, file_name).await?;

    // 获取当前程序的名称
    let args: Vec<String> = env::args().collect();
    let client_name = Path::new(&args[0]).file_name().unwrap().to_str().unwrap().to_string();

    // 构建旧客户端名称
    let old_client_name = format!(".{}", client_name);

    // 重命名客户端
    fs::rename(&client_name, &old_client_name).map_err(conv_err!(ApiError::RenameFileErr))?;

    // 解压最新的程序 并删除压缩包
    extract_tar_gz(file_name, &client_name).map_err(conv_err!(ApiError::ExtFileErr))?;
    fs::remove_file(file_name).unwrap();

    // 如果是linux系统 为文件添加执行权限
    #[cfg(target_os = "linux")]
    {
        Command::new("chmod")
            .arg("+x")
            .arg(client_name)
    }

    tokio::spawn(async move {
        let mut cmd = String::from("");
        // 删除旧程序
        #[cfg(target_os = "windows")]
        {
            cmd.push_str(&format!("del {}*", old_client_name))
        }
        #[cfg(target_os = "linux")]
        {
            cmd.push_str(&format!("rm {}*", old_client_name))
        }
        run_cmd_in_new_process(&cmd, 2).await; // 两秒后删除老程序文件（需要等程序退出后再删除，所以要延时）
        start_program_delay(4).await; // 4秒后重启拉起新的程序
        sleep(Duration::from_secs(1)).await; // 等待1秒后退出程序（要让程序先处理完这个请求，所以要延时退出）
        exit(0);
    });

    Ok(())
}

// 比较两个版本
async fn cmp_version(version1: &str, version2: &str) -> Ordering {
    let v1_parts: Vec<u32> = version1.split('.')
        .map(|s| s.parse().unwrap_or(0))
        .collect();
    let v2_parts: Vec<u32> = version2.split('.')
        .map(|s| s.parse().unwrap_or(0))
        .collect();

    for (v1, v2) in v1_parts.iter().zip(v2_parts.iter()) {
        match v1.cmp(v2) {
            Ordering::Equal => continue,
            non_eq => return non_eq,
        }
    }

    v1_parts.len().cmp(&v2_parts.len())
}