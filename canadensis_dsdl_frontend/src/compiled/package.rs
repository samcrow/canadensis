//! A package of compiled data types

use crate::compiled::CompiledDsdl;
use crate::type_key::TypeKey;
use crate::warning::Warnings;
use std::collections::btree_map;
use std::collections::BTreeMap;

/// A package of types compiled from DSDL files
#[derive(Debug)]
pub struct CompiledPackage {
    /// Each compiled DSDL type
    types: BTreeMap<TypeKey, CompiledDsdl>,
    /// Warnings reported while compiling
    warnings: Warnings,
}

impl CompiledPackage {
    pub(crate) fn new(types: BTreeMap<TypeKey, CompiledDsdl>, warnings: Warnings) -> Self {
        CompiledPackage { types, warnings }
    }

    /// Returns a reference to the type with the provided key
    #[inline]
    pub fn get_by_key(&self, key: &TypeKey) -> Option<&CompiledDsdl> {
        self.types.get(key)
    }

    /// Removes and returns the type with the provided key
    #[inline]
    pub fn remove_by_key(&mut self, key: &TypeKey) -> Option<CompiledDsdl> {
        self.types.remove(key)
    }

    /// Returns an iterator over the types in this package
    ///
    /// The order of iteration is unspecified.
    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.types.iter())
    }

    /// Returns the warnings reported while compiling
    #[inline]
    pub fn warnings(&self) -> &Warnings {
        &self.warnings
    }
}

impl<'p> IntoIterator for &'p CompiledPackage {
    type Item = (&'p TypeKey, &'p CompiledDsdl);
    type IntoIter = Iter<'p>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for CompiledPackage {
    type Item = (TypeKey, CompiledDsdl);
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.types.into_iter())
    }
}

/// An iterator over borrowed types
pub struct Iter<'p>(btree_map::Iter<'p, TypeKey, CompiledDsdl>);

impl<'p> Iterator for Iter<'p> {
    type Item = (&'p TypeKey, &'p CompiledDsdl);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

/// An iterator over types
pub struct IntoIter(btree_map::IntoIter<TypeKey, CompiledDsdl>);

impl Iterator for IntoIter {
    type Item = (TypeKey, CompiledDsdl);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
