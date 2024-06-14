use {
    std::{
        env,
        io,
    },
    winresource::WindowsResource,
};

fn main() -> io::Result<()> {
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