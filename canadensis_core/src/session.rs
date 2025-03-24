//! Methods to keep track of receive sessions

use crate::time::{MicrosecondDuration32, Microseconds32};
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

    /// Removes all sessions that have expired
    fn remove_expired(&mut self, now: Microseconds32);
}

/// A session, associated with a port ID and source node ID
///
/// Multiple transfers may be received during the lifetime of a session
pub struct Session<T, D> {
    /// The time when a frame for this session was last received
    last_activity: Microseconds32,
    /// The timeout for this session
    ///
    /// This session will be deleted if it has not had any activity for this duration
    timeout: MicrosecondDuration32,
    /// The ID of the last successfully received transfer, if any
    ///
    /// This is used to eliminate duplicate transfers.
    last_transfer_id: Option<T>,
    /// Additional transport-specific data
    data: D,
}

impl<T, D> Session<T, D> {
    /// Creates a new session
    pub fn new(
        last_activity: Microseconds32,
        timeout: MicrosecondDuration32,
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
    pub fn is_expired(&self, now: Microseconds32) -> bool {
        let deadline = self.last_activity + self.timeout;
        now > deadline
    }

    /// Returns the time when this session was last active
    pub fn last_activity(&self) -> Microseconds32 {
        self.last_activity
    }
    /// Sets the time when this session was last active
    pub fn set_last_activity(&mut self, time: Microseconds32) {
        self.last_activity = time;
    }
    /// Returns the timeout duration for this session
    pub fn timeout(&self) -> MicrosecondDuration32 {
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

#[cfg(test)]
mod test {
    use super::Session;
    use crate::time::{MicrosecondDuration32, Microseconds32};

    #[test]
    fn test_session_expiration_basic() {
        let session = Session::new(
            Microseconds32::from_ticks(300),
            MicrosecondDuration32::from_ticks(100),
            None::<()>,
            (),
        );
        assert!(!session.is_expired(Microseconds32::from_ticks(300)));
        assert!(!session.is_expired(Microseconds32::from_ticks(301)));
        assert!(!session.is_expired(Microseconds32::from_ticks(399)));
        assert!(!session.is_expired(Microseconds32::from_ticks(400)));
        assert!(session.is_expired(Microseconds32::from_ticks(401)));
    }

    #[test]
    fn test_session_expiration_wraparound() {
        let session = Session::new(
            Microseconds32::from_ticks(u32::MAX - 1),
            MicrosecondDuration32::from_ticks(100),
            None::<()>,
            (),
        );
        assert!(!session.is_expired(Microseconds32::from_ticks(u32::MAX - 1)));
        assert!(!session.is_expired(Microseconds32::from_ticks(u32::MAX)));
        assert!(!session.is_expired(Microseconds32::from_ticks(98)));
        assert!(session.is_expired(Microseconds32::from_ticks(99)));
        assert!(session.is_expired(Microseconds32::from_ticks(100)));
    }
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

    fn remove_expired(&mut self, now: Microseconds32) {
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

    fn remove_expired(&mut self, now: Microseconds32) {
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

    fn remove_expired(&mut self, now: Microseconds32) {
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
