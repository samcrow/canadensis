//! Instant and duration definitions
//!
//!
//! # Clock requirements
//!
//! The clock that generates instants must be monotonic (its instant values never decrease),
//! except that it may overflow when it reaches an implementation-defined maximum time value.
//!
//! The instant type must be able to correctly calculate the duration between two instants
//! when overflow has happened once. If overflow has happened more than once between two instants,
//! the calculated duration will be too short.
//!

use fugit::{Instant, MicrosDurationU32, MillisDurationU32};

/// A duration represented as a 32-bit number of microseconds
///
/// This type can represent durations of up to about 1 hour.
pub type MicrosecondDuration32 = MicrosDurationU32;

/// An instant represented as a 32-bit number of microseconds
///
/// This type overflows after about 1 hour.
pub type Microseconds32 = Instant<u32, 1, 1_000_000>;

/// Something that can provide the current time
pub trait Clock {
    /// Returns the current time
    fn now(&mut self) -> Microseconds32;
}

/// Creates a duration from a number of milliseconds
///
/// # Panics
///
/// This function panics if the provided number of milliseconds, converted into microseconds,
/// is too large for a u32
pub const fn milliseconds(milliseconds: u32) -> MicrosecondDuration32 {
    let milliseconds = MillisDurationU32::from_ticks(milliseconds);
    milliseconds
        .const_try_into()
        .expect("Number of milliseconds out of range")
}
