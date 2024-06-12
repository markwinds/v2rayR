use std::fs;
use std::path::Path;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    app_name: String,
    database_url: String,
    port: u16,
}

// 提供默认配置
impl Default for Config {
    fn default() -> Self {
        Config {
            app_name: "MyApp".to_string(),
            database_url: "postgres://user:password@localhost/dbname".to_string(),
            port: 8080,
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
    fn get_config<P: AsRef<Path>>(path: P) -> Self {
        match Self::from_file(&path) {
            Ok(config) => config,
            Err(_) => {
                eprintln!("Could not read config file, using default configuration");
                Self::default()
            }
        }
    }
}

// fn main() {
//     let config_path = "config.toml";
//     let config = Config::get_config(config_path);
//     println!("{:?}", config);
// }
