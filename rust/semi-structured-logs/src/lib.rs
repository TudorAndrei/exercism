// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => return format!("[INFO]: {}", message),
        LogLevel::Warning => return format!("[WARNING]: {}", message),
        LogLevel::Error => return format!("[ERROR]: {}", message),
    }
}
pub fn info(message: &str) -> String {
    return log(LogLevel::Info, message);
}
pub fn warn(message: &str) -> String {
    return log(LogLevel::Warning, message);
}
pub fn error(message: &str) -> String {
    return log(LogLevel::Error, message);
}
