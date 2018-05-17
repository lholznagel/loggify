#[macro_use]
extern crate log;
extern crate loggify;

use loggify::LogBuilder;

mod exclude_example;

fn main() {
    LogBuilder::new()
        .add_exclude("exclude::exclude_example".to_string())
        .set_level(log::Level::Trace)
        .build()
        .unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("My debug message");
    trace!("My trace message");

    exclude_example::example::call_me();
}