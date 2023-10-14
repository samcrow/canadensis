use crate::queue::FrameQueue;
use crate::Frame;
use canadensis_core::OutOfMemoryError;

/// An outgoing frame queue that can hold only one frame
pub struct SingleFrameQueue {
    frame: Option<Frame>,
}

impl SingleFrameQueue {
    /// Creates a new empty queue
    pub fn new() -> Self {
        SingleFrameQueue { frame: None }
    }
}

impl Default for SingleFrameQueue {
    fn default() -> Self {
        SingleFrameQueue::new()
    }
}

impl FrameQueue for SingleFrameQueue {
    fn try_reserve(&mut self, additional: usize) -> Result<(), OutOfMemoryError> {
        if self.frame.is_none() && additional == 1 {
            Ok(())
        } else {
            Err(OutOfMemoryError)
        }
    }

    fn shrink_to_fit(&mut self) {
        // Nothing to do
    }

    fn push_frame(&mut self, frame: Frame) -> Result<(), OutOfMemoryError> {
        if self.frame.is_none() {
            self.frame = Some(frame);
            Ok(())
        } else {
            Err(OutOfMemoryError)
        }
    }

    fn peek_frame(&self) -> Option<&Frame> {
        self.frame.as_ref()
    }

    fn pop_frame(&mut self) -> Option<Frame> {
        self.frame.take()
    }

    fn return_frame(&mut self, frame: Frame) -> Result<(), OutOfMemoryError> {
        if self.frame.is_some() {
            Err(OutOfMemoryError)
        } else {
            self.frame = Some(frame);
            Ok(())
        }
    }
}
