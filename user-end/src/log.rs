use flate2::write::GzEncoder;
use flate2::Compression;
use once_cell::sync::Lazy;
use std::fmt::Arguments;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::utils::{get_time_s_dir, get_time_str_ms};

// 增加日志等级
// 读取配置

const LOG_FILENAME: &str = "v2rayR.log";
const MAX_LOG_FILESIZE: u64 = 10 * 1024 * 1024;

// 需要使用macro_export导出宏 才能在其他模块使用
#[macro_export]
macro_rules! logD {
    ($($arg:tt)*) => {{
        let file = std::file!();
        let line = std::line!();
        Logger::instance().lock().unwrap().log(LogLevel::DebugLevel, format_args!($($arg)*), file, line);
    }};
}

#[macro_export]
macro_rules! logI {
    ($($arg:tt)*) => {{
        let file = std::file!();
        let line = std::line!();
        Logger::instance().lock().unwrap().log(LogLevel::InfoLevel, format_args!($($arg)*), file, line);
    }};
}

#[macro_export]
macro_rules! logW {
    ($($arg:tt)*) => {{
        let file = std::file!();
        let line = std::line!();
        Logger::instance().lock().unwrap().log(LogLevel::WarningLevel, format_args!($($arg)*), file, line);
    }};
}

#[macro_export]
macro_rules! logE {
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

#[derive(Debug, PartialEq, PartialOrd)]
pub enum LogLevel {
    ErrorLevel,
    WarningLevel,
    InfoLevel,
    DebugLevel,
}

pub struct Logger {
    file: Mutex<io::BufWriter<std::fs::File>>,
    level: LogLevel,
}

impl Logger {
    fn new<P: AsRef<Path>>(file_path: P, level: LogLevel) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(file_path)?;
        let buf_writer = io::BufWriter::new(file);
        Ok(Logger {
            file: Mutex::new(buf_writer),
            level,
        })
    }

    pub fn instance() -> Arc<Mutex<Self>> {
        static INSTANCE: Lazy<Arc<Mutex<Logger>>> = Lazy::new(|| {
            let logger =
                Logger::new(LOG_FILENAME, LogLevel::DebugLevel).expect("Failed to create logger");
            Arc::new(Mutex::new(logger))
        });
        INSTANCE.clone()
    }

    pub fn set_level(&mut self, level: LogLevel) {
        self.level = level;
    }

    pub fn log(&self, level: LogLevel, args: Arguments, file: &str, line: u32) {
        if level <= self.level {
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

            let mut file_writer = self.file.lock().unwrap();
            // Check file size
            if let Ok(metadata) = file_writer.get_ref().metadata() {
                if metadata.len() > MAX_LOG_FILESIZE {
                    let log_file = LOG_FILENAME;
                    let archive_file = get_time_s_dir() + ".gz";
                    file_writer.flush().unwrap();
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
                    *file_writer = BufWriter::new(file);
                }
            }

            let log_str = format!(
                "[{}][{}][{}:{}] {}",
                level_str,
                get_time_str_ms(),
                file,
                line,
                args
            );
            let _ = writeln!(file_writer, "{}", log_str);
            let _ = file_writer.flush();
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            let _ = stdout.set_color(ColorSpec::new().set_fg(Some(output_color)).set_bold(true));
            let _ = writeln!(&mut stdout, "{}", log_str);
            let _ = stdout.reset();
        }
    }
}
