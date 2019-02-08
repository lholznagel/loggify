use log::{error, warn, info};
use loggify::LogBuilder;

fn main() {
    LogBuilder::new()
        .set_time_format(String::from("%H:%M:%S"))
        .build()
        .unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
}