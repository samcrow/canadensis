//!
//! # UAVCAN acceptance filter configuration
//!
//! This library implements the automatic hardware acceptance filter configuration described
//! in section 4.2.4.4 of the UAVCAN specification.
//!
//! To reduce the amount of CPU time spent processing messages, a UAVCAN device can use hardware
//! acceptance filters to ignore CAN messages that it is not interested in. When the application
//! is interested in more message IDs than the number of filters that the hardware supports,
//! this library can find a quasi-optimal set of filters that accepts all interesting message
//! IDs and the minimum number of non-interesting message IDs.
//!
//! ## Basic operation
//!
//! 1. Find the set of message IDs the application is interested in, based on the topics, requests,
//! and responses it wants to receive
//! 2. For each interesting message ID, create a filter that matches exactly that ID. Optimize those
//! filters down to the number of filters the hardware supports:
//!
//! ```
//! use canadensis_filter_config::{optimize, Filter};
//!
//! let interesting_message_ids = [0x107d552a, 0x11733775, 0x136b957b, 0x126bbdaa, 0x1073373b];
//! let mut ideal_filters = [
//!     Filter::exact_match(interesting_message_ids[0]),
//!     Filter::exact_match(interesting_message_ids[1]),
//!     Filter::exact_match(interesting_message_ids[2]),
//!     Filter::exact_match(interesting_message_ids[3]),
//!     Filter::exact_match(interesting_message_ids[4]),
//! ];
//! // Using an imaginary CAN peripheral that supports only 2 receive filters
//! let max_hardware_filters = 2;
//! let optimized_filters = optimize(&mut ideal_filters, max_hardware_filters);
//! assert_eq!(optimized_filters.len(), 2);
//!
//! // Each interesting message ID will be accepted by at least one of the optimized filters
//! for &id in interesting_message_ids.iter() {
//!     assert!(optimized_filters.iter().any(|filter| filter.accepts(id)));
//! }
//! ```
//!
//! 3. Apply the resulting filters to the CAN hardware

#![no_std]
#![deny(missing_docs)]

/// Mask of allowed extended CAN IDs
const EXTENDED_ID_MASK: u32 = 0x1fff_ffff;

/// Bit 31, used to mark a filter as valid
const VALID_BIT: u32 = 0x8000_0000;

/// A generic mask-based filter for extended CAN IDs
///
/// A filter will accept a message if (message_id & filter.mask) == (filter.id & filter.mask).
#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Filter {
    /// Mask of bits to compare (0x1fff_ffff requires all ID bits to match, 0x0 accepts any ID)
    mask: u32,
    /// Message ID to accept
    ///
    /// The most significant bit (bit 31) indicates that this filter is valid. The optimize function
    /// uses this bit to keep track of empty slots in the slice of filters.
    id_and_valid: u32,
}

impl Filter {
    /// Creates a filter
    ///
    /// If the mask or ID is too large to fit into 29 bits, it will be silently truncated.
    #[inline]
    pub fn new(mask: u32, id: u32) -> Self {
        Filter {
            mask: mask & EXTENDED_ID_MASK,
            id_and_valid: (id & EXTENDED_ID_MASK) | VALID_BIT,
        }
    }
    /// Creates a filter that matches exactly one message ID
    ///
    /// If the ID is too larg to fit into 29 bits, it will be silently truncated.
    pub fn exact_match(id: u32) -> Self {
        Filter::new(EXTENDED_ID_MASK, id)
    }

    /// Returns the mask of this filter, which indicates the bits that are checked
    #[inline]
    pub fn mask(&self) -> u32 {
        self.mask
    }
    /// Returns the message ID that this filter (partially) matches
    #[inline]
    pub fn id(&self) -> u32 {
        self.id_and_valid & EXTENDED_ID_MASK
    }

    /// Returns true if this filter is valid
    fn is_valid(&self) -> bool {
        (self.id_and_valid & VALID_BIT) != 0
    }
    /// Marks this filter as not valid and resets its mask and ID
    fn invalidate(&mut self) {
        self.mask = 0;
        self.id_and_valid = 0;
    }

