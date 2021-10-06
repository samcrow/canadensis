use canadensis_dsdl_parser::{TypeVersion, VersionedType};
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};

/// A key that identifies a data type
///
/// The `PartialOrd`, `Ord`, `PartialEq`, and `Eq` implementations for this type are based on the
/// lowercase versions of the path and name. This means that type keys that differ only in case
/// are considered equal.
#[derive(Debug, Clone)]
pub struct TypeKey {
    /// Type path and name, like `uavcan.node.Heartbeat`
    ///
    /// This is not used for comparison.
    name: TypeFullName,
    /// Type path and name, but in lowercase
    lowercase_name: TypeFullName,
    /// Type version, like `1.0`
    version: TypeVersion,
}

impl PartialEq for TypeKey {
    fn eq(&self, other: &Self) -> bool {
        // Compare only the lowercase name and fields
        PartialEq::eq(
            &(&self.lowercase_name, &self.version),
            &(&other.lowercase_name, &other.version),
        )
    }
}
impl Eq for TypeKey {}

impl Ord for TypeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare only the lowercase name and fields
        Ord::cmp(
            &(&self.lowercase_name, &self.version),
            &(&other.lowercase_name, &other.version),
        )
    }
}

impl PartialOrd for TypeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl TypeKey {
    pub fn new(name: TypeFullName, version: TypeVersion) -> Self {
        let lowercase_name = name.to_lowercase();
        TypeKey {
            name,
            lowercase_name,
            version,
        }
    }

    /// Returns true if this key and another are exactly equal, including the cases of letters
    pub fn case_sensitive_equal(&self, other: &TypeKey) -> bool {
        (&self.name, &self.version) == (&other.name, &other.version)
    }
    /// Returns the type name
    pub fn name(&self) -> &TypeFullName {
        &self.name
    }
    /// Returns the type version
    pub fn version(&self) -> &TypeVersion {
        &self.version
    }
}

/// The path and name of a type, but not its version
#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq)]
pub struct TypeFullName {
    /// Path segments leading to the type, like `[uavcan, node]`
    path: Vec<String>,
    /// Type name, like `Heartbeat`
    name: String,
}

impl TypeFullName {
    pub fn new(path: Vec<String>, name: String) -> Self {
        TypeFullName { path, name }
    }

    /// Returns the number of characters needed to display the path and name of this type,
    /// but not the version
    pub fn len(&self) -> usize {
        // Each path segment takes up its own length plus one dot
        // Because paths and names are all ASCII, we can use byte lengths.
        self.path
            .iter()
            .map(|segment| segment.len() + 1)
            .sum::<usize>()
            + self.name.len()
    }

    fn to_lowercase(&self) -> TypeFullName {
        TypeFullName {
            path: self
                .path
                .iter()
                .map(|component| component.to_lowercase())
                .collect(),
            name: self.name.to_lowercase(),
        }
    }

    /// Returns the path to the package containing the type
    pub fn path(&self) -> &[String] {
        &self.path
    }
    /// Returns the type name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<VersionedType<'_>> for TypeKey {
    fn from(versioned_type: VersionedType<'_>) -> Self {
        TypeKey::new(
            TypeFullName {
                path: versioned_type.path.into_iter().map(String::from).collect(),
                name: versioned_type.name.to_owned(),
            },
            versioned_type.version,
        )
    }
}

mod fmt_impl {
    use super::{TypeFullName, TypeKey};
    use std::fmt::{Display, Formatter, Result};
    impl Display for TypeKey {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "{}.{}", self.name, self.version)
        }
    }

    impl Display for TypeFullName {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            for component in &self.path {
                write!(f, "{}.", component)?;
            }
            write!(f, "{}", self.name)
        }
    }
}

#[cfg(test)]
mod test {
    use super::{TypeFullName, TypeKey};
    use canadensis_dsdl_parser::TypeVersion;

    #[test]
    fn case_insensitive() {
        assert_eq!(
            TypeKey::new(
                TypeFullName::new(
                    vec!["uavcan".to_owned(), "node".to_owned()],
                    "GetInfo".to_owned()
                ),
                TypeVersion { major: 1, minor: 0 }
            ),
            TypeKey::new(
                TypeFullName::new(
                    vec!["uavcan".to_owned(), "node".to_owned()],
                    "getInfo".to_owned()
                ),
                TypeVersion { major: 1, minor: 0 }
            )
        );
    }
}
