//! Nice module!

use chrono::Utc;
use colored::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Clone, Deserialize, Eq, PartialOrd, Ord)] // This probably wants :Copy as well.
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub trait Logger {
    fn log(&self, log_level: LogLevel, msg: &str);

    fn log_debug(&self, msg: &str) {
        self.log(LogLevel::Debug, msg)
    }

    fn log_info(&self, msg: &str) {
        self.log(LogLevel::Info, msg)
    }

    fn log_warn(&self, msg: &str) {
        self.log(LogLevel::Warn, msg)
    }

    fn log_error(&self, msg: &str) {
        self.log(LogLevel::Error, msg)
    }
}

pub struct ConsoleLogger {
    log_level_for_output: LogLevel,
}

impl Logger for ConsoleLogger {
    fn log(&self, log_level: LogLevel, msg: &str) {
        if log_level >= self.log_level_for_output {
            let formatted_date = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            match log_level {
                LogLevel::Debug => {
                    eprintln!(
                        "[{}] - {}: {}",
                        "DEBUG".purple(),
                        formatted_date.purple(),
                        msg.purple()
                    );
                }
                LogLevel::Info => {
                    eprintln!(
                        "[{}] - {}: {}",
                        "INFO".cyan(),
                        formatted_date.cyan(),
                        msg.cyan()
                    );
                }
                LogLevel::Warn => {
                    eprintln!(
                        "[{}] - {}: {}",
                        "WARN".yellow(),
                        formatted_date.yellow(),
                        msg.yellow()
                    );
                }
                LogLevel::Error => {
                    eprintln!(
                        "[{}] - {}: {}",
                        "ERROR".red(),
                        formatted_date.red(),
                        msg.red()
                    );
                }
            }
        }
    }
}

impl ConsoleLogger {
    // I'd call this `new`.
    pub fn init(level: LogLevel) -> Self {
        Self {
            log_level_for_output: level,
        }
    }
}
