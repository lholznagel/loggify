//! # Basic usage for the logger
use log::{error, warn, info, debug, trace};
use loggify::Loggify;

/// The default level is INFO. So debug and trace outputs are oppressed
fn main() {
    Loggify::init().unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("Will not be shown");
    trace!("Will not be shown");
}
