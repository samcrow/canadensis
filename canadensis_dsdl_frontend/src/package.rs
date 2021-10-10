use crate::compiled::package::CompiledPackage;
use crate::error::Error;
use crate::type_key::{TypeFullName, TypeKey};
use crate::types::keywords::{is_reserved_keyword, is_valid_identifier};
use canadensis_dsdl_parser::TypeVersion;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::iter;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// The maximum length of a type name in characters, including the package name and .s but not
/// including the version
pub(crate) const TYPE_NAME_LENGTH_MAX: usize = 255;

/// A package of zero or more data structure definitions
///
/// Data structure definitions can be read from directories or provided as strings.
#[derive(Debug)]
pub struct Package {
    /// All known files that have been scanned but not parsed
    ///
    /// Because TypeKey compares using lowercase forms of the path and name, this does not allow
    /// multiple definitions with names that differ only in case.
    files: BTreeMap<TypeKey, DsdlFile>,
}

impl Package {
    pub fn new() -> Self {
        Package {
            files: BTreeMap::new(),
        }
    }
    /// Scans for DSDL files in the provided root directory and adds them to this package
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// * The path does not exist or does not point to a directory
    /// * An I/O error occurs when scanning for files
    /// * A file name or path segment is not valid UTF-8
    /// * A file name does not have the correct format
    /// * The combined path and name of a data type are too long
    /// * A data type version is 0.0
    /// * A file represents a data type that is already in this package
    ///
    pub fn add_files<P>(&mut self, root: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let root = root.as_ref();
        for entry in WalkDir::new(root) {
            let entry = entry.map_err(walk_dir_error(root))?;
            if is_dsdl(&entry) {
                self.add_file_from_path(root, entry)?;
            }
        }
        Ok(())
    }

    /// Adds one DSDL file to this package
    ///
    /// This function ignores the path and name of the file. Instead, the fixed port ID and type key
    /// are provided as parameters to this function.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// * The type key does not have a package
    /// * Any part of the namespace or name is not a valid identifier, or is a reserved keyword
    /// * The path and name (combined) are too long
    /// * A type for this key already exists in this package
    /// * The version number is 0.0
    ///
    pub fn add_file<P>(
        &mut self,
        fixed_port_id: Option<u32>,
        key: TypeKey,
        path: P,
    ) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        self.try_insert(
            key,
            DsdlFile {
                source: FileSource::File(path.as_ref().into()),
                fixed_port_id,
            },
        )
    }

    /// Adds a string containing the content of one DSDL file to this package
    ///
    /// The fixed port ID and key are passed as parameters to this function.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// * The type key does not have a package
    /// * Any part of the namespace or name is not a valid identifier, or is a reserved keyword
    /// * The path and name (combined) are too long
    /// * A type for this key already exists in this package
    /// * The version number is 0.0
    ///
    pub fn add_string(
        &mut self,
        fixed_port_id: Option<u32>,
        key: TypeKey,
        dsdl: String,
    ) -> Result<(), Error> {
        self.try_insert(
            key,
            DsdlFile {
                source: FileSource::String(dsdl),
                fixed_port_id,
            },
        )
    }

    fn add_file_from_path(&mut self, root: &Path, file: DirEntry) -> Result<(), Error> {
        let (key, fixed_port_id) = info_from_path(root, file.path())?;

        let dsdl = DsdlFile {
            source: FileSource::File(file.path().into()),
            fixed_port_id,
        };
        self.try_insert(key, dsdl)
    }

    /// Inserts a DSDL file, or returns an error if a type with the same key is already present,
    /// the type key does not have a package, any part of the type key is a reserved keyword,
    /// the namespace and name are too long, or the version is 0.0
    fn try_insert(&mut self, key: TypeKey, dsdl: DsdlFile) -> Result<(), Error> {
        validate_full_key(&key)?;
        match self.files.entry(key.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(dsdl);
                Ok(())
            }
            Entry::Occupied(entry) => Err(Error::DuplicateKey {
                old: entry.key().clone(),
                new: key,
            }),
        }
    }

    /// Compiles all input files that were previously added
    ///
    /// # Errors
    ///
    /// This function returns an error if any DSDL file could not be read, if any DSDL file has
    /// invalid content, or an `@assert` directive fails.
    pub fn compile(self) -> Result<CompiledPackage, Error> {
        let result = crate::compile::compile(self.files)?;
        Ok(CompiledPackage::new(result))
    }
}

/// Checks that the provided key has a package and does not contain any reserved keywords,
/// the path and name (not including the version) is not too long, and the version is not 0.0
fn validate_full_key(key: &TypeKey) -> Result<(), Error> {
    if key.name().path().is_empty() {
        return Err(Error::TypeNotInNamespace(key.clone()));
    }
    if key.name().len() > TYPE_NAME_LENGTH_MAX {
        return Err(Error::TypeNameLength {
            name: key.name().to_string(),
            key: key.clone(),
        });
    }
    for segment in key
        .name()
        .path()
        .iter()
        .map(Deref::deref)
        .chain(iter::once(key.name().name()))
    {
        if !is_valid_identifier(segment) {
            return Err(Error::NameInvalidIdentifier {
                component: segment.into(),
                key: key.clone(),
            });
        }
        if is_reserved_keyword(segment) {
            return Err(Error::NameKeyword {
                keyword: segment.into(),
                key: key.clone(),
            });
        }
    }
    let version = key.version();
    if version.major == 0 && version.minor == 0 {
        return Err(Error::VersionZero(key.clone()));
    }
    Ok(())
}

