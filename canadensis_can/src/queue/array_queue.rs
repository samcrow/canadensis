use crate::queue::{FrameQueueSource, FrameSink};
use crate::{Frame, OutOfMemoryError};
use core::mem::{self, MaybeUninit};
use core::ptr;

/// A frame queue implemented as a ring buffer in a fixed-capacity array
///
/// `N` is the maximum number of frames that the queue can hold. This should be at least as large
/// as the number of frames required for the largest outgoing transfer that will be sent.
#[derive(Debug)]
pub struct ArrayQueue<I, const N: usize> {
    /// The frames in this queue
    ///
    /// The head of the queue is at index `self.head`. Other items are at increasing indices,
    /// potentially wrapping around up to and including `self.head - 1`.
    items: [Frame<I>; N],
    /// The index in self.items of the front of the queue
    head: usize,
    /// The number of valid frames in the queue
    length: usize,
}

impl<I, const N: usize> ArrayQueue<I, N>
where
    I: Default,
{
    /// Returns a new emtpy queue
    pub fn new() -> Self {
        // Need to use unsafe code to initialize the array of items.
        let items: [Frame<I>; N] = unsafe {
            let mut items: MaybeUninit<[Frame<I>; N]> = MaybeUninit::uninit();

            for i in 0..N {
                // Get a pointer to the frame in the array
                // Is this really safe? The rules about creating references from pointers are complicated.
                let slot_ptr = (*items.as_mut_ptr()).as_mut_ptr().add(i);
                // Use ptr::write to avoid dropping the uninitialized value
                // If Frame::default() panics and unwinds, the drop of MaybeUninit<[Frame<I>; N]>
                // will do nothing. This may leak memory if the frame owns dynamically allocated
                // memory, but that is not considered unsafe.
                ptr::write(slot_ptr, Frame::default());
            }
            // All frames have been written
            items.assume_init()
        };

        ArrayQueue {
            items,
            head: 0,
            length: 0,
        }
    }

    /// Returns the number of frames in this queue
    pub fn len(&self) -> usize {
        self.length
    }
    /// Returns true if this queue does not contain any frames
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Returns the maximum number of frames that this queue can hold
    pub fn capacity(&self) -> usize {
        N
    }

    fn increment_head(&mut self) {
        if N != 0 {
            self.head = self.head.wrapping_add(1) % N;
        }
    }

    fn decrement_head(&mut self) {
        if N != 0 {
            self.head = self.head.wrapping_add(N - 1) % N;
        }
    }
}

impl<I, const N: usize> FrameSink<I> for ArrayQueue<I, N>
where
    I: Clone + Default,
{
    fn try_reserve(&mut self, additional: usize) -> Result<(), OutOfMemoryError> {
        let free_capacity = N - self.length;
        if free_capacity >= additional {
            Ok(())
        } else {
            // Too full, can't allocate memory
            Err(OutOfMemoryError)
        }
    }

    fn shrink_to_fit(&mut self) {
        // Doesn't dynamically allocate memory, nothing to do
    }

    fn push_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        if self.length == N {
            Err(OutOfMemoryError)
        } else {
            let inserted_frame_id = frame.id();
            // Insert the frame at the back
            let tail_index = self.head.wrapping_add(self.length) % N;
            self.items[tail_index] = frame;
            self.length += 1;

            // Move the frame towards the front (lower index) until the frame in front of it
            // has a lesser or equal CAN ID
            let mut inserted_index = tail_index;
            while inserted_index != self.head {
                let ahead_of_inserted_index = inserted_index.wrapping_add(N - 1) % N;

                if self.items[ahead_of_inserted_index].id() <= inserted_frame_id {
                    break;
                } else {
                    // Swap the frames, moving the inserted frame up
                    self.items.swap(inserted_index, ahead_of_inserted_index);
                }

                // Advance down
                inserted_index = ahead_of_inserted_index;
            }

            Ok(())
        }
    }
}

