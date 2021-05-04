//!
//! Error definitions
//!

use fallible_collections::TryReserveError;

/// An error indicating that memory could not be allocated
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct OutOfMemoryError;

impl From<TryReserveError> for OutOfMemoryError {
    fn from(_: TryReserveError) -> Self {
        OutOfMemoryError
    }
}
