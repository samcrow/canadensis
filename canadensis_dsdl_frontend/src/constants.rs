//! Constants declared in DSDL files

use std::collections::btree_map::{self, BTreeMap};

pub use crate::types::constant::{Constant, ConstantValue};

/// A mapping from constant names to values
// This wrapper obscures the type of map that is being used.
#[derive(Debug, Clone)]
pub struct Constants(BTreeMap<String, Constant>);

impl Constants {
    pub(crate) fn from_map(constants: BTreeMap<String, Constant>) -> Self {
        Constants(constants)
    }

    /// Returns a reference to the constant with the provided name, if one exists
    pub fn get(&self, name: &str) -> Option<&Constant> {
        self.0.get(name)
    }

    /// Returns an iterator over the constant names and values
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.0.iter())
    }
}

impl<'c> IntoIterator for &'c Constants {
    type Item = (&'c String, &'c Constant);
    type IntoIter = Iter<'c>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.0.iter())
    }
}

/// An iterator over constant names and values
pub struct Iter<'c>(btree_map::Iter<'c, String, Constant>);

impl<'c> Iterator for Iter<'c> {
    type Item = (&'c String, &'c Constant);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// An iterator over constant names and values
pub struct IntoIter(btree_map::IntoIter<String, Constant>);

impl Iterator for IntoIter {
    type Item = (String, Constant);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl IntoIterator for Constants {
    type Item = (String, Constant);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}
