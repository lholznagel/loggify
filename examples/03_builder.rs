//! Example for initializing the logger with the LogBuilder
use log::{error, warn, info, debug, trace};
use loggify::LogBuilder;

/// The `LogBuilder` is used to set more logger options
/// This example will change the log level to Trace
/// and the printed time format to time only
fn main() {
    LogBuilder::new()
        .set_level(log::Level::Trace)
        .set_time_format(String::from("%H:%M:%S"))
        .build()
        .unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("My debug message");
    trace!("My trace message");
}