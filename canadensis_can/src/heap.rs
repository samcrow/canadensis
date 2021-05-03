//!
//! A heap with fallible allocation, used as a priority queue for outgoing frames
//!

use alloc::vec::Vec;

use core::cmp::Ordering;
use fallible_collections::FallibleVec;
use fallible_collections::TryReserveError;

/// A binary min heap, with three extra features:
///
/// 1. Fallible allocations: push() and related functions return a TryReserveError if no memory
/// is available, instead of aborting.
///
/// 2. Transactions: A transaction can allocate memory for several items and stage them for
/// insertion. It is easy to commit the transaction (without allocating any more memory)
/// or roll back the transaction and discard the associated elements.
///
/// 3. Stability: Items that compare equal will maintain the order that they were inserted, and
/// will not be rearranged.
///
pub struct Heap<T> {
    /// Items in the heap (root is at index 0)
    data: Vec<Stable<T>>,
    /// The number of items that are in data but not in a pending transaction
    ///
    /// (any items at this index or beyond are not really in the heap)
    committed_size: usize,
}

impl<T> Heap<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Heap {
            data: Vec::new(),
            committed_size: 0,
        }
    }

    /// Starts a transaction to add items to this heap
    pub fn transaction(&mut self) -> Transaction<'_, T> {
        self.clear_pending();
        Transaction {
            heap: self,
            committed: false,
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.clear_pending();
        if self.data.is_empty() {
            None
        } else {
            let item = self.data.swap_remove(0);
            self.bubble_down();
            // Extract item from the Stable
            Some(item.item)
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first().map(|stable| &stable.item)
    }

    /// Removes all non-committed items
    fn clear_pending(&mut self) {
        while self.committed_size < self.data.len() {
            self.data.pop().unwrap();
        }
    }
    /// Makes all items in the heap committed and bubbles them up
    fn commit_transaction(&mut self) {
        // Bubble all items that haven't been committed
        for i in self.committed_size..self.data.len() {
            self.bubble_up(i);
        }
        self.committed_size = self.data.len();
    }
    fn push_pending(&mut self, item: T) -> Result<(), TryReserveError> {
        // Increment the index
        let index = self
            .data
            .last()
            .map(|last| last.index.wrapping_add(1))
            .unwrap_or(0);
        // Add this to the bottom, don't bubble yet
        FallibleVec::try_push(&mut self.data, Stable { item, index })
    }

    fn bubble_up(&mut self, mut index: usize) {
        while index != 0 {
            let parent = parent_index(index);
            if self.data[parent] > self.data[index] {
                self.data.swap(parent, index);
                // Loop again, looking at the place where the item was moved
                index = parent;
            } else {
                break;
            }
        }
    }

    fn bubble_down(&mut self) {
        let mut index = 0;
        loop {
            let (left, right) = child_indices(index);
            let smallest = if self
                .data
                .get(left)
                .map(|left_child| left_child < &self.data[index])
                .unwrap_or(false)
            {
                left
            } else {
                index
            };
            let smallest = if self
                .data
                .get(right)
                .map(|right_child| right_child < &self.data[smallest])
                .unwrap_or(false)
            {
                right
            } else {
                smallest
            };
            if smallest == index {
                break;
            } else {
                self.data.swap(index, smallest);
                index = smallest;
            }
        }
    }

    /// Removes any non-committed items and attempts to shrink the underlying storage to
    /// free memory
    pub fn shrink_to_fit(&mut self) {
        self.clear_pending();
        self.data.shrink_to_fit();
    }
}

/// A transaction operating on a heap
pub struct Transaction<'h, T>
where
    T: Ord,
{
    heap: &'h mut Heap<T>,
    committed: bool,
}

impl<'h, T> Transaction<'h, T>
where
    T: Ord,
{
    /// Adds an item to this transaction
    pub fn push(&mut self, item: T) -> Result<(), TryReserveError> {
        self.heap.push_pending(item)
    }

    /// Commits this transaction, putting all associated items in the heap
    ///
    /// This function is cannot fail because it does not allocate memory.
    pub fn commit(mut self) {
        self.committed = true;
        self.heap.commit_transaction();
    }

    /// Rolls this transaction back, removing all associated items
    pub fn rollback(self) {
        self.heap.clear_pending()
    }
}

impl<'h, T> Drop for Transaction<'h, T>
where
    T: Ord,
{
    fn drop(&mut self) {
        if !self.committed {
            self.heap.clear_pending()
        }
    }
}

fn child_indices(i: usize) -> (usize, usize) {
    (2 * i + 1, 2 * i + 2)
}

fn parent_index(i: usize) -> usize {
    (i - 1) / 2
}

#[cfg(test)]
mod test {
    use super::*;
    use core::mem;

