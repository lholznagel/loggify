use crate::LogBuilder;

use chalk_rs::Chalk;
use chrono::Utc;
use log::{Log, Level, Metadata, Record, SetLoggerError};

/// The logger instance
/// To configure the logger, use the `LogBuilder` struct
pub struct Loggify {
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
    /// determines if the output is colored or not
    /// per default `true`
    pub(crate) color: bool
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
                break;
            }
        }

        !self.exclude.contains(&metadata.target().to_string()) && result && metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
           return;
        }

        let level_msg = if self.color {
            match record.level() {
                Level::Error => Chalk::new().red().bold().string(&"Error"),
                Level::Warn  => Chalk::new().yellow().bold().string(&"Warn "),
                Level::Info  => Chalk::new().cyan().bold().string(&"Info "),
                Level::Debug => Chalk::new().magenta().bold().string(&"Debug"),
                Level::Trace => Chalk::new().blue().bold().string(&"Trace"),
            }
        } else {
            match record.level() {
                Level::Error => "Error".into(),
                Level::Warn  => "Warn ".into(),
                Level::Info  => "Info ".into(),
                Level::Debug => "Debug".into(),
                Level::Trace => "Trace".into(),
            }
        };

        let time = if self.color {
            Chalk::new().grey().string(&Utc::now().format(&self.time_format))
        } else {
            Utc::now().format(&self.time_format).to_string()
        };

        let msg = if self.color {
            Chalk::new().white().bold().string(&record.args())
        } else {
            record.args().to_string()
        };

        println!(
            "[{}] > {} > {}",
            time,
            level_msg,
            msg
        );
    }

    fn flush(&self) {
    }
}