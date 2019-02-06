use log::{error, warn, info, debug, trace};
use loggify::LogBuilder;

fn main() {
    LogBuilder::new()
        .set_level(log::Level::Trace)
        .build()
        .unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("My debug message");
    trace!("My trace message");
}