/// Returns true if the provided entry is a DSDL file
fn is_dsdl(entry: &DirEntry) -> bool {
    if entry.file_type().is_file() {
        entry
            .path()
            .extension()
            .map_or(false, |extension| extension == "uavcan")
    } else {
        false
    }
}

/// Returns a function that converts a walkdir error to an Error associated with the provided path
fn walk_dir_error(root: &Path) -> impl Fn(walkdir::Error) -> Error + '_ {
    move |e| Error::WalkDir {
        root: root.into(),
        inner: e,
    }
}

impl Default for Package {
    fn default() -> Self {
        Package::new()
    }
}

/// A DSDL file that has not yet been parsed
#[derive(Debug)]
pub(crate) struct DsdlFile {
    source: FileSource,
    fixed_port_id: Option<u32>,
}

#[derive(Debug)]
enum FileSource {
    File(PathBuf),
    String(String),
}

impl DsdlFile {
    /// Reads a DSDL file and returns its content
    pub(crate) fn read(&self) -> Result<String, Error> {
        match &self.source {
            FileSource::File(path) => std::fs::read_to_string(path).map_err(|e| Error::FileRead {
                path: path.clone(),
                inner: e,
            }),
            FileSource::String(content) => Ok(content.clone()),
        }
    }
    /// Returns the fixed port ID for this type, if it has one
    pub(crate) fn fixed_port_id(&self) -> Option<u32> {
        self.fixed_port_id
    }
    /// Returns the path to this file, or None if this DSDL is not from a file
    pub(crate) fn path(&self) -> Option<&Path> {
        match &self.source {
            FileSource::File(path) => Some(&path),
            FileSource::String(_) => None,
        }
    }
}

/// Parses a type key from a file path
///
/// This function also checks that the package and name are valid non-reserved identifiers, and that
/// the combined package and name are not too long.
///
/// # Panics
///
/// This function may panic if root is not an ancestor of `file_path`.
fn info_from_path(root: &Path, file_path: &Path) -> Result<(TypeKey, Option<u32>), Error> {
    let mut file_components = file_path.iter();
    // Consume all components of the file path that are the same as the root
    for (root_component, file_component) in root.iter().zip(file_components.by_ref()) {
        assert_eq!(
            root_component, file_component,
            "Root and beginning of file path do not match"
        );
    }
    // Now the remaining values in file_components are the path and file name
    let mut path_components: Vec<String> = file_components
        .map(|component| {
            component
                .to_str()
                .ok_or_else(|| Error::PathUtf8(file_path.into()))
                .map(String::from)
        })
        .collect::<Result<_, _>>()?;

    let file_name = path_components.pop().expect("No file name");
    let name_info = parse_file_name(&file_name).ok_or_else(|| Error::FileName(file_path.into()))?;
    let full_name = TypeFullName::new(path_components, name_info.name);

    Ok((
        TypeKey::new(full_name, name_info.version),
        name_info.port_id,
    ))
}

/// Information about a type, potentially from its file name
#[derive(Debug, PartialOrd, PartialEq)]
struct FileInfo {
    /// Fixed port ID, if any
    port_id: Option<u32>,
    /// Type name
    name: String,
    /// Type version
    version: TypeVersion,
}

/// Extracts information about a data type from its file name and returns it
fn parse_file_name(name: &str) -> Option<FileInfo> {
    static PATTERN: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^((?P<port_id>\d+)\.)?(?P<short_name>[^.]+)\.(?P<version_major>\d+)\.(?P<version_minor>\d+)\.uavcan$")
        .unwrap()
    });

    if let Some(captures) = PATTERN.captures(name) {
        let port_id = match captures.name("port_id") {
            Some(id_str) => Some(id_str.as_str().parse::<u32>().ok()?),
            None => None,
        };
        let name = captures["short_name"].to_owned();
        let version_major: u8 = captures["version_major"].parse().ok()?;
        let version_minor: u8 = captures["version_minor"].parse().ok()?;

        Some(FileInfo {
            port_id,
            name,
            version: TypeVersion {
                major: version_major,
                minor: version_minor,
            },
        })
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::{parse_file_name, FileInfo};
    use canadensis_dsdl_parser::TypeVersion;

    #[test]
    fn test_parse_file_name() {
        assert_eq!(
            Some(FileInfo {
                port_id: Some(970),
                name: "DataType1".to_string(),
                version: TypeVersion { major: 3, minor: 7 }
            }),
            parse_file_name("970.DataType1.3.7.uavcan")
        );
        assert_eq!(
            Some(FileInfo {
                port_id: None,
                name: "DataType1".to_string(),
                version: TypeVersion { major: 3, minor: 7 }
            }),
            parse_file_name("DataType1.3.7.uavcan")
        );
    }
}
