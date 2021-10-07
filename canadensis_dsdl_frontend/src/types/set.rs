use crate::types::{ExprType, Value};
use num_rational::BigRational;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::iter::FromIterator;

/// A set of values
///
/// A set has an element type if and only if its not empty. If it is not empty, all its elements
/// have the same type.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Set(BTreeSet<Value>);

impl Set {
    pub fn new() -> Self {
        Set(BTreeSet::new())
    }

    /// Returns the number of values in this set
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns a borrowed iterator over values in this set
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }

    /// Returns the type of this set, or None if this set is empty
    pub fn ty(&self) -> Option<ExprType> {
        self.0.iter().next().map(Value::ty)
    }

    /// Returns the minimum value of this set, if one exists
    ///
    /// This function returns None if the set is empty, or if the < operator is not defined on the
    /// element type.
    pub fn min_value(&self) -> Option<Value> {
        match self.ty() {
            Some(ExprType::Rational) => {
                // This is easy. They're already sorted.
                self.0.iter().next().cloned()
            }
            Some(ExprType::Set(_)) => {
                // Need to copy values out and sort by proper subset relationship
                Some(Value::Set(self.sets_sorted().remove(0)))
            }
            Some(ExprType::String) => {
                // This is defined only if all the strings can be implicitly converted to integers
                let sorted_values = self.string_as_ints_sorted()?;
                let min = sorted_values[0];
                Some(Value::String(single_character_string(min).into()))
            }
            _ => {
                // Not supported
                None
            }
        }
    }

    /// Returns the maximum value of this set, if one exists
    ///
    /// This function returns None if the set is empty, or if the < operator is not defined on the
    /// element type.
    pub fn max_value(&self) -> Option<Value> {
        match self.ty() {
            Some(ExprType::Rational) => {
                // This is easy. They're already sorted.
                self.0.iter().next_back().cloned()
            }
            Some(ExprType::Set(_)) => {
                // Need to copy values out and sort by proper subset relationship
                Some(Value::Set(self.sets_sorted().pop().unwrap()))
            }
            Some(ExprType::String) => {
                // This is defined only if all the strings can be implicitly converted to integers
                let sorted_values = self.string_as_ints_sorted()?;
                let max = *sorted_values.last().unwrap();
                Some(Value::String(single_character_string(max).into()))
            }
            _ => {
                // Not supported
                None
            }
        }
    }

    /// If this set contains sets, this function copies them and sorts them by proper subset
    /// relationship: If a is a proper subset of b, a is before b in the returned vec.
    ///
    /// This function panics if self contains an element that is not a set.
    fn sets_sorted(&self) -> Vec<Set> {
        let mut inner_sets: Vec<Set> = self
            .0
            .iter()
            .map(|element| match element {
                Value::Set(set) => set.clone(),
                _ => panic!("Set contains a non-set element"),
            })
            .collect();

        inner_sets.sort_by(|a, b| {
            // If a is a subset of b and a != b, less
            // Otherwise, equal
            if a.is_subset(b) && a != b {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });

        inner_sets
    }

    /// If every value in this set is a string with an implicit integer value, this function returns
    /// a sorted vector of those values.
    fn string_as_ints_sorted(&self) -> Option<Vec<u8>> {
        let mut as_ints = Vec::with_capacity(self.len());
        for value in &self.0 {
            match value {
                Value::String(s) => match s.implicit_int() {
                    Some(int_value) => as_ints.push(int_value),
                    None => return None,
                },
                _ => return None,
            }
        }
        as_ints.sort_unstable();
        Some(as_ints)
    }

    /// Attempts to insert a value into this set
    ///
    /// If this set is empty, or if this set is not empty and has a type that matches the type
    /// of the provided value, the operation succeeds (even if this set already contained a matching
    /// value).
    ///
    /// A string containing one ASCII character can also be added to a set of rationals. The
    /// character will be converted into its numerical value.
    ///
    /// Otherwise, the types do not match and the value is rejected.
    pub fn insert(&mut self, value: Value) -> Result<(), TypeError> {
        if let Some(ty) = self.ty() {
            if ty == value.ty() {
                self.0.insert(value);
                Ok(())
            } else {
                // Check if the value is a string that can be implicitly converted into an integer
                match value {
                    Value::String(value)
                        if value.implicit_int().is_some() && ty == ExprType::Rational =>
                    {
                        self.0.insert(Value::Rational(BigRational::from_integer(
                            value.implicit_int().unwrap().into(),
                        )));
                        Ok(())
                    }
                    _ => Err(TypeError {
                        left: ty,
                        right: value.ty(),
                    }),
                }
            }
        } else {
            // Adding the first element
            self.0.insert(value);
            Ok(())
        }
    }

    /// Returns true if two sets have compatible types
    ///
    /// Sets are compatible if either one is empty, or if both are non-empty and have the same
    /// type.
    ///
    /// Sets that are both non-empty and have different types are not compatible.
    pub fn is_compatible(&self, rhs: &Set) -> bool {
        self.check_types_with(rhs).is_ok()
    }
    /// Returns true if this set is either empty or contains elements of the specified type
    pub fn can_hold(&self, ty: ExprType) -> bool {
        Self::check_element_types(self.ty(), Some(ty)).is_ok()
    }

    fn check_types_with(&self, other: &Set) -> Result<(), TypeError> {
        Self::check_element_types(self.ty(), other.ty())
    }

    /// Checks that two optional element types are compatible
    fn check_element_types(
        left: Option<ExprType>,
        right: Option<ExprType>,
    ) -> Result<(), TypeError> {
        match (left, right) {
            (None, _) | (_, None) => Ok(()),
            (Some(left), Some(right)) => {
                if left == right {
                    Ok(())
                } else {
                    Err(TypeError { left, right })
                }
            }
        }
    }

    /// Returns true if this is a subset of another set
    pub fn is_subset(&self, other: &Set) -> bool {
        self.0.is_subset(&other.0)
    }
    /// Returns true if this is a superset of another set
    pub fn is_superset(&self, other: &Set) -> bool {
        self.0.is_superset(&other.0)
    }

    /// Calculates and returns the union of this set and another set
    pub fn union(&self, other: &Set) -> Result<Set, TypeError> {
        self.check_types_with(other)?;
        Ok(Set(self.0.union(&other.0).cloned().collect()))
    }

    /// Calculates and returns the intersection of this set and another set
    pub fn intersection(&self, other: &Set) -> Result<Set, TypeError> {
        self.check_types_with(other)?;
        Ok(Set(self.0.intersection(&other.0).cloned().collect()))
    }

    /// Calculates and returns the symmetric difference of this set and another set
    pub fn symmetric_difference(&self, other: &Set) -> Result<Set, TypeError> {
        self.check_types_with(other)?;
        Ok(Set(self
            .0
            .symmetric_difference(&other.0)
            .cloned()
            .collect()))
    }
}

