#![no_std]
#![deny(missing_docs)]

//!
//! # CAN and CAN FD transport for UAVCAN v1.0
//!
//! This library implements the UAVCAN/CAN transport layer. It splits transfers into frames,
//! reassembles frames into transfers, and keeps track of subscriptions.
//!

extern crate alloc;
extern crate canadensis_core;
extern crate canadensis_filter_config;
extern crate fallible_collections;
extern crate hash32;
extern crate hash32_derive;
extern crate heapless;
extern crate log;

pub use crate::crc::TransferCrc;
pub use crate::data::*;
pub use crate::rx::CanReceiver;
pub use crate::tx::CanTransmitter;
pub use crate::types::*;

mod crc;
mod data;
pub mod driver;
pub mod queue;
pub mod redundant;
mod rx;
mod tx;
mod types;

use core::cmp;

/// Calculates the number of frames required and the number of padding bytes to add to a payload so
/// that all frames will have valid length values for CAN FD
///
/// * `payload_length`: The number of bytes of payload
/// * `mtu`: The maximum size of a frame
fn calculate_frame_stats(payload_length: usize, mtu: usize) -> FrameStats {
    assert!(mtu <= 64, "MTU too large for CAN FD");
    assert!(mtu > 1, "MTU too small");
    let mtu_without_tail = mtu - 1;

    // Calculate the length of the payload, CRC, and tail bytes
    let crc_length = if payload_length <= mtu_without_tail {
        // Fits into one frame, no need to add a transfer CRC
        0
    } else {
        // Add two bytes for the transfer CRC
        2
    };
    // Total length of all tail bytes
    // Divide and round up (minimum 1 tail byte)
    let tail_bytes = cmp::max(
        1,
        (payload_length + crc_length + (mtu_without_tail - 1)) / mtu_without_tail,
    );
    // Total length of the payloads of all frames, including CRC and tail bytes
    let total_length = payload_length + crc_length + tail_bytes;
    let frames = (total_length + mtu - 1) / mtu;

    // Get the number of bytes in the last frame (may be 0)
    let last_frame_length = total_length % mtu;
    let last_frame_rounded_length = round_up_frame_length(last_frame_length);
    let last_frame_padding = last_frame_rounded_length - last_frame_length;

    FrameStats {
        frames,
        last_frame_padding,
    }
}

/// Information about how to fit a transfer payload into frames
#[derive(Debug, Eq, PartialEq)]
struct FrameStats {
    /// The total number of frames
    pub frames: usize,
    /// The number of bytes that must be added to the last frame to give it a valid length
    /// for CAN FD
    pub last_frame_padding: usize,
}

/// Rounds up a frame length to a value that can be represented by a CAN FD data length code
fn round_up_frame_length(length: usize) -> usize {
    match length {
        0..=8 => length,
        9..=12 => 12,
        13..=16 => 16,
        17..=20 => 20,
        21..=24 => 24,
        25..=32 => 32,
        33..=48 => 48,
        49..=64 => 64,
        _ => panic!("MTU too large for CAN FD"),
    }
}
