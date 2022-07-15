//! Instant and duration definitions

pub mod u48;

use crate::time::u48::U48;
use core::cmp::Ordering;
use core::convert::TryInto;
use core::fmt::Debug;
use core::ops::Add;

/// A moment in time relative to some point in the past
///
/// This trait defines the basic properties of an instant that Cyphal requires.
///
/// # Clock requirements
///
/// The clock that generates instants must be monotonic (its instant values never decrease),
/// except that it may overflow when it reaches an implementation-defined maximum time value.
///
/// The instant type must be able to correctly calculate the duration between two instants
/// when overflow has happened once. If overflow has happened more than once between two instants,
/// the calculated duration will be too short.
///
pub trait Instant: Debug + Copy + Clone {
    /// The duration between two instants
    ///
    /// This type must be able to represent the difference between the maximum and minimum instant
    /// values.
    ///
    /// The Duration must also support adding a Duration and Instant to produce an Instant
    type Duration: Duration + Add<Self, Output = Self>;

    /// Calculates the duration between other and self
    ///
    /// # Overflow
    ///
    /// Instants may overflow, leading to other having a greater numerical value than self even if
    /// other is really earlier. Implementations of this function must handle this case correctly,
    /// assuming that overflow has only happened once between other and self.
    ///
    /// To put it another way, if other has a larger numerical value than self, this function
    /// must assume that the clock has overflowed once and return a non-negative Duration.
    ///
    fn duration_since(&self, other: &Self) -> Self::Duration;

    /// Compares this instant and another in a way that works correctly with overflow
    ///
    /// This function should compare self and other, but with one additional rule: if other
    /// has a larger value than self but the absolute value of the difference is less than or equal
    /// to half the maximum Instant value, this function must return `Ordering::Greater`.
    fn overflow_safe_compare(&self, other: &Self) -> Ordering;
}

/// A duration created from the difference between two instants
pub trait Duration: PartialOrd + Debug + Default + Copy + Clone + Add<Self, Output = Self> {
    /// Creates a duration from a number of milliseconds
    ///
    /// This function returns None if this duration type cannot represent the provided number
    /// of milliseconds.
    fn from_millis(millis: u32) -> Option<Self>;

    /// Returns the number of whole seconds in this duration, rounded down
    fn as_secs(&self) -> u64;
    /// Returns the fractional part of this duration in nanoseconds
    fn subsec_nanos(&self) -> u32;
}

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
}

impl Duration for MicrosecondDuration32 {
    fn from_millis(millis: u32) -> Option<Self> {
        let microseconds = millis.checked_mul(1000)?;
        Some(MicrosecondDuration32(microseconds))
    }

    fn as_secs(&self) -> u64 {
        u64::from(self.0 / 1_000_000)
    }

    fn subsec_nanos(&self) -> u32 {
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
}

impl Instant for Microseconds32 {
    type Duration = MicrosecondDuration32;

    fn duration_since(&self, other: &Self) -> Self::Duration {
        MicrosecondDuration32(self.0.wrapping_sub(other.0))
    }

    fn overflow_safe_compare(&self, other: &Self) -> Ordering {
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

// End 32-bit duration/instant
// Begin 48-bit duration/instant

/// A duration represented as a 48-bit number of microseconds
///
/// This type can represent durations of up to about 9 years.
///
/// It takes up 8 bytes of space (the same as a 64-bit duration).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MicrosecondDuration48(U48);

impl MicrosecondDuration48 {
    /// Creates a duration from a number of microseconds
    pub fn new(microseconds: U48) -> Self {
        MicrosecondDuration48(microseconds)
    }

    /// Returns the number of microseconds this duration represents
    pub fn as_microseconds(&self) -> U48 {
        self.0
    }
}

impl Duration for MicrosecondDuration48 {
    fn from_millis(millis: u32) -> Option<Self> {
        let microseconds = u64::from(millis).checked_mul(1000)?;
        let microseconds_48: U48 = microseconds.try_into().ok()?;
        Some(MicrosecondDuration48(microseconds_48))
    }

    fn as_secs(&self) -> u64 {
        u64::from(self.0 / 1_000_000)
    }

    fn subsec_nanos(&self) -> u32 {
        let microseconds = self.0 % 1_000_000;
        (microseconds * 1000).try_into().unwrap()
    }
}

impl Add for MicrosecondDuration48 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MicrosecondDuration48(self.0 + rhs.0)
    }
}

impl Add<Microseconds48> for MicrosecondDuration48 {
    type Output = Microseconds48;

