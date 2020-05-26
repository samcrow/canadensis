//!
//! Error definitions
//!

use fallible_collections::TryReserveError;

/// An error indicating that memory could not be allocated
#[derive(Debug, Eq, PartialEq)]
pub struct OutOfMemoryError(pub(crate) ());

impl From<TryReserveError> for OutOfMemoryError {
    fn from(_: TryReserveError) -> Self {
        OutOfMemoryError(())
    }
}
