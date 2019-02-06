use crate::LogBuilder;

use log::{Log, Level, Metadata, Record, SetLoggerError};
use time;

/// Logger
pub struct Loggify {
    /// defines the minimum log level
    pub level: Level,
    /// all targets added are excluded from the logger
    pub exclude: Vec<String>
}

impl Loggify {
    /// Creates a new logger using the default values
    /// 
    /// # Defaults
    /// - `level` -> The default level is `Info`
    /// - `exclude` -> No targets are excluded
    /// 
    /// # Example
    /// ```
    /// #[macro_use]
    /// extern crate log;
    /// extern crate loggify;
    /// 
    /// use loggify::Loggify;
    /// 
    /// fn main() {
    ///     Loggify::init().unwrap();
    /// 
    ///     error!("My error message");
    ///     warn!("My warn message");
    ///     info!("My info message");
    ///     debug!("Will not be shown");
    ///     trace!("Will not be shown");
    /// }
    /// ```
    pub fn init() -> Result<(), SetLoggerError> {
        LogBuilder::new().build()
    }

    /// Same as `init` but with log level
    /// 
    /// # Example
    /// ```
    /// #[macro_use]
    /// extern crate log;
    /// extern crate loggify;
    /// 
    /// use loggify::Loggify;
    /// 
    /// fn main() {
    ///     Loggify::init_with_level(log::Level::Trace).unwrap();
    /// 
    ///     error!("My error message");
    ///     warn!("My warn message");
    ///     info!("My info message");
    ///     debug!("My debug message");
    ///     trace!("My trace message");
    /// }
    /// ```
    pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
        LogBuilder::new().set_level(level).build()
    }
}

impl Log for Loggify {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let mut result = true;

        for value in self.exclude.clone() {
            if metadata.target().contains(&value) {
                result = false;
            }
        }

        !self.exclude.contains(&metadata.target().to_string()) && result && metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
           return;
        }

        let mut level_msg = String::new();
        match record.level() {
            Level::Error => level_msg.push_str("\x1B[0;31mError"),
            Level::Warn  => level_msg.push_str("\x1B[0;93mWarn "),
            Level::Info  => level_msg.push_str("\x1B[0;34mInfo "),
            Level::Debug => level_msg.push_str("\x1B[0;35mDebug"),
            Level::Trace => level_msg.push_str("\x1B[0;36mTrace")
        };

        println!(
            "\x1B[1;30m[{}] > \x1B {} \x1B[1;30m>\x1B[0m {}", 
            time::strftime("%d.%m.%Y %H:%M:%S", &time::now()).unwrap(), 
            level_msg, 
            record.args());
    }

    fn flush(&self) {
    }
}