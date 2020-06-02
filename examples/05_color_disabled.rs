//! Example for initializing the logger with the LogBuilder
use log::{error, warn, info, debug, trace};
use loggify::LogBuilder;

/// The `LogBuilder` is used to set more logger options
/// This example will disable the color output
fn main() {
    LogBuilder::new()
        .disable_color()
        .build()
        .unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("My debug message");
    trace!("My trace message");
}