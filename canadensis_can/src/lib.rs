#![cfg_attr(not(test), no_std)]

//!
//! # CAN and CAN FD transport for UAVCAN v1.0
//!
//! This library implements the UAVCAN/CAN transport layer. It splits transfers into frames,
//! reassembles frames into transfers, and keeps track of subscriptions.
//!

extern crate alloc;
extern crate fallible_collections;

mod crc;
mod data;
mod error;
mod heap;
mod rx;
pub mod transfer;
mod tx;

pub use crate::data::*;
pub use crate::error::*;

pub use crate::rx::Receiver;
pub use crate::tx::Transmitter;
