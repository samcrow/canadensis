use std::collections::BTreeSet;

use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::compiled::{CompiledDsdl, DsdlKind, Message, MessageKind};
use crate::TypeKey;

/// A non-fatal warning encountered while processing DSDL
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Warning(WarningKind);

impl std::fmt::Display for Warning {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum WarningKind {
    /// Part of a package name is not in all-lowercase with _ separators
    PackageCase {
        /// Actual package name or name fragment
        name: String,
        /// Suggested alternative name
        suggestion: String,
    },
    /// A field name in a type is not in all-lowercase with _ separators
    FieldCase {
        /// Type that contains the field
        key: TypeKey,
        /// Actual field name
        name: String,
        /// Suggested alternative name
        suggestion: String,
    },
    /// A constant name in a type is not in all-uppercase with _ separators
    ConstantCase {
        /// Type that contains the constant
        key: TypeKey,
        /// Actual constant name
        name: String,
        /// Suggested alternative name
        suggestion: String,
    },
    /// A type name is not in upper camel case
    TypeNameCase {
        /// Type with the unconventional name
        key: TypeKey,
        /// Suggested alternative name
        suggestion: String,
    },
}

impl std::fmt::Display for WarningKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WarningKind::PackageCase {
                name,
                suggestion: alternative,
            } => {
                write!(
                    f,
                    "Package \"{}\" should have a snake_case name like \"{}\"",
                    name, alternative
                )
            }
            WarningKind::FieldCase {
                key: ty,
                name,
                suggestion: alternative,
            } => {
                write!(
                    f,
                    "In type {}, the field or variant \"{}\" should have a snake_case name like \"{}\"",
                    ty, name, alternative
                )
            }
            WarningKind::ConstantCase {
                key: ty,
                name,
                suggestion: alternative,
            } => {
                write!(f, "In type {}, the constant \"{}\" should have a SHOUTING_SNAKE_CASE name like \"{}\"", ty, name, alternative)
            }
            WarningKind::TypeNameCase {
                key: ty,
                suggestion: alternative,
            } => {
                write!(
                    f,
                    "The type {} should have a CamelCase name like \"{}\"",
                    ty, alternative
                )
            }
        }
    }
}

/// A collection of warnings
#[derive(Debug, Clone)]
pub struct Warnings {
    warnings: BTreeSet<Warning>,
}

impl Warnings {
    /// Creates an empty collection of warnings
    pub(crate) fn new() -> Self {
        Warnings {
            warnings: BTreeSet::new(),
        }
    }

    /// Returns true if this collection of warnings is empty
    pub fn is_empty(&self) -> bool {
        self.warnings.is_empty()
    }

    /// Inserts a warning, or does nothing if this set of warnings already contains the provided
    /// warning
    fn insert(&mut self, kind: WarningKind) {
        self.warnings.insert(Warning(kind));
    }

    /// Checks for warnings on a type before it is compiled and adds any warnings to this collection
    pub(crate) fn check_pre_compile(&mut self, key: &TypeKey) {
        // All package segments should be snake_case
        for part in key.name().path() {
            if !is_permissive_snake_case(part) {
                let suggestion = part.to_snake_case();
                self.insert(WarningKind::PackageCase {
                    name: part.to_owned(),
                    suggestion: suggestion,
                })
            }
        }
        // Type name should be UpperCamelCase
        let actual_name = key.name().name();
        if !is_permissive_upper_camel_case(actual_name) {
            let suggestion = actual_name.to_upper_camel_case();
            self.insert(WarningKind::TypeNameCase {
                key: key.clone(),
                suggestion: suggestion,
            })
        }
    }

    /// Checks for warnings on a compiled type and adds any warnings to this collection
    pub(crate) fn check_post_compile(&mut self, key: &TypeKey, dsdl: &CompiledDsdl) {
        match &dsdl.kind {
            DsdlKind::Message(message) => self.check_message(key, message),
            DsdlKind::Service { request, response } => {
                self.check_message(key, request);
                self.check_message(key, response);
            }
        }
    }

    fn check_message(&mut self, key: &TypeKey, message: &Message) {
        // Fields should be snake_case
        match &message.kind {
            MessageKind::Struct(struct_data) => {
                for field in &struct_data.fields {
                    if let Some(name) = field.name() {
                        self.check_field_name(key, name);
                    }
                }
            }
            MessageKind::Union(union_data) => {
                for variant in &union_data.variants {
                    self.check_field_name(key, &variant.name);
                }
            }
        }
        // Constants should be SCREAMING_SNAKE_CASE
        for (name, _) in &message.constants {
            let suggestion = name.to_shouty_snake_case();
            if name != &suggestion {
                self.insert(WarningKind::ConstantCase {
                    key: key.to_owned(),
                    name: name.to_owned(),
                    suggestion: suggestion,
                })
            }
        }
    }

    fn check_field_name(&mut self, key: &TypeKey, name: &str) {
        // Should contain _, a-z, or 0-9
        let snake_case = is_permissive_snake_case(name);
        if !snake_case {
            let suggested = name.to_snake_case();
            self.insert(WarningKind::FieldCase {
                key: key.to_owned(),
                name: name.to_owned(),
                suggestion: suggested,
            })
        }
    }

    /// Returns a borrowed iterator over the warnings
    pub fn iter(&self) -> Iter<'_> {
        Iter(self.warnings.iter())
    }
}

fn is_permissive_snake_case(s: &str) -> bool {
    // Should contain _, a-z, or 0-9
    s.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

fn is_permissive_upper_camel_case(s: &str) -> bool {
    // All characters should be alphanumeric, but the first and last can also be _
    static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new("^_?[a-zA-Z0-9]*_?$").unwrap());
    PATTERN.is_match(s)
}

/// An iterator over warnings in a collection
#[derive(Debug, Clone)]
pub struct Iter<'a>(std::collections::btree_set::Iter<'a, Warning>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Warning;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for Iter<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

/// An owned iterator over warnings in a collection
#[derive(Debug)]
pub struct IntoIter(std::collections::btree_set::IntoIter<Warning>);

impl Iterator for IntoIter {
    type Item = Warning;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

impl ExactSizeIterator for IntoIter {
    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl IntoIterator for Warnings {
    type Item = Warning;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.warnings.into_iter())
    }
}

impl<'a> IntoIterator for &'a Warnings {
    type Item = &'a Warning;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod test {
    use super::is_permissive_upper_camel_case;

    #[test]
    fn test_upper_camel_case() {
        assert!(is_permissive_upper_camel_case(""));
        assert!(is_permissive_upper_camel_case("_"));
        assert!(is_permissive_upper_camel_case("_Struct_"));
        assert!(is_permissive_upper_camel_case("Struct_"));
    }
}
