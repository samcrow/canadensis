extern crate itertools;
extern crate num_integer;

mod operator;

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::{RangeToInclusive, Rem};
use std::{iter, mem};

use crate::operator::Operator;

/// A non-empty set of possible lengths (in bits) for a data type
///
/// This is based on the Python version: <https://github.com/OpenCyphal/pydsdl/blob/master/pydsdl/_bit_length_set/_bit_length_set.py>
#[derive(Debug, Clone)]
pub struct BitLengthSet {
    operator: Operator,
}

impl BitLengthSet {
    /// Creates a bit length set with one length value in bits
    pub fn single(length: u64) -> BitLengthSet {
        let mut values = BTreeSet::new();
        values.insert(length);
        BitLengthSet {
            operator: Operator::Leaf(values),
        }
    }
    /// Creates a bit length set from an iterator of possible length values
    ///
    /// If the provided iterator does not yield any elements, this function returns None.
    pub fn from_lengths<I>(values: I) -> Option<BitLengthSet>
    where
        I: IntoIterator<Item = u64>,
    {
        let value_set: BTreeSet<u64> = values.into_iter().collect();
        if value_set.is_empty() {
            None
        } else {
            Some(BitLengthSet {
                operator: Operator::Leaf(value_set),
            })
        }
    }

    /// Creates a bit length set from a set of possible length values
    ///
    /// If the provided set is empty, this function returns None.
    pub fn from_length_set(values: BTreeSet<u64>) -> Option<BitLengthSet> {
        if values.is_empty() {
            None
        } else {
            Some(BitLengthSet {
                operator: Operator::Leaf(values),
            })
        }
    }

    /// Returns the minimum length value in this set
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::single(37);
    /// assert_eq!(37, lengths.min_value());
    /// ```
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::from_lengths([1, 10, 30]).unwrap();
    /// assert_eq!(1, lengths.min_value());
    /// ```
    pub fn min_value(&self) -> u64 {
        self.operator.min()
    }
    /// Returns the maximum length value in this set
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::single(37);
    /// assert_eq!(37, lengths.max_value());
    /// ```
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::from_lengths([1, 10, 30]).unwrap();
    /// assert_eq!(30, lengths.max_value());
    /// ```
    pub fn max_value(&self) -> u64 {
        self.operator.max()
    }

    /// Returns true if this set's minimum and maximum values are equal
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::single(37);
    /// assert!(lengths.is_fixed_size());
    /// ```
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::from_lengths([1, 10, 30]).unwrap();
    /// assert!(!lengths.is_fixed_size());
    /// ```
    pub fn is_fixed_size(&self) -> bool {
        self.min_value() == self.max_value()
    }

    /// Returns true if all values in this set are aligned to multiples of 8 bits
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// assert!(BitLengthSet::single(0).is_byte_aligned());
    /// assert!(BitLengthSet::single(8).is_byte_aligned());
    /// assert!(BitLengthSet::single(16).is_byte_aligned());
    ///
    /// assert!(!BitLengthSet::single(1).is_byte_aligned());
    /// assert!(!BitLengthSet::single(2).is_byte_aligned());
    /// assert!(!BitLengthSet::single(4).is_byte_aligned());
    /// assert!(!BitLengthSet::single(7).is_byte_aligned());
    ///
    /// assert!(!BitLengthSet::from_lengths([0, 1, 8, 16, 24]).unwrap().is_byte_aligned());
    /// assert!(!BitLengthSet::from_lengths([8, 9]).unwrap().is_byte_aligned());
    ///
    /// assert!(BitLengthSet::from_lengths([0, 8, 16, 24]).unwrap().is_byte_aligned());
    /// assert!(BitLengthSet::from_lengths([8, 16]).unwrap().is_byte_aligned());
    /// ```
    pub fn is_byte_aligned(&self) -> bool {
        self.is_aligned(8)
    }

    /// Returns true if all values in this set are aligned to multiples of the specified number of
    /// bits
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// assert!(BitLengthSet::from_lengths([0, 3, 6, 15]).unwrap().is_aligned(3));
    /// assert!(!BitLengthSet::from_lengths([0, 3, 6, 16]).unwrap().is_aligned(3));
    ///
    /// assert!(BitLengthSet::from_lengths([0, 17, 34, 68]).unwrap().is_aligned(17));
    /// assert!(!BitLengthSet::from_lengths([0, 17, 34, 64]).unwrap().is_aligned(17));
    /// ```
    pub fn is_aligned(&self, bit_length: u64) -> bool {
        let remainder = self % bit_length;
        remainder.is_fixed_size() && remainder.min_value() == 0
    }

