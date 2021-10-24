//! Methods to keep track of receive sessions

use crate::time::Instant;
use crate::OutOfMemoryError;
use alloc::collections::BTreeMap;
use core::fmt::Debug;
use heapless::LinearMap;

/// Something that can keep track of receive sessions associated with other nodes
///
/// This is some kind of map from node ID to session.
///
/// Type parameters:
/// * `I`: A time instant
/// * `N`: A node ID
/// * `T`: A transfer ID
/// * `D`: Additional transport-specific session data
pub trait SessionTracker<I, N, T, D>
where
    I: Instant,
{
    /// Returns a reference to the session for the provided node, if one exists
    fn get(&self, node: N) -> Option<&Session<I, T, D>>;

    /// Returns a mutable reference to the session for the provided node, if one exists
    fn get_mut(&mut self, node: N) -> Option<&mut Session<I, T, D>>;

    //noinspection RsSelfConvention
    /// Returns a mutable reference to the session for the provided node
    ///
    /// If no session exists, this function calls the provided function, inserts the result,
    /// and returns a mutable reference to it.
    fn get_mut_or_insert_with<F>(
        &mut self,
        node: N,
        generator: F,
    ) -> Result<&mut Session<I, T, D>, OutOfMemoryError>
    where
        N: Clone,
        F: FnOnce() -> Session<I, T, D>;

    /// Inserts a session
    ///
    /// If another session with the same node already exists, it is removed.
    fn insert(&mut self, node: N, session: Session<I, T, D>) -> Result<(), OutOfMemoryError>;

    /// Removes all sessions that have expired
    fn remove_expired(&mut self, now: I);
}

/// A session, associated with a port ID and source node ID
///
/// Multiple transfers may be received during the lifetime of a session
pub struct Session<I, T, D>
where
    I: Instant,
{
    /// The time when a frame for this session was last received
    last_activity: I,
    /// The timeout for this session
    ///
    /// This session will be deleted if it has not had any activity for this duration
    timeout: I::Duration,
    /// The ID of the last successfully received transfer, if any
    ///
    /// This is used to eliminate duplicate transfers.
    last_transfer_id: Option<T>,
    /// Additional transport-specific data
    data: D,
}

impl<I, T, D> Session<I, T, D>
where
    I: Instant,
{
    /// Creates a new session
    pub fn new(
        last_activity: I,
        timeout: I::Duration,
        last_transfer_id: Option<T>,
        data: D,
    ) -> Self {
        Session {
            last_activity,
            timeout,
            last_transfer_id,
            data,
        }
    }

    /// Returns true if this session has expired and should be removed
    pub fn is_expired(&self, now: I) -> bool {
        now.duration_since(&self.last_activity) > self.timeout
    }

    /// Returns the time when this session was last active
    pub fn last_activity(&self) -> &I {
        &self.last_activity
    }
    /// Sets the time when this session was last active
    pub fn set_last_activity(&mut self, time: I) {
        self.last_activity = time;
    }
    /// Returns the timeout duration for this session
    pub fn timeout(&self) -> I::Duration {
        self.timeout
    }
    /// Returns the ID of the last received transfer, if any
    pub fn last_transfer_id(&self) -> Option<&T> {
        self.last_transfer_id.as_ref()
    }
    /// Sets the ID of the most recently received transfer
    pub fn set_last_transfer_id(&mut self, id: T) {
        self.last_transfer_id = Some(id);
    }
    /// Returns a reference to the transport-specific data
    pub fn data(&self) -> &D {
        &self.data
    }
    /// Returns a mutable reference to the transport-specific data
    pub fn data_mut(&mut self) -> &mut D {
        &mut self.data
    }
}

/// A fixed-capacity session map that uses linear search to find sessions
///
/// This implementation offers configurable memory use. Its time complexity is `O(C)`.
///
/// Type parameters:
/// * `I`: A time instant
/// * `N`: A node ID
/// * `T`: A transfer ID
/// * `D`: Additional transport-specific session data
/// * `C` (usize): Maximum number of sessions to store simultaneously
pub struct SessionLinearMap<I, N, T, D, const C: usize>
where
    I: Instant,
{
    sessions: LinearMap<N, Session<I, T, D>, C>,
}

