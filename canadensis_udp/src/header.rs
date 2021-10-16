use crate::UdpTransferId;
use canadensis_core::Priority;
use core::mem;
use std::convert::TryFrom;
use zerocopy::{AsBytes, FromBytes};

pub const SIZE: usize = mem::size_of::<UdpHeader>();

/// The header placed in each UDP frame
#[derive(AsBytes, FromBytes, Debug)]
#[repr(C)]
pub struct UdpHeader {
    pub version: u8,
    pub priority: u8,
    pub _padding0: u16,
    pub frame_index_eot: u32,
    pub transfer_id: u64,
    pub _padding1: u64,
}

impl UdpHeader {
    /// Returns true if this is the last frame in a transfer
    pub fn is_last_frame(&self) -> bool {
        (self.frame_index_eot & LAST_FRAME) != 0
    }
    /// Returns the index of this frame in a transfer
    pub fn frame_index(&self) -> u32 {
        self.frame_index_eot & !LAST_FRAME
    }
}

/// Value to assign to the version field
pub const VERSION: u8 = 0;
/// Value to assign to the padding0 field
pub const PADDING0: u16 = 0;
/// Bit set in frame_index_eot if this is the last frame in the transfer
pub const LAST_FRAME: u32 = 0x8000_0000;

#[derive(Debug)]
pub struct ValidatedUdpHeader {
    pub priority: Priority,
    pub frame_index: u32,
    pub last_frame: bool,
    pub transfer_id: UdpTransferId,
}

impl TryFrom<UdpHeader> for ValidatedUdpHeader {
    type Error = ();

    fn try_from(header: UdpHeader) -> Result<Self, Self::Error> {
        if header.version != VERSION {
            return Err(());
        }
        let priority = match Priority::try_from(header.priority) {
            Ok(priority) => priority,
            Err(_) => return Err(()),
        };
        Ok(ValidatedUdpHeader {
            priority,
            frame_index: header.frame_index(),
            last_frame: header.is_last_frame(),
            transfer_id: header.transfer_id.into(),
        })
    }
}
