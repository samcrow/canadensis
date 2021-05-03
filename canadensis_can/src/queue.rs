//! A queue of CAN frames

use crate::heap::Heap;
use crate::FrameById;

/// A queue of CAN frames ordered by ID
///
/// Frames in the queue are ordered by ID, with the lowest ID (highest priority) first.
pub struct FrameQueue<I>(Heap<FrameById<I>>);
