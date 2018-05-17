#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    warnings
)]

//! Small and simple log implementation
extern crate log;
extern crate time;

mod loggify;
mod log_builder;

pub use self::loggify::Loggify;
pub use self::log_builder::LogBuilder;