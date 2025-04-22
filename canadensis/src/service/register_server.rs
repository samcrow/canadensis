use crate::register::RegisterBlock;
use crate::{Node, ResponseToken, TransferHandler};
use alloc::vec::Vec;
use canadensis_core::transfer::ServiceTransfer;
use canadensis_core::ServiceSubscribeError;
use canadensis_core::{time::milliseconds, transport::Receiver};
use canadensis_data_types::uavcan::primitive::empty_1_0::Empty;
use canadensis_data_types::uavcan::register::access_1_0::{AccessRequest, AccessResponse};
use canadensis_data_types::uavcan::register::list_1_0::{ListRequest, ListResponse};
use canadensis_data_types::uavcan::register::name_1_0::Name;
use canadensis_data_types::uavcan::register::value_1_0::Value;
use canadensis_data_types::uavcan::register::{access_1_0, list_1_0};
use canadensis_data_types::uavcan::time::synchronized_timestamp_1_0::SynchronizedTimestamp;
use canadensis_encoding::Deserialize;
use core::marker::PhantomData;
use core::str;
use log::{debug, warn};

/// A service that responds to `uavcan.register.List` and `uavcan.register.Access`
pub struct RegisterServerService<N: Node, B: RegisterBlock> {
    registers: B,
    _node: PhantomData<N>,
}

impl<N, B> RegisterServerService<N, B>
where
    N: Node,
    B: RegisterBlock,
{
    /// Creates a new [`RegisterServerService`]
    ///
    /// * `node`: The node to use for responding to requests
    /// * `registers`: The register block to use for responding to requests
    pub fn new(
        node: &mut N,
        registers: B,
    ) -> Result<Self, ServiceSubscribeError<<N::Receiver as Receiver<N::Clock>>::Error>> {
        node.subscribe_request(list_1_0::SERVICE, 2, milliseconds(1000))?;
        node.subscribe_request(access_1_0::SERVICE, 515, milliseconds(1000))?;

        Ok(Self {
            registers,
            _node: PhantomData,
        })
    }

    /// Returns a reference to the register block
    pub fn registers(&self) -> &B {
        &self.registers
    }

    /// Returns a mutable reference to the register block
    pub fn registers_mut(&mut self) -> &mut B {
        &mut self.registers
    }

    /// Returns the handler for this service
    pub fn handler(&mut self) -> RegisterServerServiceHandler<'_, N, B> {
        RegisterServerServiceHandler { server: self }
    }
}

/// The [`TransferHandler`] for the [`RegisterServerService`]
pub struct RegisterServerServiceHandler<'a, N: Node, B: RegisterBlock> {
    server: &'a mut RegisterServerService<N, B>,
}

impl<N, B> TransferHandler<N::Transport> for RegisterServerServiceHandler<'_, N, B>
where
    N: Node,
    B: RegisterBlock,
{
    fn handle_request<N2: Node<Transport = N::Transport>>(
        &mut self,
        node: &mut N2,
        token: ResponseToken<N2::Transport>,
        transfer: &ServiceTransfer<Vec<u8>, N2::Transport>,
    ) -> bool {
        match transfer.header.service {
            list_1_0::SERVICE => {
                if let Ok(request) = ListRequest::deserialize_from_bytes(&transfer.payload) {
                    debug!("Received list request for {}", request.index as u16);
                    let response = match self
                        .server
                        .registers
                        .register_by_index(request.index.into())
                    {
                        None => ListResponse {
                            name: Name {
                                name: heapless::Vec::new(),
                            },
                        },
                        Some(register) => {
                            let mut name = register.name().as_bytes();
                            if name.len() > 256 {
                                name = &name[0..256];
                            }
                            let name = heapless::Vec::from_slice(name).unwrap();
                            ListResponse {
                                name: Name { name },
                            }
                        }
                    };
                    if let Err(err) = node.send_response(token, milliseconds(1000), &response) {
                        warn!("Failed to send response: {:?}", err);
                    }
                    true
                } else {
                    false
                }
            }
            access_1_0::SERVICE => {
                if let Ok(request) = AccessRequest::deserialize_from_bytes(&transfer.payload) {
                    let mut response = AccessResponse {
                        timestamp: SynchronizedTimestamp { microsecond: 0 },
                        mutable: false,
                        persistent: false,
                        value: Value::Empty(Empty {}),
                    };
                    if let Ok(name) = str::from_utf8(&request.name.name) {
                        debug!("Received access request for {}", name);
                        if let Some(register) = self.server.registers.register_by_name_mut(name) {
                            let access = register.access();
                            if access.mutable {
                                let _ = register.write(&request.value);
                            }
                            response = AccessResponse {
                                timestamp: SynchronizedTimestamp { microsecond: 0 },
                                mutable: access.mutable,
                                persistent: access.persistent,
                                value: register.read(),
                            }
                        }
                    };
                    if let Err(err) = node.send_response(token, milliseconds(1000), &response) {
                        warn!("Failed to send response: {:?}", err);
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
