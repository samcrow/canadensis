//!
//! Error definitions
//!

use defmt::Format;
use fallible_collections::TryReserveError;

/// An error indicating that memory could not be allocated
#[derive(Debug, Eq, PartialEq, Clone, Format)]
pub struct OutOfMemoryError;

impl From<TryReserveError> for OutOfMemoryError {
    fn from(_: TryReserveError) -> Self {
        OutOfMemoryError
    }
}

/// An error that may occur when subscribing to a service
#[derive(Debug, Eq, PartialEq, Clone, Format)]
pub enum ServiceSubscribeError<E> {
    /// Can't subscribe to a service because this is an anonymous node
    Anonymous,
    /// The transport returned an error
    Transport(E),
}

impl<E> From<E> for ServiceSubscribeError<E> {
    fn from(inner: E) -> Self {
        ServiceSubscribeError::Transport(inner)
    }
}
