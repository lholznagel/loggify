//! Example for initializing the logger with a log level
use log::{error, warn, info, debug, trace, Level};
use loggify::Loggify;

/// Same as the basic example with the difference that
/// the logger is intialized with the debug log level.
fn main() {
    Loggify::init_with_level(Level::Debug).unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("My debug message");
    trace!("Will not be shown");
}
