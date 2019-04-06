//! Example for initializing the logger with the LogBuilder
use log::{debug, error, info, trace, warn};
use loggify::{AwsBuilder, LogBuilder};

fn main() {
    LogBuilder::new()
        .add_aws(AwsBuilder::default())
        .add_exclude(String::from("hyper"))
        .add_exclude(String::from("tokio"))
        .add_exclude(String::from("want"))
        .add_exclude(String::from("mio"))
        .add_exclude(String::from("rusoto"))
        .set_level(log::Level::Trace)
        .set_time_format(String::from("%H:%M:%S"))
        .set_log_target(false)
        .build()
        .unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("My debug message");
    trace!("My trace message");
}
