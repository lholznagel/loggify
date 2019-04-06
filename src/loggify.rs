use crate::aws::Aws;
use crate::LogBuilder;

use chrono::Utc;
use log::{Level, Log, Metadata, Record, SetLoggerError};

/// The logger instance
/// To configure the logger, use the `LogBuilder` struct
pub struct Loggify {
    pub(crate) aws: Option<Aws>,
    /// all targets added are excluded from the logger
    pub(crate) exclude: Vec<String>,
    /// defines the minimum log level
    pub(crate) level: Level,
    /// sets the time format
    /// see https://docs.rs/chrono/latest/chrono/format/strftime/index.html for supported escape sequences
    pub(crate) time_format: String,
    /// defines if the target should be logged or not
    /// this option should be used as a debug option
    pub(crate) log_target: bool,
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

        if self.log_target {
            dbg!(metadata.target());
        }

        for value in self.exclude.clone() {
            if metadata.target().contains(&value) {
                result = false;
            }
        }

        !self.exclude.contains(&metadata.target().to_string())
            && result
            && metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let mut level_msg = String::new();
        match record.level() {
            Level::Error => level_msg.push_str("\x1B[0;31mError \x1B"),
            Level::Warn => level_msg.push_str("\x1B[0;93mWarn  \x1B"),
            Level::Info => level_msg.push_str("\x1B[0;34mInfo  \x1B"),
            Level::Debug => level_msg.push_str("\x1B[0;35mDebug \x1B"),
            Level::Trace => level_msg.push_str("\x1B[0;36mTrace \x1B"),
        };

        println!(
            "\x1B[1;30m[{}] > \x1B {}[1;30m>\x1B[0m {}",
            Utc::now().format(&self.time_format),
            level_msg,
            record.args()
        );

        if let Some(aws) = &self.aws {
            aws.put_log(record.args().to_string());
        }
    }

    fn flush(&self) {}
}