    /// Returns the number of 1 bits in the mask
    ///
    /// A higher rank means that the filter will reject more messages.
    fn rank(&self) -> u32 {
        self.mask.count_ones()
    }

    /// Returns true if this filter accepts a message with the provided ID
    pub fn accepts(&self, id: u32) -> bool {
        (self.mask() & id) == (self.mask() & self.id())
    }
}

mod debug_impl {
    use super::Filter;
    use core::fmt::{Debug, Formatter, Result};

    impl Debug for Filter {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Filter")
                .field("valid", &self.is_valid())
                .field("mask", &DebugHex(self.mask()))
                .field("id", &DebugHex(self.id()))
                .finish()
        }
    }

    struct DebugHex(u32);
    impl Debug for DebugHex {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{:#010x}", self.0)
        }
    }
}

fn merge_masks(a: &Filter, b: &Filter) -> u32 {
    a.mask() & b.mask() & !(a.id() ^ b.id())
}

/// Merges two filters, producing a new filter that accepts the union of the IDs accepted by the
/// two input filters (and possibly more IDs)
fn merge(a: &Filter, b: &Filter) -> Filter {
    let mask = merge_masks(a, b);
    Filter::new(mask, a.id() & mask)
}

/// Combines a slice of ideal filters down to max_filters filters that will accept a superset
/// of the message IDs of the ideal filters
///
/// The returned slice will be a sub-slice of ideal_filters.
///
/// If max_filters is zero, this function returns an empty slice. If max_filters is greater than
/// the length of ideal_filters, this function returns ideal_filters.
pub fn optimize(ideal_filters: &mut [Filter], max_filters: usize) -> &[Filter] {
    if max_filters == 0 {
        // Can't really do anything when nothing can be filtered
        return &[];
    }

    let working_filters = ideal_filters;
    // Step 1: Merge filters
    merge_filters(working_filters, max_filters);

    // In debug mode, check that not too many filters remain
    debug_assert!(
        working_filters
            .iter()
            .filter(|filter| filter.is_valid())
            .count()
            <= max_filters
    );
    // Step 2: Compact
    compact(working_filters);
    // Step 3: Return only the beginning part of the slice that contains valid filters
    let first_invalid = working_filters
        .iter()
        .position(|filter| !filter.is_valid())
        .unwrap_or(working_filters.len());
    let (valid_filters, invalid_filters) = working_filters.split_at(first_invalid);

    #[cfg(debug_assertions)]
    {
        // Check that the filters have been correctly split into valid and invalid
        assert!(valid_filters.iter().all(Filter::is_valid));
        assert!(!invalid_filters.iter().any(Filter::is_valid));
    }
    #[cfg(not(debug_assertions))]
    let _ = invalid_filters;

    valid_filters
}

/// Merges filters so that a maximum of max_filters are valid
fn merge_filters(working_filters: &mut [Filter], max_filters: usize) {
    assert_ne!(max_filters, 0);
    let mut valid_filters = working_filters.len();
    while valid_filters > max_filters {
        // Find the pair of valid filters with the maximum rank when merged
        let mut max_rank = 0;
        let mut max_rank_indices = (0, 0);
        for i in 0..working_filters.len() {
            for j in (i + 1)..working_filters.len() {
                let filter1 = &working_filters[i];
                let filter2 = &working_filters[j];
                if filter1.is_valid() && filter2.is_valid() {
                    let rank = merge(filter1, filter2).rank();
                    if rank >= max_rank {
                        max_rank_indices = (i, j);
                        max_rank = rank;
                    }
                }
            }
        }
        // Merge those filters into the first, invalidate the second
        working_filters[max_rank_indices.0] = merge(
            &working_filters[max_rank_indices.0],
            &working_filters[max_rank_indices.1],
        );
        working_filters[max_rank_indices.1].invalidate();
        valid_filters -= 1;
        debug_assert_eq!(
            valid_filters,
            working_filters
                .iter()
                .filter(|filter| filter.is_valid())
                .count()
        );
    }
}

