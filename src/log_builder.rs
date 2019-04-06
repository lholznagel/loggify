use crate::aws::Aws;
use crate::aws_builder::AwsBuilder;
use crate::Loggify;

use log;
use log::{Level, SetLoggerError};

/// Struct for building a new logger
pub struct LogBuilder {
    aws: Option<Aws>,
    exclude: Vec<String>,
    level: Level,
    time_format: String,
    log_target: bool,
}

impl LogBuilder {
    /// Creates a new instance of the `LogBuilder`
    ///
    /// # Defaults
    /// - `level` -> The default level is `Info`
    /// - `exclude` -> No targets are excluded
    /// - `time_format` -> dd.mm.YY HH:MM:SS
    /// - `log_target` -> false
    ///
    /// # Example
    /// ```
    /// //! Example for initializing the logger with the LogBuilder
    /// use log::{error, warn, info, debug, trace};
    /// use loggify::LogBuilder;
    ///
    /// /// The `LogBuilder` is used to set more logger options
    /// /// This example will change the log level to Trace
    /// /// and the printed time format to time only
    /// fn main() {
    ///     LogBuilder::new()
    ///         .set_level(log::Level::Trace)
    ///         .set_time_format(String::from("%H:%M:%S"))
    ///         .build()
    ///         .unwrap();
    ///
    ///     error!("My error message");
    ///     warn!("My warn message");
    ///     info!("My info message");
    ///     debug!("My debug message");
    ///     trace!("My trace message");
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            aws: None,
            level: Level::Info,
            exclude: Vec::new(),
            time_format: String::from("%d.%m.%Y %H:%M:%S"),
            log_target: false,
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

    /// Defines if the log target should be printed or not
    /// This option should be used to find out where a log comes
    /// from and what target to exclude from logging
    pub fn set_log_target(mut self, state: bool) -> Self {
        self.log_target = state;
        self
    }

    pub fn add_aws(mut self, aws: AwsBuilder) -> Self {
        self.aws = Some(aws.build());
        self
    }

    /// Creates a new logger
    pub fn build(self) -> Result<(), SetLoggerError> {
        let logger = Loggify {
            aws: self.aws,
            level: self.level,
            exclude: self.exclude,
            time_format: self.time_format,
            log_target: self.log_target,
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(self.level.to_level_filter());
        Ok(())
    }
}

impl Default for LogBuilder {
    fn default() -> Self {
        Self {
            aws: None,
            level: Level::Info,
            exclude: Vec::new(),
            time_format: String::from("%d.%m.%Y %H:%M:%S"),
            log_target: false,
        }
    }
}
