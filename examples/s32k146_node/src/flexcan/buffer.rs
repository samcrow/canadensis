//! Data types for message buffers

pub mod header;

use self::header::Header;
use crate::flexcan::blocks::{
    MESSAGES_8_PER_BLOCK, MESSAGES_16_PER_BLOCK, MESSAGES_32_PER_BLOCK, MESSAGES_64_PER_BLOCK,
};
use crate::flexcan::{BUFFER_BLOCKS_OFFSET, REGISTER_BLOCK_MIN_ALIGNMENT};
use core::mem::{ManuallyDrop, offset_of};
use core::sync::atomic::{Ordering, compiler_fence};
use static_assertions::const_assert_eq;
use vcell::VolatileCell;

/// A message block, with a header and up to `N` bytes of data
#[repr(C)]
pub(crate) struct Message<const N: usize> {
    /// The header (status, flags, IDs) in the format that the hardware handles
    ///
    /// Caution: This is big-endian
    control_status: VolatileCell<u32>,
    /// The local priority and message ID, in the format that the hardware handles
    ///
    /// Caution: This is big-endian
    id: VolatileCell<u32>,
    /// Message data
    data: [u8; N],
}

impl<const N: usize> Message<N> {
    /// Reads the header from this message block
    ///
    /// This function reads the control and status first, and then the ID.
    pub(crate) fn read_header(&self) -> Header {
        let control_status = self.control_status.get();
        let id = self.id.get();
        Header::from_bits(control_status, id)
    }
    /// Writes the header of this message block
    ///
    /// This function writes the ID in one write operation, and then the control and status in another.
    ///
    /// Caution: This may make the hardware immediately try to send the message, depending on
    /// the contents of the header.
    pub(crate) fn write_header(&mut self, header: &Header) {
        let (control_status, id) = header.as_bits();
        // Don't let the compiler reorder these writes before write_data
        compiler_fence(Ordering::Acquire);
        self.id.set(id);
        self.control_status.set(control_status);
    }
    pub(crate) fn write_data(&mut self, data: &[u8]) {
        let length = data.len().min(N);
        // The layout of the data in the message buffer is unusual. In each group of four bytes,
        // the bytes are reversed.
        for i in 0..length {
            let dest_offset = shift_index(i);
            self.data[dest_offset] = data[i];
        }
        // Don't let the compiler reorder the write_header operations before the above writes
        compiler_fence(Ordering::Release);
    }
    pub(crate) fn read_data(&mut self, data: &mut [u8]) {
        let size = data.len().min(N);
        for i in 0..size {
            let source_offset = shift_index(i);
            data[i] = self.data[source_offset];
        }
    }
}

/// Converts a byte index in a message data field to an offset in the data section of a message buffer
pub(crate) fn shift_index(index: usize) -> usize {
    index ^ 3
}

#[cfg(test)]
#[test]
fn test_shift_index() {
    assert_eq!(3, shift_index(0));
    assert_eq!(2, shift_index(1));
    assert_eq!(1, shift_index(2));
    assert_eq!(0, shift_index(3));

    assert_eq!(7, shift_index(4));
    assert_eq!(6, shift_index(5));
    assert_eq!(5, shift_index(6));
    assert_eq!(4, shift_index(7));

    assert_eq!(11, shift_index(8));
    assert_eq!(10, shift_index(9));
    assert_eq!(9, shift_index(10));
    assert_eq!(8, shift_index(11));
}

pub(crate) type Message8 = Message<8>;
pub(crate) type Message16 = Message<16>;
pub(crate) type Message32 = Message<32>;
pub(crate) type Message64 = Message<64>;

/// 32x message blocks with 8-byte capacity
#[repr(C)]
struct BufferBlock8([Message8; MESSAGES_8_PER_BLOCK]);
/// 21x message blocks with 16-byte capacity, padded to 512 bytes
#[repr(C)]
struct BufferBlock16([Message16; MESSAGES_16_PER_BLOCK], [u8; 8]);
/// 12x message blocks with 32-byte capacity, padded to 512 bytes
#[repr(C)]
struct BufferBlock32([Message32; MESSAGES_32_PER_BLOCK], [u8; 32]);
/// 7x message blocks with 64-byte capacity, padded to 512 bytes
#[repr(C)]
struct BufferBlock64([Message64; MESSAGES_64_PER_BLOCK], [u8; 8]);

#[repr(C)]
pub(crate) union BufferBlock {
    payload8: ManuallyDrop<BufferBlock8>,
    payload16: ManuallyDrop<BufferBlock16>,
    payload32: ManuallyDrop<BufferBlock32>,
    payload64: ManuallyDrop<BufferBlock64>,
}

