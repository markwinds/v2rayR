use std::fmt::Arguments;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::panic;
use std::path::Path;
use std::sync::{Arc, Mutex};

use backtrace::Backtrace;
use flate2::Compression;
use flate2::write::GzEncoder;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::log::LogLevel::ErrorLevel;
use crate::utils::{get_time_s_dir, get_time_str_ms};

const LOG_FILENAME: &str = "v2rayR.log";
const MAX_LOG_FILESIZE: u64 = 10 * 1024 * 1024;

// 需要使用macro_export导出宏 才能在其他模块使用
#[macro_export]
macro_rules! log_d {
    ($($arg:tt)*) => {{
        let file = std::file!();
        let line = std::line!();
        Logger::instance().lock().unwrap().log(LogLevel::DebugLevel, format_args!($($arg)*), file, line);
    }};
}

#[macro_export]
macro_rules! log_i {
    ($($arg:tt)*) => {{
        let file = std::file!();
        let line = std::line!();
        Logger::instance().lock().unwrap().log(LogLevel::InfoLevel, format_args!($($arg)*), file, line);
    }};
}

#[macro_export]
macro_rules! log_w {
    ($($arg:tt)*) => {{
        let file = std::file!();
        let line = std::line!();
        Logger::instance().lock().unwrap().log(LogLevel::WarningLevel, format_args!($($arg)*), file, line);
    }};
}

#[macro_export]
macro_rules! log_e {
    ($($arg:tt)*) => {{
        let file = std::file!();
        let line = std::line!();
        Logger::instance().lock().unwrap().log(LogLevel::ErrorLevel, format_args!($($arg)*), file, line);
    }};
}

#[macro_export]
macro_rules! set_log_level {
    ($level:expr) => {{
        Logger::instance().lock().unwrap().set_level($level);
    }};
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Serialize, Deserialize)]
pub enum LogLevel {
    DebugLevel,
    InfoLevel,
    WarningLevel,
    ErrorLevel,
}

pub struct Logger {
    file: BufWriter<File>,
    level: LogLevel,
}

impl Logger {
    fn new<P: AsRef<Path>>(file_path: P, level: LogLevel) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;
        let buf_writer = BufWriter::new(file);
        Ok(Logger {
            file: buf_writer,
            level,
        })
    }

    pub fn instance() -> Arc<Mutex<Self>> {
        static INSTANCE: Lazy<Arc<Mutex<Logger>>> = Lazy::new(|| {
            Logger::save_panic_info();
            let logger =
                Logger::new(LOG_FILENAME, LogLevel::DebugLevel).expect("Failed to create logger");
            Arc::new(Mutex::new(logger))
        });
        INSTANCE.clone()
    }

    pub fn save_panic_info() {
        panic::set_hook(Box::new(|info| {
            let file_path = LOG_FILENAME;

            // 打开文件并设置为追加模式
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .expect("Could not open panic log file");

            // 写入 panic 信息
            write!(file, "\nPanic occurred: {}\n", info).expect("Could not write to panic log file");
        }));
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    // 从堆栈信息中过滤出本工程的堆栈 并最多展示10层堆栈
    pub fn get_backtrace_str(bt: &Backtrace) -> String {
        const MAX_FRAMES: usize = 10;
        let mut count = 0;
        let mut res = String::new();

        for frame in bt.frames() {
            if count >= MAX_FRAMES {
                break;
            }

            for symbol in frame.symbols().iter() {
                if let Some(file) = symbol.filename() {
                    if let Some(line) = symbol.lineno() {
                        // 仅显示本工程内的堆栈条目
                        if file.starts_with(env!("CARGO_MANIFEST_DIR")) {
                            res += &format!("    {} {}\n", file.display(), line);
                            count += 1;
                        }
                    }
                }
            }
        }

        res
    }

    pub fn log(&mut self, level: LogLevel, args: Arguments, file: &str, line: u32) {
        if level >= self.level {
            let level_str = match level {
                LogLevel::DebugLevel => "D",
                LogLevel::InfoLevel => "I",
                LogLevel::WarningLevel => "W",
                LogLevel::ErrorLevel => "E",
            };

            let output_color = match level {
                LogLevel::DebugLevel => Color::Blue,
                LogLevel::InfoLevel => Color::White,
                LogLevel::WarningLevel => Color::Yellow,
                LogLevel::ErrorLevel => Color::Red,
            };

            // Check file size
            if let Ok(metadata) = self.file.get_ref().metadata() {
                if metadata.len() > MAX_LOG_FILESIZE {
                    let log_file = LOG_FILENAME;
                    let archive_file = get_time_s_dir() + ".gz";
                    self.file.flush().unwrap();
                    let input = File::open(log_file).unwrap();
                    let output = File::create(archive_file).unwrap();
                    let mut encoder = GzEncoder::new(output, Compression::default());
                    io::copy(&mut &input, &mut encoder).unwrap();
                    encoder.finish().unwrap();

                    // Clear the original log file
                    OpenOptions::new()
                        .write(true)
                        .truncate(true)
                        .open(log_file)
                        .unwrap();

                    // Reinitialize the file writer
                    let file = OpenOptions::new()
                        .create(true)
                        .write(true)
                        .append(true)
                        .open(log_file)
                        .unwrap();
                    self.file = BufWriter::new(file);
                }
            }

            // 如果是大于等于error的打印 需要打印出响应堆栈 其他的不打印
            let log_str;
            if level >= ErrorLevel {
                // let bt = Backtrace::capture();
                let bt = Backtrace::new();

                log_str = format!(
                    "[{}][{}][{}:{}] {} \n{}\n",
                    level_str,
                    get_time_str_ms(),
                    file,
                    line,
                    args,
                    Self::get_backtrace_str(&bt)
                );
            } else {
                log_str = format!(
                    "[{}][{}][{}:{}] {}\n",
                    level_str,
                    get_time_str_ms(),
                    file,
                    line,
                    args
                );
            }

            let _ = writeln!(self.file, "{}", log_str);
            let _ = self.file.flush(); // 根据实际需求 是否每次写日志都刷到文件
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(output_color)).set_bold(true));
            let _ = writeln!(&mut stdout, "{}", log_str);
            let _ = stdout.reset();
        }
    }
}
