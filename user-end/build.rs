use chrono::{DateTime, Utc};

use {
    std::{
        env,
        io,
    },
    winresource::WindowsResource,
};

fn main() -> io::Result<()> {
    // 获取版本信息
    let version = env::var("CARGO_PKG_VERSION").unwrap();

    // 获取当前时间并格式化为所需的格式
    let now: DateTime<Utc> = Utc::now();
    let build_time = now.format("%Y-%m-%d_%H:%M:%S").to_string();

    // 将信息写入环境变量
    println!("cargo:rustc-env=VERSION={}", version);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // 生成图标资源文件
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            // .set_icon("../web/public/favicon.ico")
            // 需要指定资源名称，使用默认的纯数字名称，系统托盘加载资源时找不到
            .set_icon_with_id("../web/public/favicon.ico", "IDI_ICON1")
            .compile()?;
    }
    Ok(())
}