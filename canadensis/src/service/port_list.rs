use core::marker::PhantomData;

use crate::core::time::milliseconds;
use crate::core::Priority;
use crate::{Node, PublishError, StartSendError};
use canadensis_core::nb;
use canadensis_core::transport::Transmitter;
use canadensis_data_types::uavcan::node::port::list_1_0::{List, SUBJECT};
use canadensis_data_types::uavcan::node::port::service_id_list_1_0::ServiceIDList;
use canadensis_data_types::uavcan::node::port::subject_id_1_0::SubjectID;
use canadensis_data_types::uavcan::node::port::subject_id_list_1_0::SubjectIDList;
use canadensis_encoding::bits::BitArray;

/// Publishes `uavcan.node.port.List` messages
pub struct PortListService<N> {
    _node: PhantomData<N>,
}

impl<N> PortListService<N>
where
    N: Node,
{
    /// Creates a new PortListService
    pub fn new(node: &mut N) -> Result<Self, NewError<N>> {
        if node.node_id().is_none() {
            return Err(NewError::Anonymous);
        }
        let token = node
            .start_publishing(SUBJECT, milliseconds(10000), Priority::Optional.into())
            .map_err(|err| match err {
                StartSendError::Memory(_) => NewError::OutOfMemory,
                StartSendError::Duplicate => NewError::Duplicate,
                StartSendError::Transport(err) => NewError::Other(err),
                StartSendError::AnonymousRequest => unreachable!(), // We are publishing a message so this should never happen
            })?;
        Ok(Self { _node: PhantomData })
    }

    /// Publishes a `uavcan.node.port.List` message. This function should be called at least every 10 seconds.
    pub fn publish_port_list(
        &mut self,
        node: &mut N,
    ) -> nb::Result<(), PublishError<<N::Transmitter as Transmitter<N::Clock>>::Error>> {
        node.publish(
            SUBJECT,
            &List {
                publishers: SubjectIDList::SparseList(
                    node.publishers()
                        .map(|x| SubjectID { value: x.into() })
                        .collect(),
                ),
                subscribers: SubjectIDList::SparseList(
                    node.subscribers()
                        .map(|x| SubjectID { value: x.into() })
                        .collect(),
                ),
                clients: ServiceIDList {
                    mask: node.clients().fold(BitArray::new(512), |mut bits, x| {
                        bits.set(x.into(), true);
                        bits
                    }),
                },
                servers: ServiceIDList {
                    mask: node.servers().fold(BitArray::new(512), |mut bits, x| {
                        bits.set(x.into(), true);
                        bits
                    }),
                },
            },
        )
    }
}

/// Error type returned by [`PortListService::new`]
pub enum NewError<N: Node> {
    /// The service could not allocate a publish token due to an out of memory error.
    OutOfMemory,
    /// The service could not allocate a publish token as the subject is already in use.
    Duplicate,
    /// The service could not allocate a publish token due to a transmitter error.
    Other(<N::Transmitter as Transmitter<N::Clock>>::Error),
    /// The node is anonymous
    Anonymous,
}
