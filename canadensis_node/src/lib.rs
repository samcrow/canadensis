#![no_std]

extern crate alloc;
extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_core;
extern crate canadensis_data_types;
extern crate canadensis_derive_register_block;
extern crate canadensis_encoding;
extern crate canadensis_filter_config;
extern crate fallible_collections;
extern crate heapless;

/// Forwards to rtt_target::rprintln if the rtt-debug feature is enabled
#[cfg(feature = "rtt-debug")]
macro_rules! debugln {
    ($fmt:expr) => { rtt_target::rprintln!($fmt) };
    ($fmt:expr, $($arg:tt)*) => { rtt_target::rprintln!($fmt, $($arg)*) };
}
#[cfg(not(feature = "rtt-debug"))]
macro_rules! debugln {
    ($fmt:expr) => {};
    ($fmt:expr, $($arg:tt)*) => {};
}

mod basic;
mod minimal;
pub mod register;
pub use crate::basic::BasicNode;
pub use crate::minimal::MinimalNode;
