use crate::Frame;
use canadensis_core::time::Instant;

/// Deduplicates frames
///
/// Type parameters:
/// * `I`: The Instant type used for timing
/// * `N`: The number of transports to handle (must not be 0)
///
/// For more explanation, see [the comments in pyuavcan](https://github.com/UAVCAN/pyuavcan/blob/87c27a978119d24ac77c9a7f2d6f289846ac96fd/pyuavcan/transport/redundant/__init__.py).
///
#[derive(Debug)]
pub struct Deduplicator<I: Instant, const N: usize> {
    /// The state for each transport
    states: [TransportState<I>; N],
    /// The index of the transport that is currently active
    active_index: usize,
    /// The time to wait to switch to another transport if the active transport has not received
    /// any frames
    timeout: I::Duration,
}

impl<I, const N: usize> Deduplicator<I, N>
where
    I: Instant,
{
    /// Creates a deduplicator
    ///
    /// The deduplicator will initially use transport 0.
    ///
    /// # Panics
    ///
    /// This function panics if `N` is zero.
    pub fn new(timeout: I::Duration) -> Self {
        assert_ne!(N, 0, "Can't deduplicate from zero transports");

        Deduplicator {
            states: [TransportState::default(); N],
            active_index: 0,
            timeout,
        }
    }

    /// Determines if the provided frame should be accepted and passed to the receiver
    ///
    /// index is the index of the transport where this frame was received
    ///
    /// This function returns true if the frame should be processed, or false if it should be
    /// discarded.
    pub fn accept(&mut self, frame: &Frame<I>, index: usize) -> bool {
        // Update frame time
        self.states[index].last_frame_time = Some(frame.timestamp());

        if self.active_transport_timed_out(frame.timestamp()) {
            // The transport that received this frame is definitely doing something. Switch to it.
            self.active_index = index;
        }

        // Accept frames from the active transport only
        index == self.active_index
    }

    /// Returns true if the currently active transfer has not received a frame within the last
    /// self.timeout duration
    fn active_transport_timed_out(&self, now: I) -> bool {
        let active_last_frame = &self.states[self.active_index].last_frame_time;
        match active_last_frame {
            Some(active_last_frame) => now.duration_since(active_last_frame) > self.timeout,
            None => {
                // The active transport has never received a frame, so it's timed out.
                true
            }
        }
    }
}

/// Information about a transport
#[derive(Debug, Copy, Clone)]
struct TransportState<I> {
    last_frame_time: Option<I>,
}

impl<I> Default for TransportState<I> {
    fn default() -> Self {
        TransportState {
            last_frame_time: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::redundant::Deduplicator;
    use crate::{CanId, Frame};
    use canadensis_core::time::{milliseconds, Microseconds32};

    type TestInstant = Microseconds32;

    fn make_frame(microseconds: u32) -> Frame<TestInstant> {
        Frame::new(TestInstant::new(microseconds), CanId::default(), &[])
    }

    #[test]
    #[should_panic]
    fn zero_transports() {
        Deduplicator::<TestInstant, 0>::new(milliseconds(1));
    }

    #[test]
    fn one_transport() {
        let mut deduplicator = Deduplicator::<TestInstant, 1>::new(milliseconds(1));
        assert!(deduplicator.accept(&make_frame(0), 0));
        assert!(deduplicator.accept(&make_frame(1), 0));
        assert!(deduplicator.accept(&make_frame(2), 0));
        // Wait long enough for the timeout to expire
        assert!(deduplicator.accept(&make_frame(2000), 0));
        assert!(deduplicator.accept(&make_frame(2001), 0));
    }

    #[test]
    fn two_transports() {
        let mut deduplicator = Deduplicator::<TestInstant, 2>::new(milliseconds(1));
        // The transport with the first frame becomes active
        assert!(deduplicator.accept(&make_frame(0), 1));
        assert!(!deduplicator.accept(&make_frame(3), 0));
        assert!(deduplicator.accept(&make_frame(5), 1));
        // All frames from transport 0 are discarded...
        assert!(!deduplicator.accept(&make_frame(6), 0));
        assert!(!deduplicator.accept(&make_frame(900), 0));
        assert!(!deduplicator.accept(&make_frame(1005), 0));
        // .. until the timeout since the last frame on transport 1 has passed
        assert!(deduplicator.accept(&make_frame(1006), 0));
        assert!(!deduplicator.accept(&make_frame(1006), 1));
    }
}
