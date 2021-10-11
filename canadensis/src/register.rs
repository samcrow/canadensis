//!
//! Node configuration registers that can be accessed from other nodes

pub mod basic;

use alloc::vec::Vec;
use core::str;

use crate::{Node, ResponseToken, TransferHandler};
use canadensis_can::OutOfMemoryError;
use canadensis_core::time::{milliseconds, Instant};
use canadensis_core::transfer::ServiceTransfer;
use canadensis_data_types::uavcan::register::access_1_0::{self, AccessRequest, AccessResponse};
use canadensis_data_types::uavcan::register::list_1_0::{self, ListRequest, ListResponse};
use canadensis_data_types::uavcan::register::name_1_0::Name;
use canadensis_data_types::uavcan::register::value_1_0::Value;
use canadensis_data_types::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp;
use canadensis_encoding::Deserialize;

pub use canadensis_derive_register_block::RegisterBlock;

/// A block of registers that can be accessed externally through the uavcan.register interface
///
/// This trait can be derived for any struct whose fields all implement [`Register`].
///
/// # Examples
///
/// ```
/// # use canadensis::register::basic::{SimpleRegister, RegisterString};
/// # use canadensis::register::RegisterBlock;
/// #[derive(RegisterBlock)]
/// struct Registers {
///     node_id: SimpleRegister<u16>,
///     description: SimpleRegister<RegisterString>,
/// }
///
/// impl Default for Registers {
///     fn default() -> Self {
///         Registers {
///             node_id: SimpleRegister::with_value("uavcan.node.id", true, true, 65535),
///             description: SimpleRegister::new("uavcan.node.description", true, true),
///         }
///     }
/// }
/// ```
pub trait RegisterBlock {
    /// Returns a reference to the register at the provided index
    ///
    /// Register indexes should start at 0 and not contain any gaps.
    fn register_by_index(&self, index: usize) -> Option<&dyn Register>;
    /// Returns a mutable reference to the register at the provided index
    ///
    /// Register indexes should start at 0 and not contain any gaps.
    fn register_by_index_mut(&mut self, index: usize) -> Option<&mut dyn Register>;
    /// Returns a mutable reference to the register with the provided name
    fn register_by_name_mut(&mut self, name: &str) -> Option<&mut dyn Register>;
}

/// Information about how a register can be accessed
#[derive(Debug, Clone)]
pub struct Access {
    /// If this register is mutable
    ///
    /// Mutable registers can be written by other nodes.
    pub mutable: bool,
    /// If this register is persistent
    ///
    /// Persistent registers are preserved when the node restarts.
    pub persistent: bool,
}

/// A register that can be read and optionally written
pub trait Register {
    /// Returns the name of this register
    ///
    /// The name must not be more than 256 bytes long. Each register must have a distinct name.
    fn name(&self) -> &str;

    /// Returns information about how this register can be accessed
    fn access(&self) -> Access;

    /// Reads this register and returns its value
    ///
    /// This function must not return `Value::Empty`.
    fn read(&self) -> Value;
    /// Writes the value of this register
    ///
    /// This function may be used on a register that is both persistent and mutable to load its
    /// value from persistent storage.
    ///
    /// Outside code must not call this function in response to a write request from another node
    /// if this register is not mutable.
    ///
    /// This function returns an error if the provided value does not have an appropriate type
    /// for this register.
    ///
    /// If this function returns an error, the value of this register must be the same as before
    /// the call to write().
    ///
    /// # Panics
    ///
    /// This function may panic if `self.access()` returned a value with `mutable` set to false.
    fn write(&mut self, value: &Value) -> Result<(), WriteError>;
}

/// Errors that can occur when attempting to write a register
#[derive(Debug)]
pub enum WriteError {
    /// The type of the value, or the number of values in an array, was incorrect
    Type,
}

/// Handles access requests for registers
///
/// Basic steps:
/// 1. Create a register block (this may be a tuple of `Register`s or a custom type that implements
///    `RegisterBlock`)
/// 2. Create a handler using `RegisterHandler::init`. Pass the register block and a node used to
///    receive service requests
/// 3. When calling `accept` on the node, pass the register handler as a transfer handler
///    (or use some other method to pass incoming service requests to the register handler).
///    This lets the register handler process requests and send responses.
pub struct RegisterHandler<B> {
    block: B,
}

