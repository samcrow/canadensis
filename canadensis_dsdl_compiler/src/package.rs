use crate::compiled::CompiledDsdl;
use crate::error::Error;
use crate::type_key::{TypeFullName, TypeKey};
use crate::types::keywords::{is_reserved_keyword, is_valid_identifier};
use canadensis_dsdl_parser::TypeVersion;
use regex::Regex;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

/// The maximum length of a type name in characters, including the package name and .s but not
/// including the version
pub const TYPE_NAME_LENGTH_MAX: usize = 255;

/// A package of zero or more data structure definitions read from files
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
    pub fn add_files<P>(&mut self, root: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let root = root.as_ref();
        for entry in WalkDir::new(root) {
            let entry = entry.map_err(walk_dir_error(root))?;
            if is_dsdl(&entry) {
                self.add_file(root, entry)?;
            }
        }
        Ok(())
    }

    fn add_file(&mut self, root: &Path, file: DirEntry) -> Result<(), Error> {
        let (key, name_info) = info_from_path(root, file.path())?;

        let dsdl = DsdlFile {
            path: file.path().into(),
            fixed_port_id: name_info.port_id,
        };

        // Check for a duplicate type name
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
    pub fn compile(self) -> Result<CompiledPackage, Error> {
        let result = crate::compile::compile(self.files)?;
        Ok(CompiledPackage { types: result })
    }
}

#[derive(Debug)]
pub struct CompiledPackage {
    types: BTreeMap<TypeKey, CompiledDsdl>,
}

/// Returns true if the provided entry is a DSDL file
fn is_dsdl(entry: &DirEntry) -> bool {
    if entry.file_type().is_file() {
        if let Some(extension) = entry.path().extension() {
            extension == "uavcan"
        } else {
            false
        }
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
pub struct DsdlFile {
    path: PathBuf,
    fixed_port_id: Option<u32>,
}

impl DsdlFile {
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn fixed_port_id(&self) -> Option<u32> {
        self.fixed_port_id
    }
}

/// Parses a type key from a file path
///
/// This function also checks that the package and name are valid non-reserved identifiers, and that
/// the combined package and name are not too long.
///
/// # Panics
///
/// This function may panic if root is not an ancestor of file_path
fn info_from_path(root: &Path, file_path: &Path) -> Result<(TypeKey, FileNameInfo), Error> {
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
    if path_components.len() < 2 {
        return Err(Error::FileNotInPackage(file_path.into()));
    }

    let file_name = path_components.pop().expect("No file name");
    let name_info = parse_file_name(&file_name).ok_or_else(|| Error::FileName(file_path.into()))?;

    // Check path and name for reserved keywords
    for component in path_components.iter() {
        if !is_valid_identifier(&*component) {
            return Err(Error::NameInvalidIdentifier {
                path: file_path.into(),
                component: component.to_owned(),
            });
        }
        if is_reserved_keyword(&*component) {
            return Err(Error::NameKeyword {
                path: file_path.into(),
                keyword: component.to_owned(),
            });
        }
    }
    if !is_valid_identifier(&name_info.name) {
        return Err(Error::NameInvalidIdentifier {
            path: file_path.into(),
            component: name_info.name,
        });
    }
    if is_reserved_keyword(&name_info.name) {
        return Err(Error::NameKeyword {
            path: file_path.into(),
            keyword: name_info.name,
        });
    }

    // Check version
    if name_info.version.major == 0 && name_info.version.minor == 0 {
        return Err(Error::VersionZero(file_path.into()));
    }

    let full_name = TypeFullName::new(path_components, name_info.name.clone());

    if full_name.len() > TYPE_NAME_LENGTH_MAX {
        return Err(Error::TypeNameLength {
            path: file_path.into(),
            name: full_name.to_string(),
        });
    }
    Ok((
        TypeKey::new(full_name, name_info.version.clone()),
        name_info,
    ))
}

/// Information about a type from its file name
#[derive(Debug, PartialOrd, PartialEq)]
struct FileNameInfo {
    /// Fixed port ID, if any
    port_id: Option<u32>,
    /// Type name
    name: String,
    /// Type version
    version: TypeVersion,
}

/// Extracts information about a data type from its file name and returns it
fn parse_file_name(name: &str) -> Option<FileNameInfo> {
    let pattern = Regex::new(
        r"^((?P<port_id>\d+)\.)?(?P<short_name>[^.]+)\.(?P<version_major>\d+)\.(?P<version_minor>\d+)\.uavcan$",
    )
    .unwrap();
    if let Some(captures) = pattern.captures(name) {
        let port_id = match captures.name("port_id") {
            Some(id_str) => Some(id_str.as_str().parse::<u32>().ok()?),
            None => None,
        };
        let name = captures["short_name"].to_owned();
        let version_major: u8 = captures["version_major"].parse().ok()?;
        let version_minor: u8 = captures["version_minor"].parse().ok()?;

        Some(FileNameInfo {
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
    use super::{parse_file_name, FileNameInfo};
    use canadensis_dsdl_parser::TypeVersion;

    #[test]
    fn test_parse_file_name() {
        assert_eq!(
            Some(FileNameInfo {
                port_id: Some(970),
                name: "DataType1".to_string(),
                version: TypeVersion { major: 3, minor: 7 }
            }),
            parse_file_name("970.DataType1.3.7.uavcan")
        );
        assert_eq!(
            Some(FileNameInfo {
                port_id: None,
                name: "DataType1".to_string(),
                version: TypeVersion { major: 3, minor: 7 }
            }),
            parse_file_name("DataType1.3.7.uavcan")
        );
    }
}
