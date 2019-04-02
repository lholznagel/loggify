//! Example for excluding log targets from getting logged
use log::{error, warn, info, debug, trace};
use loggify::LogBuilder;

mod example {
    pub mod excluded {
        use log::info;

        pub fn call_me() {
            info!("I will not be logged");
        }
    }

    pub mod included {
        use log::info;

        pub fn call_me() {
            info!("I will be logged");
        }
    }
}

/// Exmple on how to exclude specific log targets
fn main() {
    LogBuilder::new()
        // this will show the log targets so that we can determine
        // what to exclude
        .set_log_target(true)
        // this will oppress all logs coming from example::excluded::*
        .add_exclude("example::excluded".to_string())
        .set_level(log::Level::Trace)
        .build()
        .unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("My debug message");
    trace!("My trace message");

    // the log message of this call will not be shown
    example::excluded::call_me();
    // this log message will be shown
    example::included::call_me();
}