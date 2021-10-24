use std::cmp::min;
use std::collections::BTreeSet;
use std::convert::TryInto;
use std::ops::RangeToInclusive;

use itertools::Itertools;

#[derive(Debug, Clone)]
pub enum Operator {
    /// A fixed set of bit lengths
    ///
    /// The set of lengths must not be empty.
    Leaf(BTreeSet<u64>),
    /// Adds up to ``alignment - 1`` padding bits to each entry of the child to ensure that the
    /// values are aligned
    Padding {
        child: Box<Operator>,
        alignment: u32,
    },
    /// Given a set of children, transforms them into a single bit length set expression where each
    /// item is the elementwise sum of the cartesian product of the children's bit length sets
    Concatenate { children: Vec<Operator> },
    /// Concatenates ``count`` copies of the child.
    /// This is equivalent to `Concatenate` where the child is replicated ``count`` times.
    Repeat { child: Box<Operator>, count: u64 },
    /// Concatenates 0..=count.end copies of the child
    RangeRepeat {
        child: Box<Operator>,
        count: RangeToInclusive<u64>,
    },
    /// A union of one or more children
    Union { children: Vec<Operator> },
}

impl Operator {
    pub fn min(&self) -> u64 {
        match self {
            Operator::Leaf(values) => *SetMinMax::min(values).unwrap(),
            Operator::Padding { child, alignment } => align_up(child.min(), u64::from(*alignment)),
            Operator::Concatenate { children } => {
                // Sum of child minimum lengths
                children.iter().map(Operator::min).sum()
            }
            Operator::Repeat { child, count } => child.min() * *count,
            Operator::RangeRepeat { .. } => 0,
            Operator::Union { children } => children.iter().map(Operator::min).min().unwrap(),
        }
    }

    pub fn max(&self) -> u64 {
        match self {
            Operator::Leaf(values) => *SetMinMax::max(values).unwrap(),
            Operator::Padding { child, alignment } => align_up(child.max(), u64::from(*alignment)),
            Operator::Concatenate { children } => {
                // Sum of child maximum lengths
                children.iter().map(Operator::max).sum()
            }
            Operator::Repeat { child, count } => child.max() * *count,
            Operator::RangeRepeat { child, count } => child.max() * count.end,
            Operator::Union { children } => children.iter().map(Operator::max).max().unwrap(),
        }
    }

    pub fn modulo(&self, divisor: u64) -> BTreeSet<u64> {
        match self {
            Operator::Leaf(values) => {
                // Apply to each element
                values.iter().map(|length| *length % divisor).collect()
            }
            Operator::Padding { child, alignment } => {
                let max = self.max();
                let lcm = num_integer::lcm(u64::from(*alignment), divisor);
                child
                    .modulo(divisor)
                    .into_iter()
                    .map(|value| {
                        debug_assert!(value <= max);
                        debug_assert!(value < lcm);
                        align_up(value, u64::from(*alignment)) % divisor
                    })
                    .collect()
            }
            Operator::Concatenate { children } => {
                let child_mods: Vec<BTreeSet<u64>> =
                    children.iter().map(|child| child.modulo(divisor)).collect();
                child_mods
                    .iter()
                    .map(|mods| mods.iter().copied())
                    .multi_cartesian_product()
                    // For each possible combination of child sizes, add them up and then modulo
                    .map(|values: Vec<u64>| values.into_iter().sum())
                    .map(|sum: u64| sum % divisor)
                    .collect()
            }
            Operator::Repeat { child, count } => {
                let equivalent_count = min(*count, divisor + *count % divisor);
                debug_assert_eq!(*count % divisor, equivalent_count % divisor);

                child
                    .modulo(divisor)
                    .into_iter()
                    .combinations_with_replacement(
                        equivalent_count
                            .try_into()
                            .expect("equivalent_count too large for usize"),
                    )
                    .map(|values| values.into_iter().sum::<u64>() % divisor)
                    .collect()
            }
            Operator::RangeRepeat { child, count } => {
                let single = child.modulo(divisor);
                let count_max = count.end;
                let equivalent_count_max = min(count_max, divisor + count_max % divisor);
                debug_assert_eq!(count_max % divisor, equivalent_count_max % divisor);

                (0..=equivalent_count_max)
                    .flat_map(|k| {
                        single
                            .iter()
                            .copied()
                            .combinations_with_replacement(
                                k.try_into().expect("k too large for usize"),
                            )
                            .map(|values| values.into_iter().sum::<u64>() % divisor)
                    })
                    .collect()
            }
            Operator::Union { children } => children
                .iter()
                .flat_map(|child| child.modulo(divisor))
                .collect(),
        }
    }

