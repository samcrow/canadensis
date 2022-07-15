//!
//! High-level Cyphal node types
//!
//! Three different node implementations are provided with different features:
//!
//! * [`CoreNode`]: Keeps track of subscriptions and other state, but does not automatically
//!   send anything
//! * [`MinimalNode`]: Sends a heartbeat message every second (this is the minimum required
//!   application-layer functionality according to the Cyphal specification)
//! * [`BasicNode`]: Sends heartbeat messages, responds to GetInfo requests, and sends port list
//!   messages
//!

mod basic;
mod core;
mod minimal;

pub use self::basic::BasicNode;
pub use self::core::CoreNode;
pub use self::minimal::MinimalNode;

pub mod data_types {
    //! Re-exports from `canadensis_data_types` to avoid version conflicts
    pub use canadensis_data_types::uavcan::node::get_info_1_0::GetInfoResponse;
    pub use canadensis_data_types::uavcan::node::version_1_0::Version;
}

/// An error from a transmitter or receiver
#[derive(Debug)]
pub enum NodeError<T, R> {
    /// An error from a transmitter
    Transmitter(T),
    /// An error from a receiver
    Receiver(R),
}
