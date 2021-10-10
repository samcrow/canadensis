//! `zerocopy` compatibility for `f16`

use core::cmp::Ordering;
use half::f16;
use zerocopy::{AsBytes, FromBytes};

/// A 16-bit floating-point value that implements `FromBytes` and `AsBytes`
#[derive(Debug, Copy, Clone, FromBytes, AsBytes)]
#[repr(transparent)]
pub struct ZeroCopyF16(u16);

impl From<f16> for ZeroCopyF16 {
    fn from(value: f16) -> Self {
        ZeroCopyF16(value.to_bits())
    }
}

impl From<ZeroCopyF16> for f16 {
    fn from(value: ZeroCopyF16) -> Self {
        f16::from_bits(value.0)
    }
}

impl Default for ZeroCopyF16 {
    fn default() -> Self {
        ZeroCopyF16(f16::default().to_bits())
    }
}

impl PartialOrd for ZeroCopyF16 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        f16::from(*self).partial_cmp(&f16::from(*other))
    }
}

impl PartialEq for ZeroCopyF16 {
    fn eq(&self, other: &Self) -> bool {
        f16::from(*self).eq(&f16::from(*other))
    }
}

mod fmt_impl {
    use super::ZeroCopyF16;
    use core::fmt::{Display, Formatter, Result};
    use half::f16;
    impl Display for ZeroCopyF16 {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            Display::fmt(&f16::from_bits(self.0), f)
        }
    }
}
