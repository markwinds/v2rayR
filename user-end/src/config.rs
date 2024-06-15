use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{log_e, Logger, LogLevel};

fn default_log_config_level() -> LogLevel {
    LogLevel::WarningLevel
}

fn default_data_dir() -> PathBuf {
    PathBuf::from("data")
}

fn default_web_port() -> u16 {
    return 3333;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub log_config: LogConfig,

    #[serde(default = "default_data_dir")]
    pub data_dir: PathBuf, // 存放数据的路径（配置文件、日志文件除外，这两个文件默认和二进制程序放一起）

    #[serde(default = "default_web_port")]
    pub web_port: u16, // 本地web访问端口
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogConfig {
    // 可以使用字段属性宏指定名字 比如：#[serde(rename = "new_name")]
    #[serde(default = "default_log_config_level")]
    pub level: LogLevel,
}

// 提供默认配置
impl Default for Config {
    fn default() -> Self {
        Config {
            log_config: LogConfig {
                level: default_log_config_level()
            },
            data_dir: default_data_dir(),
            web_port: default_web_port(),
        }
    }
}

impl Config {
    // 从文件中读取配置
    fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }

    // 获取配置，文件不存在则使用默认配置
    pub fn load_config<P: AsRef<Path>>(path: P) -> Self {
        match Self::from_file(&path) {
            Ok(config) => config,
            Err(e) => {
                log_e!("read config failed, file[{:?}], e[{:?}]",path.as_ref(),e);
                Self::default()
            }
        }
    }

    pub fn instance() -> Arc<Mutex<Self>> {
        static INSTANCE: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| {
            let ins = Config::load_config("config.toml");
            Arc::new(Mutex::new(ins))
        });
        INSTANCE.clone()
    }
}
