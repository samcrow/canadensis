use core::mem;
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

/// Value to assign to the version field
pub const VERSION: u8 = 0;
/// Value to assign to the padding0 field
pub const PADDING0: u16 = 0;
/// Bit set in frame_index_eot if this is the last frame in the transfer
pub const LAST_FRAME: u32 = 0x8000_0000;