impl<I, N, T, D, const C: usize> SessionTracker<I, N, T, D> for SessionLinearMap<I, N, T, D, C>
where
    I: Instant,
    N: Eq,
    N: Clone,
{
    fn get(&self, node: N) -> Option<&Session<I, T, D>> {
        self.sessions.get(&node)
    }

    fn get_mut(&mut self, node: N) -> Option<&mut Session<I, T, D>> {
        self.sessions.get_mut(&node)
    }

    fn get_mut_or_insert_with<F>(
        &mut self,
        node: N,
        generator: F,
    ) -> Result<&mut Session<I, T, D>, OutOfMemoryError>
    where
        N: Clone,
        F: FnOnce() -> Session<I, T, D>,
    {
        if !self.sessions.contains_key(&node) {
            self.sessions
                .insert(node.clone(), generator())
                .map_err(|_| OutOfMemoryError)?;
        }
        Ok(self.sessions.get_mut(&node).unwrap())
    }

    fn insert(&mut self, node: N, session: Session<I, T, D>) -> Result<(), OutOfMemoryError> {
        self.sessions
            .insert(node, session)
            .map(drop)
            .map_err(|_| OutOfMemoryError)
    }

    fn remove_expired(&mut self, now: I) {
        loop {
            let mut expired_node_id: Option<N> = None;
            for (id, session) in &self.sessions {
                if session.is_expired(now) {
                    expired_node_id = Some(id.clone());
                    break;
                }
            }
            match expired_node_id {
                Some(id) => {
                    self.sessions.remove(&id);
                }
                None => break,
            }
        }
    }
}

/// A fixed-capacity array of sessions, with one session slot for each possible node ID
///
/// `C` must be one greater than the maximum node ID value.
///
/// This implementation uses a consistent large amount of memory and operates in constant time.
/// Its `insert` function never fails.
pub struct SessionArray<I, T, D, const C: usize>
where
    I: Instant,
{
    /// A session for every node ID
    sessions: [Option<Session<I, T, D>>; C],
}

impl<I, N, T, D, const C: usize> SessionTracker<I, N, T, D> for SessionArray<I, T, D, C>
where
    I: Instant,
    N: Into<usize>,
{
    fn get(&self, node: N) -> Option<&Session<I, T, D>> {
        self.sessions[node.into()].as_ref()
    }

    fn get_mut(&mut self, node: N) -> Option<&mut Session<I, T, D>> {
        self.sessions[node.into()].as_mut()
    }

    fn get_mut_or_insert_with<F>(
        &mut self,
        node: N,
        generator: F,
    ) -> Result<&mut Session<I, T, D>, OutOfMemoryError>
    where
        N: Clone,
        F: FnOnce() -> Session<I, T, D>,
    {
        let entry = &mut self.sessions[node.into()];
        if entry.is_none() {
            *entry = Some(generator());
        }
        Ok(entry.as_mut().unwrap())
    }

    fn insert(&mut self, node: N, session: Session<I, T, D>) -> Result<(), OutOfMemoryError> {
        self.sessions[node.into()] = Some(session);
        Ok(())
    }

    fn remove_expired(&mut self, now: I) {
        for entry in &mut self.sessions {
            let mut remove = false;
            if let Some(session) = entry {
                if session.is_expired(now) {
                    remove = true;
                }
            }
            if remove {
                *entry = None;
            }
        }
    }
}

/// A session map that uses dynamic memory allocation
///
/// **Caution:** This implementation cannot detect when memory allocation fails, so it may cause
/// the program to abort. Only use it when memory is plentiful.
///
/// This implementation uses variable amounts of memory and takes `O(log(number of sessions))` time.
pub struct SessionDynamicMap<I, N, T, D>
where
    I: Instant,
{
    sessions: BTreeMap<N, Session<I, T, D>>,
}

impl<I, N, T, D> Default for SessionDynamicMap<I, N, T, D>
where
    I: Instant,
    N: Ord,
{
    fn default() -> Self {
        SessionDynamicMap {
            sessions: BTreeMap::default(),
        }
    }
}

impl<I, N, T, D> SessionTracker<I, N, T, D> for SessionDynamicMap<I, N, T, D>
where
    I: Instant,
    N: Ord + Clone + Debug,
{
    fn get(&self, node: N) -> Option<&Session<I, T, D>> {
        self.sessions.get(&node)
    }

    fn get_mut(&mut self, node: N) -> Option<&mut Session<I, T, D>> {
        self.sessions.get_mut(&node)
    }

    fn get_mut_or_insert_with<F>(
        &mut self,
        node: N,
        generator: F,
    ) -> Result<&mut Session<I, T, D>, OutOfMemoryError>
    where
        N: Clone,
        F: FnOnce() -> Session<I, T, D>,
    {
        Ok(self.sessions.entry(node).or_insert_with(generator))
    }

    fn insert(&mut self, node: N, session: Session<I, T, D>) -> Result<(), OutOfMemoryError> {
        let _ = self.sessions.insert(node, session);
        Ok(())
    }

    fn remove_expired(&mut self, now: I) {
        loop {
            let mut expired_node_id: Option<N> = None;
            for (id, session) in &self.sessions {
                if session.is_expired(now) {
                    expired_node_id = Some(id.clone());
                    break;
                }
            }
            match expired_node_id {
                Some(id) => {
                    log::debug!("Removing expired session from node {:?}", id);
                    self.sessions.remove(&id);
                }
                None => break,
            }
        }
    }
}