    #[test]
    fn test_transaction_one() {
        let mut heap = Heap::<i32>::new();

        let mut transaction = heap.transaction();
        transaction.push(4).unwrap();
        transaction.commit();

        assert_eq!(Some(4), heap.pop());
        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_transaction_one_rollback() {
        let mut heap = Heap::<i32>::new();

        let mut transaction = heap.transaction();
        transaction.push(4).unwrap();
        transaction.rollback();

        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_transaction_implicit_drop() {
        let mut heap = Heap::<i32>::new();
        {
            let mut transaction = heap.transaction();
            transaction.push(4).unwrap();
        }

        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_transaction_forget() {
        let mut heap = Heap::<i32>::new();

        let mut transaction = heap.transaction();
        transaction.push(4).unwrap();
        mem::forget(transaction);

        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_order_insert_in_order() {
        let mut heap = Heap::<i32>::new();

        let mut transaction = heap.transaction();
        transaction.push(1).unwrap();
        transaction.push(2).unwrap();
        transaction.push(3).unwrap();
        transaction.push(4).unwrap();
        transaction.commit();

        assert_eq!(Some(1), heap.pop());
        assert_eq!(Some(2), heap.pop());
        assert_eq!(Some(3), heap.pop());
        assert_eq!(Some(4), heap.pop());
        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_order_insert_reverse_order() {
        let mut heap = Heap::<i32>::new();

        let mut transaction = heap.transaction();
        transaction.push(4).unwrap();
        transaction.push(3).unwrap();
        transaction.push(2).unwrap();
        transaction.push(1).unwrap();
        transaction.commit();

        assert_eq!(Some(1), heap.pop());
        assert_eq!(Some(2), heap.pop());
        assert_eq!(Some(3), heap.pop());
        assert_eq!(Some(4), heap.pop());
        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_order_insert_other_order() {
        let mut heap = Heap::<i32>::new();

        let mut transaction = heap.transaction();
        transaction.push(4).unwrap();
        transaction.push(1).unwrap();
        transaction.push(3).unwrap();
        transaction.push(2).unwrap();
        transaction.commit();

        assert_eq!(Some(1), heap.pop());
        assert_eq!(Some(2), heap.pop());
        assert_eq!(Some(3), heap.pop());
        assert_eq!(Some(4), heap.pop());
        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_insert_equal() {
        let mut heap = Heap::<i32>::new();

        let mut transaction = heap.transaction();
        transaction.push(1).unwrap();
        transaction.push(1).unwrap();
        transaction.push(1).unwrap();
        transaction.push(1).unwrap();
        transaction.commit();

        assert_eq!(Some(1), heap.pop());
        assert_eq!(Some(1), heap.pop());
        assert_eq!(Some(1), heap.pop());
        assert_eq!(Some(1), heap.pop());
        assert_eq!(None, heap.pop());
    }

    #[test]
    fn test_equal_elements_stable() {
        struct ComparesEqual(u32);
        impl PartialEq for ComparesEqual {
            fn eq(&self, _other: &Self) -> bool {
                true
            }
        }
        impl Eq for ComparesEqual {}
        impl PartialOrd for ComparesEqual {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(Ord::cmp(self, other))
            }
        }
        impl Ord for ComparesEqual {
            fn cmp(&self, _other: &Self) -> Ordering {
                Ordering::Equal
            }
        }

        let mut heap = Heap::new();
        let mut transaction = heap.transaction();
        for i in 0..1024 {
            transaction.push(ComparesEqual(i)).unwrap();
        }
        transaction.commit();

        // Items compare equal, but they should come out in order 0->1023
        for i in 0..1024 {
            let removed = heap.pop().unwrap();
            assert_eq!(i, removed.0);
        }
    }

    #[test]
    fn test_equal_elements_reinsert_order() {
        /// A struct compared based only on element 0, ignoring element 1
        struct PartialCompare(u32, u32);
        impl Eq for PartialCompare {}
        impl PartialEq for PartialCompare {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl Ord for PartialCompare {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.cmp(&other.0)
            }
        }
        impl PartialOrd for PartialCompare {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut heap = Heap::new();
        let mut transaction = heap.transaction();
        // (1, 1) and (1, 2) compare equal
        transaction.push(PartialCompare(1, 1)).unwrap();
        transaction.push(PartialCompare(1, 2)).unwrap();
        transaction.push(PartialCompare(2, 1)).unwrap();
        transaction.push(PartialCompare(3, 1)).unwrap();
        transaction.commit();

        // Remove (1, 1)
        let removed = heap.pop().unwrap();
        assert_eq!(removed.0, 1);
        assert_eq!(removed.1, 1);
        // Put (1, 1) back
        let mut transaction = heap.transaction();
        transaction.push(removed).unwrap();
        transaction.commit();
        // Because the ordering in the heap is stable, (1, 1) must end up at the front again,
        // not behind (1, 2)
        let front = heap.peek().unwrap();
        assert_eq!(front.0, 1);
        assert_eq!(front.1, 1);
    }
}

/// A heap is normally not stable, meaning that it may arbitrarily rearrange items that compare
/// equal. This adapter keeps track of the order of insertion into the heap and uses that to break
/// ties, which prevents rearrangement.
#[derive(Eq, PartialEq)]
struct Stable<T> {
    item: T,
    /// The index of this item in the sequence of items inserted
    ///
    /// This may wrap around, but because it's a usize it can only have one discontinuity
    /// in the heap at a time.
    index: usize,
}

impl<T> PartialOrd for Stable<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl<T> Ord for Stable<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare first by the item
        self.item.cmp(&other.item).then_with(|| {
            // If the items are equal, break the tie by index of insertion.
            // The fancy comparison handles wraparound.
            // https://www.rapitasystems.com/blog/what-happened-first-handling-timer-wraparound
            if self.index == other.index {
                // Shouldn't happen
                Ordering::Equal
            } else if (other.index.wrapping_sub(self.index)) < usize::MAX / 2 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }
}

#[cfg(test)]
mod test_stable {
    use super::*;

    #[test]
    fn test_stable_ord() {
        assert!(Stable { item: 1, index: 0 } < Stable { item: 1, index: 1 });
        // Near overflow
        assert!(
            Stable {
                item: 1,
                index: usize::MAX - 1
            } < Stable {
                item: 1,
                index: usize::MAX
            }
        );
        // Overflowed
        assert!(
            Stable {
                item: 1,
                index: usize::MAX
            } < Stable { item: 1, index: 0 }
        );
    }
}
