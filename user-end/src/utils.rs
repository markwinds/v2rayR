use chrono::{Datelike, Local, Timelike};

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