impl<B> RegisterHandler<B>
where
    B: RegisterBlock,
{
    /// Creates a register handler
    pub fn new(block: B) -> Self {
        RegisterHandler { block }
    }

    /// Subscribes to register list and register access requests
    ///
    /// This function returns an error if the provided node does not have enough space to listen
    /// for requests.
    pub fn subscribe_requests<N>(node: &mut N) -> Result<(), OutOfMemoryError>
    where
        N: Node,
    {
        node.subscribe_request(access_1_0::SERVICE, 515, milliseconds(1000))?;
        node.subscribe_request(list_1_0::SERVICE, 2, milliseconds(0))?;
        Ok(())
    }

    /// Returns a reference to the register block
    ///
    /// This can be used to read the current values.
    pub fn block(&self) -> &B {
        &self.block
    }
    /// Returns a mutable reference to the register block
    ///
    /// This can be used to write the register values.
    pub fn block_mut(&mut self) -> &mut B {
        &mut self.block
    }

    fn handle_access_request(&mut self, request: &AccessRequest) -> AccessResponse {
        match str::from_utf8(&request.name.name) {
            Ok(register_name) => {
                log::debug!("Handling access request for {}", register_name);
                if let Some(register) = self.block.register_by_name_mut(register_name) {
                    register_handle_access(register, request)
                } else {
                    // Register doesn't exist, return empty
                    AccessResponse {
                        timestamp: SynchronizedTimestamp { microsecond: 0 },
                        mutable: false,
                        persistent: false,
                        value: Value::Empty(
                            canadensis_data_types::uavcan::primitive::empty_1_0::Empty {},
                        ),
                    }
                }
            }
            Err(_) => {
                // Invalid name, return empty
                AccessResponse {
                    timestamp: SynchronizedTimestamp { microsecond: 0 },
                    mutable: false,
                    persistent: false,
                    value: Value::Empty(
                        canadensis_data_types::uavcan::primitive::empty_1_0::Empty {},
                    ),
                }
            }
        }
    }

    fn handle_list_request(&mut self, request: &ListRequest) -> ListResponse {
        log::debug!("Handling register list request, index {}", {
            request.index
        });
        match self.block.register_by_index(request.index.into()) {
            Some(register) => {
                let name = register.name().as_bytes();
                // Truncate to 256 bytes if necessary
                let name = if name.len() <= 256 {
                    name
                } else {
                    &name[..256]
                };
                ListResponse {
                    name: Name {
                        name: heapless::Vec::from_slice(name).expect("Incorrect name length"),
                    },
                }
            }
            None => {
                // Empty name
                ListResponse {
                    name: Name {
                        name: heapless::Vec::new(),
                    },
                }
            }
        }
    }
}

fn register_handle_access(register: &mut dyn Register, request: &AccessRequest) -> AccessResponse {
    let access = register.access();
    if access.mutable
        && !matches!(
            request.value,
            Value::Empty(canadensis_data_types::uavcan::primitive::empty_1_0::Empty {})
        )
    {
        // Write errors are reported by returning the unmodified register value.
        let _ = register.write(&request.value);
    }
    // Now read the register and return its properties
    AccessResponse {
        timestamp: SynchronizedTimestamp { microsecond: 0 },
        mutable: access.mutable,
        persistent: access.persistent,
        value: register.read(),
    }
}

impl<I, B> TransferHandler<I> for RegisterHandler<B>
where
    I: Instant,
    B: RegisterBlock,
{
    fn handle_request<N: Node<Instant = I>>(
        &mut self,
        node: &mut N,
        token: ResponseToken,
        transfer: &ServiceTransfer<Vec<u8>, I>,
    ) -> bool {
        match transfer.header.service {
            access_1_0::SERVICE => {
                if let Ok(request) = AccessRequest::deserialize_from_bytes(&transfer.payload) {
                    let response = self.handle_access_request(&request);
                    let status = node.send_response(token, milliseconds(1000), &response);
                    if status.is_err() {
                        log::warn!("Out of memory when sending register access response");
                    }
                    true
                } else {
                    false
                }
            }
            list_1_0::SERVICE => {
                if let Ok(request) = ListRequest::deserialize_from_bytes(&transfer.payload) {
                    let response = self.handle_list_request(&request);
                    let status = node.send_response(token, milliseconds(1000), &response);
                    if status.is_err() {
                        log::warn!("Out of memory when sending register list response");
                    }
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
