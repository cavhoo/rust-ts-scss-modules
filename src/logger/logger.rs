use owo_colors::OwoColorize;
use owo_colors::colors::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
}

pub struct Logger {
    level: LogLevel,
}

impl Logger {
    pub fn new(level: LogLevel) -> Self {
        Logger { level }
    }

    // Core logging method
    fn log(&self, level: LogLevel, message: &str) {
        // Check if message should be logged based on log level
        if !self.should_log(&level) {
            return;
        }

        let level_str = match level {
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
        };

        // Format console output with colors
        let console_message = match level {
            LogLevel::Warning => format!("[{}] {}\n", level_str, message).fg::<Yellow>()
                .to_string(),
            LogLevel::Error => format!("[{}] {}\n", level_str, message)
                .fg::<Red>()
                .to_string(),
            _ => format!("[{}] {}\n", level_str, message),
        };

        // Print to console
        print!("{}", console_message);
    }

    // Check if message should be logged based on configured log level
    fn should_log(&self, level: &LogLevel) -> bool {
        match self.level {
            LogLevel::Debug => true,
            LogLevel::Info => matches!(level, LogLevel::Info | LogLevel::Warning | LogLevel::Error),
            LogLevel::Warning => matches!(level, LogLevel::Warning | LogLevel::Error),
            LogLevel::Error => matches!(level, LogLevel::Error),
        }
    }

    // Public logging methods
    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn info(&self, message: &str) {
        self.log(LogLevel::Info, message);
    }

    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }
}
