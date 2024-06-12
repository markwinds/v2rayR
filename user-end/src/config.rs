use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{log_e, Logger, LogLevel};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub log_config: LogConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogConfig {
    // 可以使用字段属性宏指定名字 比如：#[serde(rename = "new_name")]
    pub level: LogLevel,
}

// 提供默认配置
impl Default for Config {
    fn default() -> Self {
        Config {
            log_config: LogConfig {
                level: LogLevel::WarningLevel
            }
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
