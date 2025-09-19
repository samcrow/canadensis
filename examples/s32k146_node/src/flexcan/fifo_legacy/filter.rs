use crate::flexcan::id::Id;
///! Data types for legacy FIFO receive filters
use crate::flexcan::id::{EXTENDED_ID_MASK, STANDARD_ID_MASK};
use core::mem::ManuallyDrop;
use vcell::VolatileCell;

const FILTERS: usize = 128;

/// A filter that matches an exact ID and RTR bit
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FilterA(/* Big-endian */ u32);

impl FilterA {
    /// Creates a filter that matches an exact standard ID and RTR bit
    pub fn standard(id: u16, rtr: bool) -> Self {
        let bits = (rtr as u32) << 31 | (u32::from(id) & STANDARD_ID_MASK) << 19;
        Self(u32::to_be(bits))
    }
    /// Creates a filter that matches an exact extended ID and RTR bit
    pub fn extended(id: u32, rtr: bool) -> Self {
        let bits = (rtr as u32) << 31 | 1 << 30 | (id & EXTENDED_ID_MASK) << 1;
        Self(u32::to_be(bits))
    }
}

/// A filter that matches two IDs, standard or extended
#[repr(C)]
#[derive(Copy, Clone)]
pub struct FilterB(/* Big-endian */ u32);

impl FilterB {
    /// Creates a filter that matches either of two ID and RTR groups
    ///
    /// For a standard ID, all 11 ID bits must match. For an extended ID, the 11 most significant
    /// bits must match.
    pub fn two_ids(ids: [IdRtr; 2]) -> Self {
        let bits = u32::from(filter_b_part(ids[0])) << 16 | u32::from(filter_b_part(ids[1]));
        Self(u32::to_be(bits))
    }
}

/// Creates a 16-bit value that matches half of the filter B format
fn filter_b_part(id: IdRtr) -> u16 {
    let mut bits = (id.1 as u16) << 15 | (id.0.is_extended() as u16) << 14;
    if id.0.is_extended() {
        // Extended ID, 14 bits in positions 13:0
        bits |= (id.0.bits() >> 15) as u16;
    } else {
        // Standard ID, 11 bits in positions 13:3
        bits |= (id.0.bits() as u16) << 3;
    }
    bits
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FilterC(/* Big-endian */ u32);

impl FilterC {
    /// Creates a filter that matches any of four IDs
    ///
    /// The filter will accept both standard and extended IDs with RTR set to 0 or 1. Regardless
    /// of the ID format, it matches the 8 most significant bits of the ID and ignores the rest.
    pub fn four_ids(ids: [Id; 4]) -> Self {
        let bits = u32::from(filter_c_part(ids[0])) << 24
            | u32::from(filter_c_part(ids[1])) << 16
            | u32::from(filter_c_part(ids[2])) << 8
            | u32::from(filter_c_part(ids[3]));
        Self(u32::to_be(bits))
    }
}
/// Creates an 8-bit value that matches one quarter of the filter C format
fn filter_c_part(id: Id) -> u8 {
    if id.is_extended() {
        // Ignore 21 less significant bits
        (id.bits() >> 21) as u8
    } else {
        // Ignore 3 less significant bits
        (id.bits() >> 3) as u8
    }
}

/// A message identifier and corresponding RTR bit
#[derive(Copy, Clone)]
pub struct IdRtr(pub Id, pub bool);

/// A block of 128 filters, all in the same format
pub union FilterTable {
    /// Format A
    format_a: ManuallyDrop<[VolatileCell<FilterA>; FILTERS]>,
    /// Format B
    format_b: ManuallyDrop<[VolatileCell<FilterB>; FILTERS]>,
    /// Format C
    format_c: ManuallyDrop<[VolatileCell<FilterB>; FILTERS]>,
}