impl<I, const N: usize> FrameQueueSource<I> for ArrayQueue<I, N>
where
    I: Default,
{
    fn peek_frame(&self) -> Option<&Frame<I>> {
        if self.length != 0 {
            Some(&self.items[self.head])
        } else {
            None
        }
    }

    fn pop_frame(&mut self) -> Option<Frame<I>> {
        if self.length != 0 {
            let frame = mem::take(&mut self.items[self.head]);
            self.increment_head();
            self.length -= 1;
            Some(frame)
        } else {
            None
        }
    }

    fn return_frame(&mut self, frame: Frame<I>) -> Result<(), OutOfMemoryError> {
        if self.length == N {
            Err(OutOfMemoryError)
        } else {
            let inserted_frame_id = frame.id();
            // Insert the frame at the front
            self.decrement_head();
            self.items[self.head] = frame;
            self.length += 1;

            // Move the frame towards the back (higher index) until the frame behind it
            // has a greater or equal CAN ID
            let mut inserted_index = self.head;
            let tail = self.head.wrapping_add(self.length - 1) % N;
            while inserted_index != tail {
                let behind_inserted_index = inserted_index.wrapping_add(1) % N;

                if self.items[behind_inserted_index].id() >= inserted_frame_id {
                    break;
                } else {
                    // Swap the frames, moving the inserted frame up
                    self.items.swap(inserted_index, behind_inserted_index);
                }

                // Advance up
                inserted_index = behind_inserted_index;
            }

            Ok(())
        }
    }
}

