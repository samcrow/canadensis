//! Constants declared in DSDL files

pub use crate::types::constant::{Constant, ConstantValue};

/// A mapping from constant names to values
///
/// This preserves the order of constants in the source DSDL file.
// This wrapper obscures the inner type.
#[derive(Debug, Clone, Default)]
pub struct Constants(Vec<(String, Constant)>);

impl Constants {
    /// Returns true if a constant with the provided name is already stored
    pub(crate) fn contains_key(&self, name: &str) -> bool {
        self.0
            .iter()
            .find(|(entry_name, _)| entry_name == name)
            .is_some()
    }

    /// Inserts a constant
    ///
    /// If a constant with the provided name already exists, the new constant replaces it.
    pub(crate) fn insert(&mut self, name: String, constant: Constant) {
        if let Some(entry) = self
            .0
            .iter_mut()
            .find(|(entry_name, _)| entry_name == &name)
        {
            entry.1 = constant;
        } else {
            self.0.push((name, constant));
        }
    }

    /// Returns a reference to the constant with the provided name, if one exists
    pub fn get(&self, name: &str) -> Option<&Constant> {
        self.0
            .iter()
            .find(|(entry_name, _)| entry_name == name)
            .map(|(_, constant)| constant)
    }

    /// Returns a mutable reference to the constant added most recently
    pub(crate) fn last_mut(&mut self) -> Option<&mut Constant> {
        self.0.last_mut().map(|(_, constant)| constant)
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
pub struct Iter<'c>(std::slice::Iter<'c, (String, Constant)>);

impl<'c> Iterator for Iter<'c> {
    type Item = (&'c String, &'c Constant);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(map_refs)
    }
}

/// An iterator over constant names and values
pub struct IntoIter(std::vec::IntoIter<(String, Constant)>);

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

/// Converts a reference to a tuple into a tuple of references
fn map_refs((name, constant): &(String, Constant)) -> (&String, &Constant) {
    (name, constant)
}