/// Moves all valid filter to the beginning of filters, and all invalid filters to the end
fn compact(filters: &mut [Filter]) {
    // This could use the core library sort functions, but they add a lot of code size
    // (about 5000 bytes for thumbvem-none-eabihf).
    // Do this simple thing, based on insertion sort, instead.
    // This is O(n^2), but the number of filters is likely to be less than 100.
    for i in 1..filters.len() {
        let mut j = i;
        while j != 0 && !filters[j - 1].is_valid() && filters[j].is_valid() {
            filters.swap(j - 1, j);
            j -= 1;
        }
    }
}

#[cfg(test)]
mod test_compact {
    use super::{compact, Filter};

    /// Returns an invalid filter
    fn invalid() -> Filter {
        let mut filter = Filter::new(0, 0);
        filter.invalidate();
        filter
    }
    /// Returns a valid filter
    fn valid() -> Filter {
        Filter::new(0, 0)
    }

    fn check(inputs: &mut [Filter]) {
        compact(inputs);
        let first_invalid_index = inputs
            .iter()
            .position(|filter| !filter.is_valid())
            .unwrap_or(inputs.len());
        let (valid, invalid) = inputs.split_at(first_invalid_index);
        assert!(valid.iter().all(Filter::is_valid));
        assert!(!invalid.iter().any(Filter::is_valid));
    }

    #[test]
    fn basics() {
        check(&mut []);
        check(&mut [valid()]);
        check(&mut [invalid()]);
        check(&mut [valid(), invalid()]);
        check(&mut [invalid(), valid()]);
        check(&mut [invalid(), invalid()]);
        check(&mut [valid(), valid()]);
    }

    fn valid_from_bit_8(value: u8, bit: u8) -> Filter {
        if ((value >> bit) & 1) == 1 {
            valid()
        } else {
            invalid()
        }
    }

    #[test]
    fn longer() {
        // Check all combinations of 8 valid and invalid filters
        for permutation in 0..=u8::MAX {
            let mut filters = [
                valid_from_bit_8(permutation, 0),
                valid_from_bit_8(permutation, 1),
                valid_from_bit_8(permutation, 2),
                valid_from_bit_8(permutation, 3),
                valid_from_bit_8(permutation, 4),
                valid_from_bit_8(permutation, 5),
                valid_from_bit_8(permutation, 6),
                valid_from_bit_8(permutation, 7),
            ];
            check(&mut filters);
        }
    }
}

#[cfg(test)]
mod test_single_merge {
    use super::{merge, Filter, EXTENDED_ID_MASK};

    /// Merge two filters that are the same, result should be equal to the inputs
    #[test]
    fn merge_two_equal() {
        let test_filters = [
            Filter::new(0x0, 0x0),
            Filter::new(0x1, 0x1),
            Filter::new(0x1, 0x0),
            Filter::new(0x3, 0x0),
            Filter::new(0x3, 0x1),
            Filter::new(0x3, 0x2),
            Filter::new(0x3, 0x3),
        ];

        for filter in test_filters.iter() {
            let combined = merge(filter, filter);
            assert_eq!(&combined, filter);
        }
    }

    /// Merge two filters that accept exactly one ID each
    ///
    /// The combination should accept both IDs
    #[test]
    fn merge_two_exact_id() {
        let ids = [
            (0x0, 0x0),
            (0x10, 0x0),
            (0x0, 0x10),
            (EXTENDED_ID_MASK, 0x0),
            (0x0, EXTENDED_ID_MASK),
            (0x12933, 0x12932),
        ];
        for &(id1, id2) in ids.iter() {
            let filter1 = Filter::new(EXTENDED_ID_MASK, id1);
            let filter2 = Filter::new(EXTENDED_ID_MASK, id2);
            assert!(filter1.accepts(id1));
            assert!(filter2.accepts(id2));
            let merged = merge(&filter1, &filter2);

            assert!(merged.accepts(id1));
            assert!(merged.accepts(id2));
        }
    }
}
