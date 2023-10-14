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

use core::cmp::Ordering;
use core::fmt::Debug;
use core::ops::Add;

/// A duration represented as a 32-bit number of microseconds
///
/// This type can represent durations of up to about 1 hour.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MicrosecondDuration32(u32);

impl MicrosecondDuration32 {
    /// Creates a duration from a number of microseconds
    pub fn new(microseconds: u32) -> Self {
        MicrosecondDuration32(microseconds)
    }

    /// Returns the number of microseconds this duration represents
    pub fn as_microseconds(&self) -> u32 {
        self.0
    }

    /// Creates a duration from a number of milliseconds and returns it, or returns None if the
    /// number of milliseconds is too large to be represented
    pub fn from_millis(millis: u32) -> Option<Self> {
        let microseconds = millis.checked_mul(1000)?;
        Some(MicrosecondDuration32(microseconds))
    }

    /// Returns the number of seconds in this duration, rounded down
    pub fn as_secs(&self) -> u64 {
        u64::from(self.0 / 1_000_000)
    }

    /// Returns the number of nanoseconds in the fractional part of this duration
    pub fn subsec_nanos(&self) -> u32 {
        let microseconds = self.0 % 1_000_000;
        microseconds * 1000
    }
}

impl Add for MicrosecondDuration32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MicrosecondDuration32(self.0 + rhs.0)
    }
}

impl Add<Microseconds32> for MicrosecondDuration32 {
    type Output = Microseconds32;

    fn add(self, rhs: Microseconds32) -> Self::Output {
        Microseconds32(self.0 + rhs.0)
    }
}

/// An instant represented as a 32-bit number of microseconds
///
/// This type overflows after about 1 hour.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Microseconds32(u32);

impl Microseconds32 {
    /// Creates an instant from a number of microseconds
    pub fn new(microseconds: u32) -> Self {
        Microseconds32(microseconds)
    }

    /// Returns the number of microseconds this instant represents
    pub fn as_microseconds(&self) -> u32 {
        self.0
    }

    /// Returns the duration between this time and another previous time
    pub fn duration_since(&self, other: Self) -> MicrosecondDuration32 {
        MicrosecondDuration32(self.0.wrapping_sub(other.0))
    }

    /// Compares this instant to another in a way that correctly handles one overflow
    pub fn overflow_safe_compare(&self, other: Self) -> Ordering {
        // https://www.rapitasystems.com/blog/what-happened-first-handling-timer-wraparound
        let half_max = u32::MAX / 2;
        if self.0 == other.0 {
            Ordering::Equal
        } else {
            let subtract_result = other.0.wrapping_sub(self.0);
            if subtract_result <= half_max {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }
}

/// Something that can provide the current time
pub trait Clock {
    /// Returns the current time
    fn now(&mut self) -> Microseconds32;
}

/// Creates a duration from a number of milliseconds
///
/// This is a convenient wrapper for T::from_millis() when T has a long-difficult-to-type name.
///
/// # Panics
///
/// This function panics of T's `from_millis` function returns `None`.
pub fn milliseconds(milliseconds: u32) -> MicrosecondDuration32 {
    MicrosecondDuration32::from_millis(milliseconds).expect("Invalid duration")
}

/// Creates a duration from a number of milliseconds
///
/// This is a convenient wrapper for T::from_millis() when T has a long-difficult-to-type name.
pub fn try_milliseconds<T>(milliseconds: u32) -> Option<MicrosecondDuration32> {
    MicrosecondDuration32::from_millis(milliseconds)
}
