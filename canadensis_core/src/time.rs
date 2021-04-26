use core::cmp::Ordering;
use core::ops::Shr;
use num_traits::{Bounded, WrappingSub};

/// A moment in time relative to some point in the past
///
/// This trait defines the basic properties of an instant that UAVCAN requires.
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
pub trait Instant {
    /// The duration between two instants
    ///
    /// This type must be able to represent the difference between the maximum and minimum instant
    /// values.
    type Duration: PartialOrd;

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

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PrimitiveDuration<I>(I);

impl<I> PrimitiveDuration<I>
where
    I: Clone,
{
    pub fn new(ticks: I) -> Self {
        PrimitiveDuration(ticks)
    }

    pub fn ticks(&self) -> I {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PrimitiveInstant<I>(I);

impl<I> PrimitiveInstant<I>
where
    I: Clone,
{
    pub fn new(ticks: I) -> Self {
        PrimitiveInstant(ticks)
    }

    pub fn ticks(&self) -> I {
        self.0.clone()
    }
}

impl<I> Instant for PrimitiveInstant<I>
where
    I: PartialOrd + Bounded + WrappingSub + Shr<u32, Output = I>,
{
    type Duration = PrimitiveDuration<I>;

    fn duration_since(&self, other: &Self) -> Self::Duration {
        PrimitiveDuration(self.0.wrapping_sub(&other.0))
    }

    fn overflow_safe_compare(&self, other: &Self) -> Ordering {
        // https://www.rapitasystems.com/blog/what-happened-first-handling-timer-wraparound
        let half_max = I::max_value() >> 1;
        if self.0 == other.0 {
            Ordering::Equal
        } else if (other.0.wrapping_sub(&self.0)) <= half_max {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Instant, PrimitiveInstant};
    use core::cmp::Ordering;

    #[test]
    fn instant_u8_compare() {
        fn compare(ticks1: u8, ticks2: u8) -> Ordering {
            PrimitiveInstant::new(ticks1).overflow_safe_compare(&PrimitiveInstant::new(ticks2))
        }

        // Basic equality
        assert_eq!(compare(0, 0), Ordering::Equal);
        assert_eq!(compare(127, 127), Ordering::Equal);
        assert_eq!(compare(255, 255), Ordering::Equal);

        // With a difference of less than or equal to 128, comparison assumes that overflow
        // hasn't happened and works normally
        assert_eq!(compare(0, 10), Ordering::Less);
        assert_eq!(compare(0, 126), Ordering::Less);
        assert_eq!(compare(0, 127), Ordering::Less);
        // When the difference reaches 128, comparison thinks that overflow has happened and the
        // result is reversed.
        // Example: instant(128 ticks) + duration(128 ticks) overflows to instant(0 ticks),
        // which is later.
        assert_eq!(compare(0, 128), Ordering::Greater);
        assert_eq!(compare(0, 129), Ordering::Greater);
        assert_eq!(compare(0, 130), Ordering::Greater);
        assert_eq!(compare(0, 255), Ordering::Greater);
    }

    #[test]
    fn duration_u8() {
        fn duration(from: u8, to: u8) -> u8 {
            PrimitiveInstant::new(to)
                .duration_since(&PrimitiveInstant::new(from))
                .ticks()
        }

        // Basics
        assert_eq!(duration(0, 0), 0);
        assert_eq!(duration(0, 1), 1);
        assert_eq!(duration(0, 254), 254);
        assert_eq!(duration(0, 255), 255);
        assert_eq!(duration(254, 255), 1);
        assert_eq!(duration(255, 255), 0);

        // Overflow
        assert_eq!(duration(255, 0), 1);
        assert_eq!(duration(255, 1), 2);
        assert_eq!(duration(254, 0), 2);
        assert_eq!(duration(128, 127), 255);
        assert_eq!(duration(254, 253), 255);
    }
}
