//! Methods to keep track of receive sessions

use crate::time::Microseconds32;
use crate::OutOfMemoryError;
use alloc::collections::BTreeMap;
use core::fmt::Debug;
use heapless::LinearMap;

/// Something that can keep track of receive sessions associated with other nodes
///
/// This is some kind of map from node ID to session.
///
/// Type parameters:
/// * `N`: A node ID
/// * `T`: A transfer ID
/// * `D`: Additional transport-specific session data
pub trait SessionTracker<N, T, D> {
    /// Returns a reference to the session for the provided node, if one exists
    fn get(&self, node: N) -> Option<&Session<T, D>>;

    /// Returns a mutable reference to the session for the provided node, if one exists
    fn get_mut(&mut self, node: N) -> Option<&mut Session<T, D>>;

    //noinspection RsSelfConvention
    /// Returns a mutable reference to the session for the provided node
    ///
    /// If no session exists, this function calls the provided function, inserts the result,
    /// and returns a mutable reference to it.
    fn get_mut_or_insert_with<F>(
        &mut self,
        node: N,
        generator: F,
    ) -> Result<&mut Session<T, D>, OutOfMemoryError>
    where
        N: Clone,
        F: FnOnce() -> Session<T, D>;

    /// Inserts a session
    ///
    /// If another session with the same node already exists, it is removed.
    fn insert(&mut self, node: N, session: Session<T, D>) -> Result<(), OutOfMemoryError>;

    /// Removes a session for the provided node if one exists
    ///
    /// If no matching session exists, this function has no effect.
    fn remove(&mut self, node: N);
}

/// A session, associated with a port ID and source node ID
///
/// If a session for a specific port and node ID does not exist, this node has never received a
/// transfer.
#[derive(Debug)]
pub enum Session<T, D> {
    /// This node is in the process of reassembling a transfer
    Active(ActiveSession<T, D>),
    /// This node has successfully received a transfer
    Complete {
        /// The timestamp of the first frame of the most recent successfully received transfer
        time: Microseconds32,
        /// The ID of that transfer
        transfer_id: T,
    },
}

/// A session with an incoming transfer undergoing reassembly
#[derive(Debug)]
pub struct ActiveSession<T, D> {
    /// The timestamp of the first frame in this transfer
    pub time: Microseconds32,
    /// The ID of this transfer
    pub transfer_id: T,
    /// Additional transport-specific data
    pub data: D,
}

/// A fixed-capacity session map that uses linear search to find sessions
///
/// This implementation offers configurable memory use. Its time complexity is `O(C)`.
///
/// Type parameters:
/// * `N`: A node ID
/// * `T`: A transfer ID
/// * `D`: Additional transport-specific session data
/// * `C` (usize): Maximum number of sessions to store simultaneously
pub struct SessionLinearMap<N, T, D, const C: usize> {
    sessions: LinearMap<N, Session<T, D>, C>,
}

impl<N, T, D, const C: usize> SessionTracker<N, T, D> for SessionLinearMap<N, T, D, C>
where
    N: Eq,
    N: Clone,
{
    fn get(&self, node: N) -> Option<&Session<T, D>> {
        self.sessions.get(&node)
    }

    fn get_mut(&mut self, node: N) -> Option<&mut Session<T, D>> {
        self.sessions.get_mut(&node)
    }

    fn get_mut_or_insert_with<F>(
        &mut self,
        node: N,
        generator: F,
    ) -> Result<&mut Session<T, D>, OutOfMemoryError>
    where
        N: Clone,
        F: FnOnce() -> Session<T, D>,
    {
        if !self.sessions.contains_key(&node) {
            self.sessions
                .insert(node.clone(), generator())
                .map_err(|_| OutOfMemoryError)?;
        }
        Ok(self.sessions.get_mut(&node).unwrap())
    }

    fn insert(&mut self, node: N, session: Session<T, D>) -> Result<(), OutOfMemoryError> {
        self.sessions
            .insert(node, session)
            .map(drop)
            .map_err(|_| OutOfMemoryError)
    }

    fn remove(&mut self, node: N) {
        self.sessions.remove(&node);
    }
}

/// A fixed-capacity array of sessions, with one session slot for each possible node ID
///
/// `C` must be one greater than the maximum node ID value.
///
/// This implementation uses a consistent large amount of memory and operates in constant time.
/// Its `insert` function never fails.
pub struct SessionArray<T, D, const C: usize> {
    /// A session for every node ID
    sessions: [Option<Session<T, D>>; C],
}

impl<N, T, D, const C: usize> SessionTracker<N, T, D> for SessionArray<T, D, C>
where
    N: Into<usize>,
{
    fn get(&self, node: N) -> Option<&Session<T, D>> {
        self.sessions[node.into()].as_ref()
    }

    fn get_mut(&mut self, node: N) -> Option<&mut Session<T, D>> {
        self.sessions[node.into()].as_mut()
    }

    fn get_mut_or_insert_with<F>(
        &mut self,
        node: N,
        generator: F,
    ) -> Result<&mut Session<T, D>, OutOfMemoryError>
    where
        N: Clone,
        F: FnOnce() -> Session<T, D>,
    {
        let entry = &mut self.sessions[node.into()];
        if entry.is_none() {
            *entry = Some(generator());
        }
        Ok(entry.as_mut().unwrap())
    }

    fn insert(&mut self, node: N, session: Session<T, D>) -> Result<(), OutOfMemoryError> {
        self.sessions[node.into()] = Some(session);
        Ok(())
    }

    fn remove(&mut self, node: N) {
        self.sessions[node.into()] = None;
    }
}

/// A session map that uses dynamic memory allocation
///
/// **Caution:** This implementation cannot detect when memory allocation fails, so it may cause
/// the program to abort. Only use it when memory is plentiful.
///
/// This implementation uses variable amounts of memory and takes `O(log(number of sessions))` time.
pub struct SessionDynamicMap<N, T, D> {
    sessions: BTreeMap<N, Session<T, D>>,
}

impl<N, T, D> Default for SessionDynamicMap<N, T, D>
where
    N: Ord,
{
    fn default() -> Self {
        SessionDynamicMap {
            sessions: BTreeMap::default(),
        }
    }
}

impl<N, T, D> SessionTracker<N, T, D> for SessionDynamicMap<N, T, D>
where
    N: Ord + Clone + Debug,
{
    fn get(&self, node: N) -> Option<&Session<T, D>> {
        self.sessions.get(&node)
    }

    fn get_mut(&mut self, node: N) -> Option<&mut Session<T, D>> {
        self.sessions.get_mut(&node)
    }

    fn get_mut_or_insert_with<F>(
        &mut self,
        node: N,
        generator: F,
    ) -> Result<&mut Session<T, D>, OutOfMemoryError>
    where
        N: Clone,
        F: FnOnce() -> Session<T, D>,
    {
        Ok(self.sessions.entry(node).or_insert_with(generator))
    }

    fn insert(&mut self, node: N, session: Session<T, D>) -> Result<(), OutOfMemoryError> {
        let _ = self.sessions.insert(node, session);
        Ok(())
    }

    fn remove(&mut self, node: N) {
        self.sessions.remove(&node);
    }
}
