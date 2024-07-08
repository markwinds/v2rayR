use std::{env, io};
use std::env::VarError;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

use chrono::{Datelike, Local, Timelike};
use flate2::read::GzDecoder;
use reqwest::{Client, Proxy, Response};
use tar::Archive;

use crate::{log_w, Logger, LogLevel};
use crate::config::{Config, LogConfig};

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
            // 不指定浏览器 使用默认浏览器打开
            // .arg("chrome")
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

pub fn extract_tar_gz(tar_gz_path: &Path, output_dir: &Path) -> io::Result<()> {
    // 打开 .tar.gz 文件
    let tar_gz = File::open(tar_gz_path)?;
    // 创建 GzDecoder 以解压 gzip 部分
    let tar = GzDecoder::new(BufReader::new(tar_gz));
    // 解压 tar 文件
    let mut archive = Archive::new(tar);
    archive.unpack(output_dir)?;

    Ok(())
}

pub struct HttpClient {
    proxy_url: String, // 使用代理的地址
    client: Client,
    user_agent: String, // 请求头带上的User-Agent字段，有些服务需要带上这个头才能正常响应
}

impl Default for HttpClient {
    fn default() -> Self {
        // 优先从配置文件读取 如果配置文件中没有则从环境变量读取
        let mut proxy = Config::instance().lock().unwrap().proxy.clone();
        if proxy == "" {
            let http_proxy_res = env::var("HTTP_PROXY");
            match http_proxy_res {
                Ok(data) => {
                    proxy = data.clone();
                }
                Err(_) => {
                    let https_proxy_res = env::var("HTTPS_PROXY");
                    match https_proxy_res {
                        Ok(data) => { proxy = data.clone(); }
                        Err(_) => {}
                    }
                }
            }
        }

        let client: Client;
        if let Ok(client_proxy) = Proxy::all(&proxy) {
            client = Client::builder().proxy(client_proxy).build().unwrap()
        } else {
            client = Client::builder().build().unwrap()
        }

        HttpClient {
            proxy_url: proxy,
            client,
            user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36".to_string(), // 模拟浏览器的头
        }
    }
}

impl HttpClient {
    pub async fn get(&self, url: &str) -> Result<Response, reqwest::Error> {
        self.client.get(url)
            .header("User-Agent", &self.user_agent)
            .send().await
    }
}