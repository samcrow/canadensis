use crate::{Node, ResponseToken, ServiceTransfer, TransferHandler};
use alloc::vec::Vec;
use canadensis_core::{time::milliseconds, transport::Receiver};
use canadensis_data_types::uavcan::node::get_info_1_0::{GetInfoResponse, SERVICE};
use core::marker::PhantomData;

/// A service that responds to `uavcan.node.GetInfo`
pub struct GetInfoService<N>
where
    N: Node,
{
    node_info: GetInfoResponse,
    _node: PhantomData<N>,
}

impl<N> GetInfoService<N>
where
    N: Node,
{
    /// Creates a new GetInfo service
    ///
    /// * `node`: The node to use for responding to requests
    pub fn new(
        node: &mut N,
        node_info: GetInfoResponse,
    ) -> Result<Self, <N::Receiver as Receiver<N::Clock>>::Error> {
        node.subscribe_request(SERVICE, 0, milliseconds(1000))?;

        Ok(Self {
            node_info,
            _node: PhantomData,
        })
    }

    /// Returns the handler for this service
    pub fn handler(&self) -> GetInfoServiceHandler<'_, N> {
        GetInfoServiceHandler { service: self }
    }
}

/// A handler for a `uavcan.node.GetInfo` request
pub struct GetInfoServiceHandler<'a, N>
where
    N: Node,
{
    service: &'a GetInfoService<N>,
}

impl<'a, N> TransferHandler<N::Transport> for GetInfoServiceHandler<'a, N>
where
    N: Node,
{
    fn handle_request<N2: Node<Transport = N::Transport>>(
        &mut self,
        node: &mut N2,
        token: ResponseToken<N2::Transport>,
        transfer: &ServiceTransfer<Vec<u8>, N2::Transport>,
    ) -> bool {
        if transfer.header.service != SERVICE {
            return false;
        }

        let _ = node.send_response(token, milliseconds(1000), &self.service.node_info);
        true
    }
}