    /// Expands this bit length set and returns a set with all enclosed values
    ///
    /// For some bit length sets, especially when large sets are concatenated, this may be slow.
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// # use std::collections::BTreeSet;
    /// let lengths1 = BitLengthSet::from_lengths([0, 8, 24, 96]).unwrap();
    /// let lengths2 = BitLengthSet::from_lengths([1, 2, 8]).unwrap();
    ///
    /// let union = lengths1.unite([lengths2]);
    /// let expanded_union = union.expand();
    ///
    /// let expected: BTreeSet<u64> = [0, 1, 2, 8, 24, 96].iter().copied().collect();
    /// assert_eq!(expected, expanded_union);
    /// ```
    pub fn expand(&self) -> BTreeSet<u64> {
        self.operator.expand()
    }

    /// Converts this bit length set into a new set that aligns up all its values to a multiple
    /// of the given alignment
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::from_lengths([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]).unwrap();
    /// let padded = lengths.pad_to_alignment(8);
    ///
    /// let expected = BitLengthSet::from_lengths([0, 8, 16]).unwrap();
    /// assert_eq!(padded.expand(), expected.expand());
    /// ```
    pub fn pad_to_alignment(self, alignment: u32) -> BitLengthSet {
        BitLengthSet {
            operator: Operator::Padding {
                child: Box::new(self.operator),
                alignment,
            },
        }
    }

    /// Converts this bit length set into a new set that represents a repetition of these values
    /// a fixed number of times
    ///
    /// # Examples
    ///
    /// ## Single length
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let length = BitLengthSet::single(12);
    /// let repeated = length.repeat(4);
    ///
    /// let expected = BitLengthSet::single(12 * 4);
    /// assert_eq!(repeated.expand(), expected.expand());
    /// ```
    ///
    /// ## Multiple lengths
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::from_lengths([0, 1, 8]).unwrap();
    /// let repeated = lengths.repeat(4);
    ///
    /// let expected = BitLengthSet::from_lengths(
    ///     [0, 1, 2, 3, 4, 8, 9, 10, 11, 16, 17, 18, 24, 25, 32]
    /// ).unwrap();
    /// assert_eq!(repeated.expand(), expected.expand());
    /// ```
    ///
    pub fn repeat(self, count: u64) -> BitLengthSet {
        BitLengthSet {
            operator: Operator::Repeat {
                child: Box::new(self.operator),
                count,
            },
        }
    }

    /// Converts this bit length set into a new set that represents a repetition of these values
    /// zero times up to an inclusive maximum count
    ///
    /// # Examples
    ///
    /// ## Single length
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::single(3);
    /// let repeated = lengths.repeat_range(..=8);
    ///
    /// let expected = BitLengthSet::from_lengths([0, 3, 6, 9, 12, 15, 18, 21, 24]).unwrap();
    /// assert_eq!(repeated.expand(), expected.expand());
    /// ```
    ///
    /// ## Multiple lengths
    ///
    /// ```
    /// # use canadensis_bit_length_set::BitLengthSet;
    /// let lengths = BitLengthSet::from_lengths([1, 2, 4]).unwrap();
    /// let repeated = lengths.repeat_range(..=2);
    ///
    /// let expected = BitLengthSet::from_lengths([0, 1, 2, 3, 4, 5, 6, 8]).unwrap();
    /// assert_eq!(repeated.expand(), expected.expand());
    /// ```
    pub fn repeat_range(self, count: RangeToInclusive<u64>) -> BitLengthSet {
        BitLengthSet {
            operator: Operator::RangeRepeat {
                child: Box::new(self.operator),
                count,
            },
        }
    }

    /// Extends this bit length set, so that this set represents a concatenation of this set
    /// followed by other sets
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::{BitLengthSet, bit_length};
    ///
    /// let combined = bit_length![0, 8].concatenate([bit_length![12]]);
    /// let expected = bit_length![12, 20];
    /// assert_eq!(combined.expand(), expected.expand());
    /// ```
    ///
    /// ```
    /// # use canadensis_bit_length_set::{BitLengthSet, bit_length};
    ///
    /// let combined = bit_length![0, 1, 3].concatenate([bit_length![5, 13]]);
    /// let expected = bit_length![5, 6, 8, 13, 14, 16];
    /// assert_eq!(combined.expand(), expected.expand());
    /// ```
    pub fn concatenate<I>(mut self, others: I) -> BitLengthSet
    where
        I: IntoIterator<Item = BitLengthSet>,
    {
        self.extend(others);
        self
    }

