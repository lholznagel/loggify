# Loggify

A small and simple log implementation that I use in my rust projects.

## Install

Just add

``` toml
[dependencies]
loggify = "1.0.0"
```

to your `Cargo.toml`

## Usage

The simpliest way is just to call `init`.
The default log level is `Info`, so that debug and trace messages are not shown.

``` rust
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
```

If you want to set the log level you an use `init_with_level`.

``` rust
#[macro_use]
extern crate log;
extern crate loggify;

use loggify::Loggify;

fn main() {
    Loggify::init_with_level(log::Level::Trace).unwrap();

    error!("My error message");
    warn!("My warn message");
    info!("My info message");
    debug!("My debug message");
    trace!("My trace message");
}
```

## Exclude targets from log

Sometimes you want to exclude some targets from the log.

``` rust
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
```

For this we need to use the `LogBuilder`.
With this all logs that come from a target that contains `exclude::exclude_example` is not shown.

## Examples

For help you can take a look in the examples folder.

## Example output

[![terminal](./assets/terminal.png)](./assets/terminal.png)