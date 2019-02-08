use crate::Loggify;

use log;
use log::{Level, SetLoggerError};

/// Struct for building a new logger
pub struct LogBuilder {
    exclude: Vec<String>,
    level: Level,
    time_format: String
}

impl LogBuilder {
    /// Creates a new instance of the `LogBuilder`
    /// 
    /// # Defaults
    /// - `level` -> The default level is `Info`
    /// - `exclude` -> No targets are excluded
    /// 
    /// # Example
    /// ```
    /// #[macro_use]
    ///    extern crate log;
    ///    extern crate loggify;
    ///
    ///    use loggify::LogBuilder;
    ///
    ///    fn main() {
    ///        LogBuilder::new()
    ///            .add_exclude("exclude::exclude_example".to_string())
    ///            .set_level(log::Level::Trace)
    ///            .set_time_format(String::from("%H:%M:%S"))
    ///            .build()
    ///            .unwrap();
    ///
    ///        error!("My error message");
    ///        warn!("My warn message");
    ///        info!("My info message");
    ///        debug!("My debug message");
    ///        trace!("My trace message");
    ///    }
    /// ```
    pub fn new() -> Self {
        Self {
            level: Level::Info,
            exclude: Vec::new(),
            time_format: String::from("%d.%m.%Y %H:%M:%S")
        }
    }

    /// Adds a new target to exclude from logging
    /// 
    /// See the example `exclude` for usage
    pub fn add_exclude(mut self, name: String) -> Self {
        self.exclude.push(name);
        self
    }

    /// Sets the minimum level
    pub fn set_level(mut self, level: Level) -> Self {
        self.level = level;
        self
    }

    /// Sets the time format
    /// See https://docs.rs/chrono/0.4.6/chrono/format/strftime/index.html for supported escape sequences
    pub fn set_time_format(mut self, format: String) -> Self {
        self.time_format = format;
        self
    }

    /// Creates a new logger
    pub fn build(self) -> Result<(), SetLoggerError> {
        let logger = Loggify { 
            level: self.level, 
            exclude: self.exclude,
            time_format: self.time_format
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(self.level.to_level_filter());
        Ok(())
    }
}