impl From<Set> for BTreeSet<Value> {
    fn from(set: Set) -> Self {
        set.0
    }
}

impl IntoIterator for Set {
    type Item = Value;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

/// A consuming iterator over values in a set
pub(crate) struct IntoIter(std::collections::btree_set::IntoIter<Value>);

impl Iterator for IntoIter {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<'s> IntoIterator for &'s Set {
    type Item = &'s Value;
    type IntoIter = Iter<'s>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// A borrowed iterator over values in a set
pub(crate) struct Iter<'s>(std::collections::btree_set::Iter<'s, Value>);

impl<'s> Iterator for Iter<'s> {
    type Item = &'s Value;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl FromIterator<Value> for Result<Set, TypeError> {
    /// Collects a set from an iterator of elements
    ///
    /// The set's type is defined by the first element. If any subsequent element has a different
    /// type, the result is an error.
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        match iter.next() {
            Some(first) => {
                let mut set = Set::new();
                set.insert(first)
                    .expect("set started empty and accepts any value");

                for item in iter {
                    set.insert(item)?;
                }

                Ok(set)
            }
            None => {
                // No first element, empty set
                Ok(Set::new())
            }
        }
    }
}

impl Default for Set {
    fn default() -> Self {
        Set::new()
    }
}

fn single_character_string(code: u8) -> String {
    let mut string = String::with_capacity(1);
    string.push(code as char);
    string
}

/// An error resulting from an operation with two sets of incompatible element types
#[derive(Debug, PartialEq)]
pub(crate) struct TypeError {
    /// The element type of the left set
    pub left: ExprType,
    /// The element type of the right set
    pub right: ExprType,
}

mod fmt_impl {
    use super::{Set, TypeError};
    use std::fmt::{Display, Formatter, Result, Write};

    impl Display for TypeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "type mismatch between {} and {}", self.left, self.right)
        }
    }

    impl std::error::Error for TypeError {}

    impl Display for Set {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.write_char('{')?;

            let len = self.len();
            for (i, value) in self.iter().enumerate() {
                write!(f, "{}", value)?;
                if i != len - 1 {
                    // Not the last value, add a separator
                    f.write_str(", ")?;
                }
            }
            f.write_char('}')?;
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::Set;
    use crate::types::Value;
    use num_rational::BigRational;
    use std::iter;

    #[test]
    fn test_from_iter_empty() {
        assert_eq!(Ok(Set::default()), iter::empty().collect())
    }
    #[test]
    fn test_from_iter_one() {
        let mut expected = Set::new();
        expected.insert(Value::Boolean(true)).unwrap();
        assert_eq!(Ok(expected), iter::once(Value::Boolean(true)).collect())
    }
    #[test]
    fn test_from_iter_rational_and_convert_string() {
        let mut expected = Set::new();
        expected
            .insert(Value::Rational(BigRational::from_integer(3.into())))
            .unwrap();
        // 0x20 is the numerical value of ASCII space
        expected
            .insert(Value::Rational(BigRational::from_integer(0x20.into())))
            .unwrap();

        assert_eq!(
            Ok(expected),
            [
                Value::Rational(BigRational::from_integer(3.into())),
                Value::String(" ".to_owned().into())
            ]
            .iter()
            .cloned()
            .collect()
        );
    }
}
