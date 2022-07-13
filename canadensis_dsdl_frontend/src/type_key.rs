use canadensis_dsdl_parser::{TypeVersion, VersionedType};
use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

/// A key that identifies a data type based on its package, name, and version
///
/// The path components and package name should be valid DSDL identifiers.
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

    /// Returns true if this name is empty
    pub fn is_empty(&self) -> bool {
        self.path.is_empty() && self.name.is_empty()
    }

    /// Returns a copy of this name with all components changed to lowercase
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

impl FromStr for TypeKey {
    type Err = ParseError;

    /// Parses a type key from a string
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"^(?P<path_and_name>(?:[^.]+\.)+)(?P<version_major>\d+)\.(?P<version_minor>\d+)$",
            )
            .unwrap()
        });

        match PATTERN.captures(s) {
            Some(captures) => {
                let path_and_name = &captures["path_and_name"];
                // Remove the . from the end, leaving only .-separated segments
                let path_and_name = path_and_name.strip_suffix('.').unwrap();
                let name = path_and_name.parse()?;

                let version = TypeVersion {
                    major: captures["version_major"].parse()?,
                    minor: captures["version_minor"].parse()?,
                };

                Ok(TypeKey::new(name, version))
            }
            None => Err(ParseError::Format),
        }
    }
}

impl FromStr for TypeFullName {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components: Vec<String> = s.split('.').map(String::from).collect();
        if components.len() >= 2 {
            // Have at least one package segment and a name
            let name = components.pop().unwrap();
            Ok(TypeFullName::new(components, name))
        } else {
            Err(ParseError::Format)
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid format")]
    Format,
    #[error("Used reserved keyword {0}")]
    ReservedKeyword(String),
    #[error("Invalid version number")]
    ParseInt(#[from] ParseIntError),
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

    #[test]
    fn parse_key() {
        let expected = TypeKey::new(
            TypeFullName::new(
                vec!["package".into(), "test".into(), "t37".into()],
                "KeyedType".into(),
            ),
            TypeVersion { major: 9, minor: 3 },
        );
        assert_eq!(expected, "package.test.t37.KeyedType.9.3".parse().unwrap());
    }
}
