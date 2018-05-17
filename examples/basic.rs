#[macro_use]
extern crate log;
extern crate loggify;

use loggify::Loggify;

fn main() {
    Loggify::init().unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("Will not be shown");
    trace!("Will not be shown");
}
