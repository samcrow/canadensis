//! Utilities for use with redundant transports

mod deduplicator;
pub use self::deduplicator::Deduplicator;
mod redundant_queue;
pub use self::redundant_queue::RedundantQueue;