    /// Expands this bit length set and returns a set with all enclosed values
    pub fn expand(&self) -> BTreeSet<u64> {
        match self {
            Operator::Leaf(values) => values.clone(),
            Operator::Padding { child, alignment } => child
                .expand()
                .into_iter()
                .map(|length| align_up(length, u64::from(*alignment)))
                .collect(),
            Operator::Concatenate { children } => {
                let child_values: Vec<BTreeSet<u64>> =
                    children.iter().map(Operator::expand).collect();
                child_values
                    .iter()
                    .multi_cartesian_product()
                    .map(|child_sizes| child_sizes.into_iter().sum())
                    .collect()
            }
            Operator::Repeat { child, count } => child
                .expand()
                .into_iter()
                .combinations_with_replacement(
                    (*count).try_into().expect("count too large for usize"),
                )
                .map(|child_sizes| child_sizes.into_iter().sum())
                .collect(),
            Operator::RangeRepeat { child, count } => {
                let ch = child.expand();
                (0..=count.end)
                    .flat_map(|k| {
                        ch.iter()
                            .copied()
                            .combinations_with_replacement(
                                k.try_into().expect("k too large for usize"),
                            )
                            .map(|child_sizes| child_sizes.into_iter().sum())
                    })
                    .collect()
            }
            Operator::Union { children } => children.iter().flat_map(Operator::expand).collect(),
        }
    }

    /// Checks that the calculated minimum, maximum, and some remainders of this set match
    /// the values calculated by expanding this set
    pub fn validate_numerically(&self) {
        let expanded = self.expand();
        assert!(!expanded.is_empty());

        assert_eq!(*SetMinMax::min(&expanded).unwrap(), self.min());
        assert_eq!(*SetMinMax::max(&expanded).unwrap(), self.max());

        for i in 1..=64 {
            let mod_expanded = self.modulo(i);
            let expected: BTreeSet<u64> = expanded.iter().map(|value| value % i).collect();
            assert_eq!(mod_expanded, expected, "Incorrect modulo for {}", i);
        }
    }
}

/// Rounds the value up to a multiple of alignment
fn align_up(value: u64, alignment: u64) -> u64 {
    (value + (alignment - 1)) / alignment * alignment
}

#[cfg(test)]
mod test_align_up {
    use super::align_up;

    #[test]
    fn align_up_simple() {
        assert_eq!(0, align_up(0, 8));
        assert_eq!(8, align_up(1, 8));
        assert_eq!(8, align_up(2, 8));
        assert_eq!(8, align_up(3, 8));
        assert_eq!(8, align_up(4, 8));
        assert_eq!(8, align_up(5, 8));
        assert_eq!(8, align_up(6, 8));
        assert_eq!(8, align_up(7, 8));
        assert_eq!(8, align_up(8, 8));
        assert_eq!(16, align_up(9, 8));
    }

    #[test]
    fn align_up_odd() {
        assert_eq!(0, align_up(0, 3));
        assert_eq!(3, align_up(1, 3));
        assert_eq!(3, align_up(2, 3));
        assert_eq!(3, align_up(3, 3));
        assert_eq!(6, align_up(4, 3));
        assert_eq!(6, align_up(5, 3));
        assert_eq!(6, align_up(6, 3));
        assert_eq!(9, align_up(7, 3));
    }
}

// Needed until `map_first_last` is specialized: https://github.com/rust-lang/rust/issues/62924
trait SetMinMax<T> {
    /// Returns the minimum value in this set, or None if this set is empty
    fn min(&self) -> Option<&T>;
    /// Returns the maximum value in this set, or None if this set is empty
    fn max(&self) -> Option<&T>;
}
impl<T> SetMinMax<T> for BTreeSet<T>
where
    T: PartialOrd,
{
    fn min(&self) -> Option<&T> {
        self.iter().next()
    }

    fn max(&self) -> Option<&T> {
        self.iter().next_back()
    }
}