impl<I, const N: usize> Default for ArrayQueue<I, N>
where
    I: Default,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::ArrayQueue;
    use super::FrameSink;
    use crate::queue::FrameQueueSource;
    use crate::{CanId, Frame};
    use core::convert::TryFrom;

    fn frame_with_id(id: u32, data: u8) -> Frame<()> {
        let id = CanId::try_from(id).unwrap();
        Frame::new((), id, &[data])
    }

    #[test]
    fn basic_insert_same_id() {
        let mut queue = ArrayQueue::<(), 4>::new();
        {
            let frame = frame_with_id(1, 0);
            queue.push_frame(frame.clone()).unwrap();
            assert_eq!(queue.len(), 1);
            assert_eq!(queue.head, 0);
            assert_eq!(queue.items[0], frame);
        }
        {
            // Add a second frame with the same ID, which should end up behind the first
            let frame = frame_with_id(1, 1);
            queue.push_frame(frame.clone()).unwrap();
            assert_eq!(queue.len(), 2);
            assert_eq!(queue.head, 0);
            assert_eq!(queue.items[0], frame_with_id(1, 0));
            assert_eq!(queue.items[1], frame_with_id(1, 1));
        }
    }

    #[test]
    fn basic_insert_different_ids() {
        let mut queue = ArrayQueue::<(), 4>::new();
        queue.push_frame(frame_with_id(10, 0)).unwrap();
        queue.push_frame(frame_with_id(10, 1)).unwrap();
        queue.push_frame(frame_with_id(10, 2)).unwrap();
        // This frame should end up in the front because its ID is lower
        queue.push_frame(frame_with_id(9, 0)).unwrap();

        assert_eq!(queue.len(), 4);
        assert_eq!(queue.head, 0);
        assert_eq!(queue.items[0], frame_with_id(9, 0));
        assert_eq!(queue.items[1], frame_with_id(10, 0));
        assert_eq!(queue.items[2], frame_with_id(10, 1));
        assert_eq!(queue.items[3], frame_with_id(10, 2));
    }

    #[test]
    fn insert_blocks_different_ids() {
        let mut queue = ArrayQueue::<(), 8>::new();

        queue.push_frame(frame_with_id(10, 0)).unwrap();
        queue.push_frame(frame_with_id(10, 1)).unwrap();

        queue.push_frame(frame_with_id(5, 0)).unwrap();
        queue.push_frame(frame_with_id(5, 1)).unwrap();

        queue.push_frame(frame_with_id(10, 2)).unwrap();
        queue.push_frame(frame_with_id(10, 3)).unwrap();

        assert_eq!(queue.items[0], frame_with_id(5, 0));
        assert_eq!(queue.items[1], frame_with_id(5, 1));
        assert_eq!(queue.items[2], frame_with_id(10, 0));
        assert_eq!(queue.items[3], frame_with_id(10, 1));
        assert_eq!(queue.items[4], frame_with_id(10, 2));
        assert_eq!(queue.items[5], frame_with_id(10, 3));
    }

    #[test]
    fn insert_and_remove_capacity_1() {
        let mut queue = ArrayQueue::<(), 1>::new();
        assert_eq!(queue.len(), 0);
        let frame = frame_with_id(37, 2);
        queue.push_frame(frame.clone()).unwrap();
        assert_eq!(queue.len(), 1);
        // No space for another frame
        assert!(queue.push_frame(frame.clone()).is_err());

        assert_eq!(&frame, queue.peek_frame().unwrap());
        assert_eq!(queue.len(), 1);
        let removed = queue.pop_frame().unwrap();
        assert_eq!(frame, removed);
        assert_eq!(queue.len(), 0);

        // Put the frame back
        queue.return_frame(removed).unwrap();
        assert_eq!(queue.len(), 1);
        assert_eq!(frame, queue.items[0]);
    }

    #[test]
    fn insert_and_remove_capacity_8() {
        let mut queue = ArrayQueue::<(), 8>::new();

        // Send a low-priority transfer that fills up the queue
        queue.try_reserve(8).unwrap();
        queue.push_frame(frame_with_id(128, 0)).unwrap();
        queue.push_frame(frame_with_id(128, 1)).unwrap();
        queue.push_frame(frame_with_id(128, 2)).unwrap();
        queue.push_frame(frame_with_id(128, 3)).unwrap();
        queue.push_frame(frame_with_id(128, 4)).unwrap();
        queue.push_frame(frame_with_id(128, 5)).unwrap();
        queue.push_frame(frame_with_id(128, 6)).unwrap();
        queue.push_frame(frame_with_id(128, 7)).unwrap();

        assert_eq!(queue.len(), 8);

        // A small high-priority transfer can't be added
        assert!(queue.try_reserve(3).is_err());
        assert!(queue.push_frame(frame_with_id(10, 0)).is_err());
        assert!(queue.push_frame(frame_with_id(10, 1)).is_err());
        assert!(queue.push_frame(frame_with_id(10, 2)).is_err());

        // 3 low-priority frames move from the queue to the CAN controller transmit mailboxes
        assert_eq!(queue.pop_frame(), Some(frame_with_id(128, 0)));
        assert!(queue.try_reserve(3).is_err());
        assert_eq!(queue.pop_frame(), Some(frame_with_id(128, 1)));
        assert!(queue.try_reserve(3).is_err());
        assert_eq!(queue.pop_frame(), Some(frame_with_id(128, 2)));
        // Now there's space for the high-priority transfer
        queue.try_reserve(3).unwrap();
        queue.push_frame(frame_with_id(10, 0)).unwrap();
        queue.push_frame(frame_with_id(10, 1)).unwrap();
        queue.push_frame(frame_with_id(10, 2)).unwrap();
        assert_eq!(queue.len(), 8);

        {
            // The driver adapter removes a high-priority frame from the queue
            let removed_high_priority = queue.pop_frame().unwrap();
            assert_eq!(removed_high_priority, frame_with_id(10, 0));
            assert_eq!(queue.len(), 7);
            // The driver displaces a frame from the transmit mailbox, replacing it with the
            // high-priority frame
            // TODO: Of the three mailboxes with equal priority, which one dose it remove?
            // TODO: Test that on the real hardware to check that it does not cause reordering.
            // Assume that it removes from mailbox 0 if all are full.
            queue.return_frame(frame_with_id(128, 0)).unwrap();
            assert_eq!(queue.len(), 8);
        }

        // Remaining frames in the queue should be the other two high-priority frames, then the
        // 6 low-priority frames
        assert_eq!(queue.pop_frame(), Some(frame_with_id(10, 1)));
        assert_eq!(queue.pop_frame(), Some(frame_with_id(10, 2)));

        assert_eq!(queue.pop_frame(), Some(frame_with_id(128, 0)));
        assert_eq!(queue.pop_frame(), Some(frame_with_id(128, 3)));
        assert_eq!(queue.pop_frame(), Some(frame_with_id(128, 4)));
        assert_eq!(queue.pop_frame(), Some(frame_with_id(128, 5)));
        assert_eq!(queue.pop_frame(), Some(frame_with_id(128, 6)));
        assert_eq!(queue.pop_frame(), Some(frame_with_id(128, 7)));
    }
}