    fn add(self, rhs: Microseconds48) -> Self::Output {
        Microseconds48(self.0 + rhs.0)
    }
}

/// An instant represented as a 48-bit number of microseconds
///
/// This type overflows after about 9 years.
///
/// It takes up 8 bytes of space (the same as a 64-bit instant).
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Microseconds48(U48);

impl Microseconds48 {
    /// Creates an instant from a number of microseconds
    pub fn new(microseconds: U48) -> Self {
        Microseconds48(microseconds)
    }

    /// Returns the number of microseconds this instant represents
    pub fn as_microseconds(&self) -> U48 {
        self.0
    }
}

impl Instant for Microseconds48 {
    type Duration = MicrosecondDuration48;

    fn duration_since(&self, other: &Self) -> Self::Duration {
        MicrosecondDuration48(self.0.wrapping_sub(other.0))
    }

    fn overflow_safe_compare(&self, other: &Self) -> Ordering {
        // https://www.rapitasystems.com/blog/what-happened-first-handling-timer-wraparound
        let half_max = U48::MAX / 2;
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

// End 48-bit duration/instant
// Begin 64-bit duration/instant

/// A duration represented as a 64-bit number of microseconds
///
/// This type can represent durations of up to about five hundred thousand years.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct MicrosecondDuration64(u64);

impl MicrosecondDuration64 {
    /// Creates a duration from a number of microseconds
    pub fn new(microseconds: u64) -> Self {
        MicrosecondDuration64(microseconds)
    }

    /// Returns the number of microseconds this duration represents
    pub fn as_microseconds(&self) -> u64 {
        self.0
    }
}

impl Duration for MicrosecondDuration64 {
    fn from_millis(millis: u32) -> Option<Self> {
        let microseconds = u64::from(millis).checked_mul(1000)?;
        Some(MicrosecondDuration64(microseconds))
    }

    fn as_secs(&self) -> u64 {
        self.0 / 1_000_000
    }

    fn subsec_nanos(&self) -> u32 {
        let microseconds = self.0 % 1_000_000;
        (microseconds * 1000) as u32
    }
}

impl Add for MicrosecondDuration64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        MicrosecondDuration64(self.0 + rhs.0)
    }
}

impl Add<Microseconds64> for MicrosecondDuration64 {
    type Output = Microseconds64;

    fn add(self, rhs: Microseconds64) -> Self::Output {
        Microseconds64(self.0 + rhs.0)
    }
}

/// An instant represented as a 64-bit number of microseconds
///
/// This type overflows after about five hundred thousand years.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Microseconds64(u64);

impl Microseconds64 {
    /// Creates an instant from a number of microseconds
    pub fn new(microseconds: u64) -> Self {
        Microseconds64(microseconds)
    }

    /// Returns the number of microseconds this instant represents
    pub fn as_microseconds(&self) -> u64 {
        self.0
    }
}

impl Instant for Microseconds64 {
    type Duration = MicrosecondDuration64;

    fn duration_since(&self, other: &Self) -> Self::Duration {
        MicrosecondDuration64(self.0.wrapping_sub(other.0))
    }

    fn overflow_safe_compare(&self, other: &Self) -> Ordering {
        // https://www.rapitasystems.com/blog/what-happened-first-handling-timer-wraparound
        let half_max = u64::MAX / 2;
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

// End 64-bit duration/instant

/// Something that can provide the current time
pub trait Clock {
    /// The type of instant that this clock produces
    type Instant: Instant;
    /// Returns the current time
    fn now(&mut self) -> Self::Instant;
}

/// Creates a duration from a number of milliseconds
///
/// This is a convenient wrapper for T::from_millis() when T has a long-difficult-to-type name.
///
/// # Panics
///
/// This function panics of T's `from_millis` function returns `None`.
pub fn milliseconds<T>(milliseconds: u32) -> T
where
    T: Duration,
{
    T::from_millis(milliseconds).expect("Invalid duration")
}

/// Creates a duration from a number of milliseconds
///
/// This is a convenient wrapper for T::from_millis() when T has a long-difficult-to-type name.
pub fn try_milliseconds<T>(milliseconds: u32) -> Option<T>
where
    T: Duration,
{
    T::from_millis(milliseconds)
}