impl BufferBlock {
    /// Returns a slice of message blocks, with up to 8 data bytes per message
    ///
    /// # Safety
    ///
    /// This block must be set to use 8-byte message size in the FDCTRL register,
    /// and must have all its bytes initialized
    pub unsafe fn messages_8(&self) -> &[Message8; MESSAGES_8_PER_BLOCK] {
        unsafe { &self.payload8.0 }
    }
    /// Returns a unique slice of message blocks, with up to 8 data bytes per message
    ///
    /// # Safety
    ///
    /// This block must be set to use 8-byte message size in the FDCTRL register,
    /// and must have all its bytes initialized
    pub unsafe fn messages_8_mut(&mut self) -> &mut [Message8; MESSAGES_8_PER_BLOCK] {
        unsafe { &mut self.payload8.0 }
    }
    /// Returns a slice of message blocks, with up to 16 data bytes per message
    ///
    /// # Safety
    ///
    /// This block must be set to use 16-byte message size in the FDCTRL register,
    /// and must have all its bytes initialized
    pub unsafe fn messages_16(&self) -> &[Message16; MESSAGES_16_PER_BLOCK] {
        unsafe { &self.payload16.0 }
    }
    /// Returns a unique slice of message blocks, with up to 16 data bytes per message
    ///
    /// # Safety
    ///
    /// This block must be set to use 16-byte message size in the FDCTRL register,
    /// and must have all its bytes initialized
    pub unsafe fn messages_16_mut(&mut self) -> &mut [Message16; MESSAGES_16_PER_BLOCK] {
        unsafe { &mut self.payload16.0 }
    }
    /// Returns a slice of message blocks, with up to 32 data bytes per message
    ///
    /// # Safety
    ///
    /// This block must be set to use 32-byte message size in the FDCTRL register,
    /// and must have all its bytes initialized
    pub unsafe fn messages_32(&self) -> &[Message32; MESSAGES_32_PER_BLOCK] {
        unsafe { &self.payload32.0 }
    }
    /// Returns a unique slice of message blocks, with up to 32 data bytes per message
    ///
    /// # Safety
    ///
    /// This block must be set to use 32-byte message size in the FDCTRL register.,
    /// and must have all its bytes initialized
    pub unsafe fn messages_32_mut(&mut self) -> &mut [Message32; MESSAGES_32_PER_BLOCK] {
        unsafe { &mut self.payload32.0 }
    }
    /// Returns a slice of message blocks, with up to 64 data bytes per message
    ///
    /// # Safety
    ///
    /// This block must be set to use 64-byte message size in the FDCTRL register,
    /// and must have all its bytes initialized
    pub unsafe fn messages_64(&self) -> &[Message64; MESSAGES_64_PER_BLOCK] {
        unsafe { &self.payload64.0 }
    }
    /// Returns a unique slice of message blocks, with up to 64 data bytes per message
    ///
    /// # Safety
    ///
    /// This block must be set to use 64-byte message size in the FDCTRL register,
    /// and must have all its bytes initialized
    pub unsafe fn messages_64_mut(&mut self) -> &mut [Message64; MESSAGES_64_PER_BLOCK] {
        unsafe { &mut self.payload64.0 }
    }
}

// Message sizes and alignment
const_assert_eq!(16, size_of::<Message8>());
const_assert_eq!(24, size_of::<Message16>());
const_assert_eq!(40, size_of::<Message32>());
const_assert_eq!(72, size_of::<Message64>());

const_assert_eq!(4, align_of::<Message8>());
const_assert_eq!(4, align_of::<Message16>());
const_assert_eq!(4, align_of::<Message32>());
const_assert_eq!(4, align_of::<Message64>());

// Message fields
const_assert_eq!(0, offset_of!(Message8, control_status));
const_assert_eq!(4, offset_of!(Message8, id));
const_assert_eq!(8, offset_of!(Message8, data));
const_assert_eq!(0, offset_of!(Message16, control_status));
const_assert_eq!(4, offset_of!(Message16, id));
const_assert_eq!(8, offset_of!(Message16, data));
const_assert_eq!(0, offset_of!(Message32, control_status));
const_assert_eq!(4, offset_of!(Message32, id));
const_assert_eq!(8, offset_of!(Message32, data));
const_assert_eq!(0, offset_of!(Message64, control_status));
const_assert_eq!(4, offset_of!(Message64, id));
const_assert_eq!(8, offset_of!(Message64, data));

// Block sizes
const_assert_eq!(512, size_of::<BufferBlock8>());
const_assert_eq!(512, size_of::<BufferBlock16>());
const_assert_eq!(512, size_of::<BufferBlock32>());
const_assert_eq!(512, size_of::<BufferBlock64>());

const_assert_eq!(512, size_of::<BufferBlock>());

// Block alignment
const_assert_eq!(4, align_of::<BufferBlock8>());
const_assert_eq!(4, align_of::<BufferBlock16>());
const_assert_eq!(4, align_of::<BufferBlock32>());
const_assert_eq!(4, align_of::<BufferBlock64>());

const_assert_eq!(4, align_of::<BufferBlock>());

// Message alignment
const_assert_eq!(4, align_of::<Message8>());
const_assert_eq!(4, align_of::<Message16>());
const_assert_eq!(4, align_of::<Message32>());
const_assert_eq!(4, align_of::<Message64>());

// Alignment checks for buffers:
// The start of the register block must be 4-byte aligned,
// checked in FlexCan::from_address().
// The first buffer block is at offset 0x80 (a multiple of 4), so it's also aligned.
// Within a buffer block, every message block size (16, 24, 40, 72) is a multiple of 4, so
// each message is also aligned.
// Each subsequent block is 512 bytes after the previous block (a multiple of 4), so it's also aligned.
const_assert_eq!(
    0,
    (REGISTER_BLOCK_MIN_ALIGNMENT + BUFFER_BLOCKS_OFFSET + size_of::<BufferBlock>()) % 4
);
