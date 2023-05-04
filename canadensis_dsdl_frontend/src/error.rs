use crate::package::TYPE_NAME_LENGTH_MAX;
use crate::type_key::TypeKey;
use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Errors that may occur when compiling
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("Failed to find DSDL files under {}", .root.display())]
    WalkDir {
        root: PathBuf,
        #[source]
        inner: walkdir::Error,
    },
    #[error("Path {} is not a directory", .0.display())]
    NotDirectory(PathBuf),
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
    #[error("Type name or path component {keyword:?} in {key} is a reserved keyword")]
    NameKeyword { keyword: String, key: TypeKey },
    #[error("Type name or path component {component:?} in {key} is not a valid identifier")]
    NameInvalidIdentifier { component: String, key: TypeKey },
    #[error(
        "Type name {name} (from type {key}) is too long. The maximum allowed length is {}",
        TYPE_NAME_LENGTH_MAX
    )]
    TypeNameLength { name: String, key: TypeKey },
    #[error("Version number 0.0 (from {0}) is not allowed")]
    VersionZero(TypeKey),
    #[error("Type {0} is not in a namespace")]
    TypeNotInNamespace(TypeKey),
    #[error("Can't add a type named {old}: another type with a conflicting name {new} has already been added")]
    DuplicateKey { old: TypeKey, new: TypeKey },
    #[error("Non-deprecated type {outer} uses deprecated type {inner}")]
    DeprecatedInNonDeprecated { outer: TypeKey, inner: TypeKey },
    /// An error triggered by a particular file
    ///
    /// Because files are compiled recursively, this may contain any other error type caused by
    /// another file.
    #[error("Error processing type {key} from file {path:?}")]
    CompileFile {
        key: TypeKey,
        path: Option<PathBuf>,
        #[source]
        inner: Box<Error>,
    },
    #[error("Invalid DSDL")]
    Compile(
        #[from]
        #[source]
        canadensis_dsdl_parser::Error,
    ),
    /// A type could not be found
    ///
    /// This sometimes indicates a cyclic dependency between DSDL types if the type not found
    /// is also being compiled further up the call stack.
    #[error("Type {0} not found")]
    UnknownType(TypeKey),
    #[error("Input/output error")]
    Io(#[from] io::Error),
}
