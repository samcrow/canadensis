//! Tests the Cyphal/CAN loopback behavior

extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_data_types;
extern crate canadensis_encoding;
extern crate canadensis_linux;
extern crate log;
extern crate simplelog;

use canadensis::node::CoreNode;
use canadensis::requester::TransferIdFixedMap;
use canadensis::{Node, PublishToken, ResponseToken, TransferHandler};
use canadensis_can::driver::{ReceiveDriver, TransmitDriver};
use canadensis_can::{CanNodeId, CanReceiver, CanTransmitter, CanTransport, Frame, Mtu};
use canadensis_core::subscription::Subscription;
use canadensis_core::time::{milliseconds, Microseconds64};
use canadensis_core::transfer::{MessageTransfer, ServiceTransfer, Transfer};
use canadensis_core::{OutOfMemoryError, Priority};
use canadensis_data_types::uavcan::time::synchronization_1_0::{self, Synchronization};
use canadensis_encoding::Deserialize;
use canadensis_linux::SystemClock;
use log::LevelFilter;
use simplelog::{ColorChoice, Config, TerminalMode};
use std::collections::vec_deque::VecDeque;
use std::convert::{Infallible, TryFrom};

#[test]
fn can_loopback_time_sync() {
    simplelog::TermLogger::init(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::default(),
        ColorChoice::Auto,
    )
    .unwrap();

    let node_id = CanNodeId::try_from(3_u8).unwrap();
    let mut node: CoreNode<
        SystemClock,
        CanTransmitter<Microseconds64, LoopbackOnlyDriver>,
        CanReceiver<Microseconds64, LoopbackOnlyDriver>,
        TransferIdFixedMap<CanTransport, 4>,
        LoopbackOnlyDriver,
        4,
        4,
    > = CoreNode::new(
        SystemClock::new(),
        node_id,
        CanTransmitter::new(Mtu::Can8),
        CanReceiver::new(node_id, Mtu::Can8),
        LoopbackOnlyDriver::default(),
    );

    // Need to subscribe to receive loopback transfers
    node.subscribe_message(synchronization_1_0::SUBJECT, 8, milliseconds(100))
        .unwrap();

    let sync_token: PublishToken<Synchronization> = node
        .start_publishing(
            synchronization_1_0::SUBJECT,
            milliseconds(100),
            Priority::Nominal,
        )
        .unwrap();
    // Send a non-loopback transfer, which should be ignored
    node.publish(
        &sync_token,
        &Synchronization {
            previous_transmission_timestamp_microsecond: 3,
        },
    )
    .unwrap();

    let mut collector = LoopbackCollector::default();
    node.receive(&mut collector)
        .expect("Unexpected error in receive");
    assert_eq!(0, collector.transfers.len());

    // Send a loopback transfer, which should be collected
    let loopback_payload = Synchronization {
        previous_transmission_timestamp_microsecond: 129,
    };
    node.publish_loopback(&sync_token, &loopback_payload)
        .unwrap();
    node.receive(&mut collector)
        .expect("Unexpected error in receive");
    assert_eq!(1, collector.transfers.len());
    let received_loopback = &collector.transfers[0];
    assert_eq!(received_loopback.loopback, true);
    assert_eq!(received_loopback.header.priority(), &Priority::Nominal);
    assert_eq!(received_loopback.header.source(), Some(&node_id));
    let loopback_deserialized_payload =
        Synchronization::deserialize_from_bytes(&received_loopback.payload).unwrap();
    assert_eq!(
        loopback_deserialized_payload.previous_transmission_timestamp_microsecond,
        loopback_payload.previous_transmission_timestamp_microsecond
    );
}

/// A CAN driver that handles loopback only
///
/// This driver discards all outgoing non-loopback frames and cannot receive any non-loopback
/// frames.
#[derive(Default)]
struct LoopbackOnlyDriver {
    loopback_frames: VecDeque<Frame<Microseconds64>>,
}

impl TransmitDriver<Microseconds64> for LoopbackOnlyDriver {
    type Error = Infallible;

    fn try_reserve(&mut self, _frames: usize) -> Result<(), OutOfMemoryError> {
        // Using std, there's no good way to detect out of memory
        Ok(())
    }

    fn transmit(
        &mut self,
        frame: Frame<Microseconds64>,
        now: Microseconds64,
    ) -> canadensis::nb::Result<Option<Frame<Microseconds64>>, Self::Error> {
        log::trace!("LoopbackOnlyDriver::transmit");
        if frame.timestamp() < now {
            log::debug!("Frame timed out");
            return Ok(None);
        }
        if !frame.loopback() {
            log::debug!("Discarding non-loopback frame");
            return Ok(None);
        }
        let mut loopback_frame = frame;
        loopback_frame.set_timestamp(now);
        self.loopback_frames.push_back(loopback_frame);
        Ok(None)
    }

    fn flush(&mut self, _now: Microseconds64) -> canadensis::nb::Result<(), Self::Error> {
        // Nothing to do
        Ok(())
    }
}
impl ReceiveDriver<Microseconds64> for LoopbackOnlyDriver {
    type Error = Infallible;

    fn receive(
        &mut self,
        _now: Microseconds64,
    ) -> canadensis::nb::Result<Frame<Microseconds64>, Self::Error> {
        self.loopback_frames
            .pop_front()
            .map(|frame| {
                log::trace!("Receiving loopback frame");
                frame
            })
            .ok_or(canadensis::nb::Error::WouldBlock)
    }

    fn apply_filters<S>(&mut self, _local_node: Option<CanNodeId>, _subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>,
    {
        // Not applicable
    }

    fn apply_accept_all(&mut self) {
        // Not applicable
    }
}

/// A transfer that collects all loopback transfers and panics if given any non-loopback transfer
#[derive(Default)]
struct LoopbackCollector {
    transfers: Vec<Transfer<Vec<u8>, Microseconds64, CanTransport>>,
}

impl TransferHandler<Microseconds64, CanTransport> for LoopbackCollector {
    fn handle_message<N: Node<Instant = Microseconds64, Transport = CanTransport>>(
        &mut self,
        _node: &mut N,
        _transfer: &MessageTransfer<Vec<u8>, Microseconds64, CanTransport>,
    ) -> bool {
        panic!("handle_message() called (not loopback)");
    }

    fn handle_request<N: Node<Instant = Microseconds64, Transport = CanTransport>>(
        &mut self,
        _node: &mut N,
        _token: ResponseToken<CanTransport>,
        _transfer: &ServiceTransfer<Vec<u8>, Microseconds64, CanTransport>,
    ) -> bool {
        panic!("handle_request() called (not loopback)");
    }

    fn handle_response<N: Node<Instant = Microseconds64, Transport = CanTransport>>(
        &mut self,
        _node: &mut N,
        _transfer: &ServiceTransfer<Vec<u8>, Microseconds64, CanTransport>,
    ) -> bool {
        panic!("handle_response() called (not loopback)");
    }

    fn handle_loopback<N: Node<Instant = Microseconds64, Transport = CanTransport>>(
        &mut self,
        _node: &mut N,
        transfer: &Transfer<Vec<u8>, Microseconds64, CanTransport>,
    ) -> bool {
        self.transfers.push(transfer.clone());
        true
    }
}
