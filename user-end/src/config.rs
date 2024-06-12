// use std::fs;
// use std::path::Path;
// use std::sync::{Arc, Mutex};
//
// use log::Log;
// use once_cell::sync::Lazy;
// use serde::Deserialize;
//
// use crate::{log_e, Logger, LogLevel};
//
// #[derive(Deserialize, Debug)]
// pub struct Config {
//     log_config: LogConfig,
// }
//
// #[derive(Deserialize, Debug)]
// pub struct LogConfig {
//     // 可以使用字段属性宏指定名字 比如：#[serde(rename = "new_name")]
//     level: LogLevel,
// }
//
// // 提供默认配置
// impl Default for Config {
//     fn default() -> Self {
//         Config {
//             log_config: LogConfig {
//                 level: LogLevel::WarningLevel
//             }
//         }
//     }
// }
//
// impl Config {
//     // 从文件中读取配置
//     fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
//         let contents = fs::read_to_string(path)?;
//         let config: Config = toml::from_str(&contents)?;
//         Ok(config)
//     }
//
//     // 获取配置，文件不存在则使用默认配置
//     pub fn load_config<P: AsRef<Path>>(path: P) -> Self {
//         match Self::from_file(&path) {
//             Ok(config) => config,
//             Err(_) => {
//                 log_e!("read config failed, file:{:?}",path.as_ref());
//                 Self::default()
//             }
//         }
//     }
//
//     pub fn instance() -> Arc<Mutex<Self>> {
//         static INSTANCE: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| {
//             let ins = load_config("config.toml");
//             Arc::new(Mutex::new(ins))
//         });
//         INSTANCE.clone()
//     }
// }
//
// // fn main() {
// //     let config_path = "config.toml";
// //     let config = Config::get_config(config_path);
// //     println!("{:?}", config);
// // }
