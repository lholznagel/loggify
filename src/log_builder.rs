use crate::Loggify;

use log;
use log::{Level, SetLoggerError};

/// Struct for building a new logger
pub struct LogBuilder {
    level: Level,
    exclude: Vec<String>
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
    ///            .set_level(log::Level::Trace)
    ///            .add_exclude("exclude::exclude_example".to_string())
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
            exclude: Vec::new()
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

    /// Creates a new logger
    pub fn build(self) -> Result<(), SetLoggerError> {
        let logger = Loggify { 
            level: self.level, 
            exclude: self.exclude 
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(self.level.to_level_filter());
        Ok(())
    }
}