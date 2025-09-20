/// CAN message identifiers

/// The bit set in the `Id` to indicate the ID is extended
const EXTENDED_ID_FLAG: u32 = 0x8000_0000;
/// 29 1s, right-aligned
pub(crate) const EXTENDED_ID_MASK: u32 = 0x1fff_ffff;
/// 11 1s, right-aligned
pub(crate) const STANDARD_ID_MASK: u32 = 0x0000_07ff;

/// A simple representation of a standard or extended ID
// The internal representation is aligned to the right.
// If the EXTENDED_ID_FLAG bit is set, the ID is extended (29 bits).
#[derive(Copy, Clone, Debug)]
pub struct Id(u32);

impl Id {
    /// Creates a standard identifier from 11 right-aligned bits in a u16
    ///
    /// This function silently discards any bits other than the 11 least significant bits.
    pub fn standard(bits: u16) -> Self {
        Id(u32::from(bits) & STANDARD_ID_MASK)
    }
    /// Creates a standard identifier from 29 right-aligned bits in a u32
    ///
    /// This function silently discards any bits other than the 29 least significant bits.
    pub fn extended(bits: u32) -> Self {
        Id(EXTENDED_ID_FLAG | (bits & EXTENDED_ID_MASK))
    }

    /// Returns true if this is an extended ID
    pub fn is_extended(&self) -> bool {
        (self.0 & EXTENDED_ID_FLAG) != 0
    }
    /// Returns the bits in this identifier, aligned to the right
    pub fn bits(&self) -> u32 {
        self.0 & EXTENDED_ID_MASK
    }
}