    /// Converts this bit length set into a new set that is the union of this set and the provided
    /// other sets
    ///
    /// This is uses to represent a union (or enum in Rust)
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::{BitLengthSet, bit_length};
    ///
    /// let combined = bit_length![0, 1, 3, 24].unite([bit_length![3, 8]]);
    /// let expected = bit_length![0, 1, 3, 8, 24];
    /// assert_eq!(combined.expand(), expected.expand());
    /// ```
    pub fn unite<I>(self, others: I) -> BitLengthSet
    where
        I: IntoIterator<Item = BitLengthSet>,
    {
        let other_operators = others.into_iter().map(|set| set.operator);
        BitLengthSet {
            operator: Operator::Union {
                children: iter::once(self.operator).chain(other_operators).collect(),
            },
        }
    }

    /// Checks that the expanded form of this set has the same properties as calculated based on
    /// the internal representation of this set
    pub fn validate_numerically(&self) {
        self.operator.validate_numerically()
    }

    /// Returns true if this set has the same structure of operators as another set
    ///
    /// If this function returns true, the expanded forms of self and other are equal. If this
    /// function returns false, the expanded forms of self and other may or may not be equal.
    fn structural_equal(&self, other: &Self) -> bool {
        self.operator.structural_equal(&other.operator)
    }
}

impl Default for BitLengthSet {
    /// Returns the default bit length set, which contains the value 0
    fn default() -> Self {
        BitLengthSet::single(0)
    }
}

impl Rem<u64> for BitLengthSet {
    type Output = BitLengthSet;

    /// Calculates the elementwise modulo of each value in this set
    fn rem(self, rhs: u64) -> Self::Output {
        // Delegate to version that takes &self
        Rem::rem(&self, rhs)
    }
}

impl Rem<u64> for &'_ BitLengthSet {
    type Output = BitLengthSet;

    /// Calculates the elementwise modulo of each value in this set
    ///
    /// # Examples
    ///
    /// ```
    /// # use canadensis_bit_length_set::{BitLengthSet, bit_length};
    ///
    /// assert_eq!((bit_length![0] % 12345).expand(), bit_length![0].expand());
    /// assert_eq!((bit_length![34] % 17).expand(), bit_length![0].expand());
    /// assert_eq!((bit_length![8, 12, 16] % 8).expand(), bit_length![0, 4].expand());
    /// assert_eq!((bit_length![0, 3, 8, 9, 27] % 8).expand(), bit_length![0, 1, 3].expand());
    ///
    /// ```
    fn rem(self, rhs: u64) -> Self::Output {
        let result = self.operator.modulo(rhs);
        debug_assert!(!result.is_empty());
        BitLengthSet {
            operator: Operator::Leaf(result),
        }
    }
}

impl Extend<BitLengthSet> for BitLengthSet {
    /// Extends this bit length set, so that this set represents a concatenation of this set
    /// followed by other sets
    ///
    /// This is equivalent to [`concatenate`](#method.concatenate).
    fn extend<T: IntoIterator<Item = BitLengthSet>>(&mut self, iter: T) {
        let other_operators = iter.into_iter().map(|set| set.operator);
        // Take out the operator from self and temporarily replace it with an invalid empty operator
        let self_operator = mem::replace(&mut self.operator, Operator::Leaf(BTreeSet::default()));
        self.operator = Operator::Concatenate {
            children: iter::once(self_operator).chain(other_operators).collect(),
        }
    }
}

impl PartialEq<Self> for BitLengthSet {
    fn eq(&self, other: &Self) -> bool {
        // There may be a more optimized way to do this
        if self.structural_equal(other) {
            true
        } else {
            self.expand() == other.expand()
        }
    }
}

impl Eq for BitLengthSet {}

impl PartialOrd for BitLengthSet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BitLengthSet {
    fn cmp(&self, other: &Self) -> Ordering {
        // There may be a more optimized way to do this, without expanding the sets
        self.expand().cmp(&other.expand())
    }
}

/// A convenience macro for creating a `BitLengthSet`
///
/// This macro takes one or more arguments and evaluates to a `BitLengthSet` value.
///
/// # Examples
///
/// ```
/// # use canadensis_bit_length_set::bit_length;
/// bit_length![8];
/// bit_length![0, 8, 10];
/// bit_length![0, 8, 10,];
/// ```
///
/// This macro will produce a compiler error if you attempt to call it with no arguments:
///
/// ```compile_fail
/// # use canadensis_bit_length_set::bit_length;
/// bit_length![];
/// ```
///
#[macro_export]
macro_rules! bit_length {
    {$single:expr} => {
        $crate::BitLengthSet::single($single)
    };
    {$first:expr, $($others:expr),+ $(,)?} => {
        $crate::BitLengthSet::from_lengths([$first, $($others),+]).unwrap()
    };
}
