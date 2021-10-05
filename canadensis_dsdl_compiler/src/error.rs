use crate::package::TYPE_NAME_LENGTH_MAX;
use crate::type_key::TypeKey;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Failed to find DSDL files under {}", .root.display())]
    WalkDir {
        root: PathBuf,
        #[source]
        inner: walkdir::Error,
    },
    #[error("Failed to read DSDL file {}", .path.display())]
    FileRead {
        path: PathBuf,
        #[source]
        inner: io::Error,
    },
    #[error("Path {0} could not be converted into UTF-8")]
    PathUtf8(PathBuf),
    #[error("File {0} has an invalid name")]
    FileName(PathBuf),
    #[error("Type name or path to {path} uses reserved keyword {keyword}")]
    NameKeyword { path: PathBuf, keyword: String },
    #[error("Type name or path component {component:?} in {} is not a valid identifier")]
    NameInvalidIdentifier { path: PathBuf, component: String },
    #[error(
        "Type name {name} (from {path}) is too long. The maximum allowed length is {}",
        TYPE_NAME_LENGTH_MAX
    )]
    TypeNameLength { path: PathBuf, name: String },
    #[error("Version number 0.0 (from {0}) is not allowed")]
    VersionZero(PathBuf),
    #[error("DSDL file {} is located in the root directory. It must be in a namespace subdirectory.", .0.display())]
    FileNotInPackage(PathBuf),
    #[error("Can't add a type named {old}: another type with a conflicting name {new} has already been added")]
    DuplicateKey { old: TypeKey, new: TypeKey },
    /// An error triggered by a particular file
    ///
    /// Because files are compiled recursively, this may contain any other error type caused by
    /// another file.
    #[error("Error processing file {}", .path.display())]
    CompileFile {
        path: PathBuf,
        #[source]
        inner: Box<Error>,
    },
    #[error("{0}")]
    Compile(#[from] canadensis_dsdl_parser::Error),
    #[error("Type {0} not found")]
    UnknownType(TypeKey),
    #[error("Input/output error")]
    Io(#[from] io::Error),
}
