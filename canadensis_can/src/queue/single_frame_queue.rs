use crate::queue::FrameQueue;
use crate::Frame;
use canadensis_core::OutOfMemoryError;

/// An outgoing frame queue that can hold only one frame
pub struct SingleFrameQueue<I> {
    frame: Option<Frame<I>>,
}

impl<I> SingleFrameQueue<I> {
    /// Creates a new empty queue
    pub fn new() -> Self {
        SingleFrameQueue { frame: None }
    }
}

impl<I> Default for SingleFrameQueue<I> {
    fn default() -> Self {
        SingleFrameQueue::new()
    }
}

impl<I> FrameQueue<I> for SingleFrameQueue<I> {
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

    fn push_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        if self.frame.is_none() {
            self.frame = Some(frame);
            Ok(())
        } else {
            Err(OutOfMemoryError)
        }
    }

    fn peek_frame(&self) -> Option<&Frame<I>> {
        self.frame.as_ref()
    }

    fn pop_frame(&mut self) -> Option<Frame<I>> {
        self.frame.take()
    }

    fn return_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        if self.frame.is_some() {
            Err(OutOfMemoryError)
        } else {
            self.frame = Some(frame);
            Ok(())
        }
    }
}
