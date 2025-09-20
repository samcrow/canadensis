use crate::flexcan::svd_generated::can::fdctrl::Mbdsr;
use core::slice;

/// Peripheral supports 256 bytes (half a RAM block) of buffers, for any of these configurations:
///
/// * 16x 16-byte buffers for 8-byte messages
/// * 10x 24-byte buffers for 16-byte messages (uncertain)
/// * 6x 40-byte buffers for 32-byte messages (uncertain)
/// * 3x 72-byte buffers for 64-byte messages (uncertain)
///
/// The contained array is the data capacity for each message in the buffer.
pub struct HalfBlock(pub MessageBufferSize);
/// Peripheral supports 512 bytes (one RAM block) of buffers, for any of these configurations:
///
/// * 32x 16-byte buffers for 8-byte messages
/// * 21x 24-byte buffers for 16-byte messages
/// * 12x 40-byte buffers for 32-byte messages
/// * 7x 72-byte buffers for 64-byte messages
///
/// The contained array is the data capacity for each message in the buffer.
pub struct OneBlock(pub MessageBufferSize);
/// Peripheral supports 1024 bytes (two RAM blocks) of buffers, for any of these configurations:
///
/// * 64x 16-byte buffers for 8-byte messages
/// * 42x 24-byte buffers for 16-byte messages
/// * 24x 40-byte buffers for 32-byte messages
/// * 14x 72-byte buffers for 64-byte messages
///
/// The contained array is the data capacity for each message in each of the buffers.
pub struct TwoBlocks(pub [MessageBufferSize; 2]);
/// Peripheral supports 1536 bytes (three RAM blocks) of buffers, for any of these configurations:
///
/// * 96x 16-byte buffers for 8-byte messages
/// * 63x 24-byte buffers for 16-byte messages
/// * 36x 40-byte buffers for 32-byte messages
/// * 21x 72-byte buffers for 64-byte messages
///
/// The contained array is the data capacity for each message in each of the buffers.
pub struct ThreeBlocks(pub [MessageBufferSize; 3]);
/// Peripheral supports 2048 bytes (four RAM blocks) of buffers, for any of these configurations:
///
/// * 128x 16-byte buffers for 8-byte messages
/// * 84x 24-byte buffers for 16-byte messages
/// * 48x 40-byte buffers for 32-byte messages
/// * 28x 72-byte buffers for 64-byte messages
///
/// The contained array is the data capacity for each message in each of the buffers.
pub struct FourBlocks(pub [MessageBufferSize; 4]);

/// Information about a FlexCAN peripheral's message buffer memory, and the sizes of messages
/// that each block can hold
pub unsafe trait Blocks {
    /// Number of blocks that the peripheral provides
    const NUM_BLOCKS: usize;
    /// Total number of bytes of memory available for message buffers
    const BUFFER_BYTES: usize;
    /// Returns a buffer size for each RAM block that this peripheral supports
    fn buffer_sizes(&self) -> &[MessageBufferSize];
    /// Returns the number of messages that each block can hold, based on the buffer sizes
    ///
    /// The number of messages will be 0 for a buffer that does not exist.
    fn messages_per_buffer(&self) -> [usize; 4] {
        let sizes = self.buffer_sizes();
        let mut messages = [0; 4];
        messages
            .iter_mut()
            .zip(sizes)
            .for_each(|(messages, size)| *messages = size.message_capacity_per_block());
        messages
    }
    /// Returns the total number of messages that all buffers can hold, based on the message size
    /// for each buffer
    fn total_messages(&self) -> usize {
        self.messages_per_buffer().into_iter().sum()
    }
}

unsafe impl Blocks for HalfBlock {
    const NUM_BLOCKS: usize = 1;
    const BUFFER_BYTES: usize = MESSAGE_BLOCK_SIZE / 2;
    fn buffer_sizes(&self) -> &[MessageBufferSize] {
        slice::from_ref(&self.0)
    }
    /// Special implementation, because this is half a block
    fn messages_per_buffer(&self) -> [usize; 4] {
        [self.0.message_capacity_per_half_block(), 0, 0, 0]
    }
}
unsafe impl Blocks for OneBlock {
    const NUM_BLOCKS: usize = 1;
    const BUFFER_BYTES: usize = MESSAGE_BLOCK_SIZE;
    fn buffer_sizes(&self) -> &[MessageBufferSize] {
        slice::from_ref(&self.0)
    }
}
unsafe impl Blocks for TwoBlocks {
    const NUM_BLOCKS: usize = 2;
    const BUFFER_BYTES: usize = MESSAGE_BLOCK_SIZE * 2;
    fn buffer_sizes(&self) -> &[MessageBufferSize] {
        &self.0
    }
}
unsafe impl Blocks for ThreeBlocks {
    const NUM_BLOCKS: usize = 3;
    const BUFFER_BYTES: usize = MESSAGE_BLOCK_SIZE * 3;
    fn buffer_sizes(&self) -> &[MessageBufferSize] {
        &self.0
    }
}
unsafe impl Blocks for FourBlocks {
    const NUM_BLOCKS: usize = 4;
    const BUFFER_BYTES: usize = MESSAGE_BLOCK_SIZE * 4;
    fn buffer_sizes(&self) -> &[MessageBufferSize] {
        &self.0
    }
}

/// The maximum number of data bytes in CAN messages that a buffer can store
#[derive(Copy, Clone)]
pub enum MessageBufferSize {
    /// 8 bytes of data per message
    Bytes8,
    /// 16 bytes of data per message
    Bytes16,
    /// 32 bytes of data per message
    Bytes32,
    /// 64 bytes of data per message
    Bytes64,
}

/// Size in bytes of a message buffer that can hold up to 8 bytes of data
pub(crate) const MESSAGE_8_BUFFER_SIZE: usize = 16;
/// Size in bytes of a message buffer that can hold up to 16 bytes of data
pub(crate) const MESSAGE_16_BUFFER_SIZE: usize = 24;
/// Size in bytes of a message buffer that can hold up to 32 bytes of data
pub(crate) const MESSAGE_32_BUFFER_SIZE: usize = 40;
/// Size in bytes of a message buffer that can hold up to 64 bytes of data
pub(crate) const MESSAGE_64_BUFFER_SIZE: usize = 72;

/// Size in bytes of one block of message buffers
pub(crate) const MESSAGE_BLOCK_SIZE: usize = 512;
pub(crate) const MESSAGES_8_PER_BLOCK: usize = 32;
pub(crate) const MESSAGES_16_PER_BLOCK: usize = 21;
pub(crate) const MESSAGES_32_PER_BLOCK: usize = 12;
pub(crate) const MESSAGES_64_PER_BLOCK: usize = 7;

impl MessageBufferSize {
    fn message_capacity_per_block(&self) -> usize {
        match self {
            MessageBufferSize::Bytes8 => MESSAGES_8_PER_BLOCK,
            MessageBufferSize::Bytes16 => MESSAGES_16_PER_BLOCK,
            MessageBufferSize::Bytes32 => MESSAGES_32_PER_BLOCK,
            MessageBufferSize::Bytes64 => MESSAGES_64_PER_BLOCK,
        }
    }
    /// Returns the number of messages a half-block of RAM can store
    ///
    /// Caution: This may not be correct.
    fn message_capacity_per_half_block(&self) -> usize {
        self.message_capacity_per_block() / 2
    }

    pub(crate) fn as_bits(&self) -> Mbdsr {
        match self {
            MessageBufferSize::Bytes8 => Mbdsr::R8Bytes,
            MessageBufferSize::Bytes16 => Mbdsr::R16Bytes,
            MessageBufferSize::Bytes32 => Mbdsr::R32Bytes,
            MessageBufferSize::Bytes64 => Mbdsr::R64Bytes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Blocks, FourBlocks, HalfBlock, MessageBufferSize::*, OneBlock, TwoBlocks};

    #[test]
    fn buffer_sizes_16() {
        assert_eq!([16, 0, 0, 0], HalfBlock(Bytes8).messages_per_buffer());
        assert_eq!([10, 0, 0, 0], HalfBlock(Bytes16).messages_per_buffer());
        assert_eq!([6, 0, 0, 0], HalfBlock(Bytes32).messages_per_buffer());
        assert_eq!([3, 0, 0, 0], HalfBlock(Bytes64).messages_per_buffer());
        assert_eq!(16, HalfBlock(Bytes8).total_messages());
    }

    #[test]
    fn buffer_sizes_32() {
        assert_eq!([32, 0, 0, 0], OneBlock(Bytes8).messages_per_buffer());
        assert_eq!([21, 0, 0, 0], OneBlock(Bytes16).messages_per_buffer());
        assert_eq!([12, 0, 0, 0], OneBlock(Bytes32).messages_per_buffer());
        assert_eq!([7, 0, 0, 0], OneBlock(Bytes64).messages_per_buffer());
        assert_eq!(32, OneBlock(Bytes8).total_messages());
    }

    #[test]
    fn buffer_sizes_64() {
        assert_eq!(
            [32, 12, 0, 0],
            TwoBlocks([Bytes8, Bytes32]).messages_per_buffer()
        );
        assert_eq!(
            [21, 7, 0, 0],
            TwoBlocks([Bytes16, Bytes64]).messages_per_buffer()
        );
        assert_eq!(
            [12, 32, 0, 0],
            TwoBlocks([Bytes32, Bytes8]).messages_per_buffer()
        );
        assert_eq!(
            [7, 21, 0, 0],
            TwoBlocks([Bytes64, Bytes16]).messages_per_buffer()
        );
        assert_eq!(64, TwoBlocks([Bytes8, Bytes8]).total_messages());
    }

    #[test]
    fn buffer_sizes_128() {
        assert_eq!(
            [32, 12, 7, 7],
            FourBlocks([Bytes8, Bytes32, Bytes64, Bytes64]).messages_per_buffer()
        );
        assert_eq!(
            [21, 7, 12, 32],
            FourBlocks([Bytes16, Bytes64, Bytes32, Bytes8]).messages_per_buffer()
        );
        assert_eq!(
            [12, 32, 32, 12],
            FourBlocks([Bytes32, Bytes8, Bytes8, Bytes32]).messages_per_buffer()
        );
        assert_eq!(
            [7, 21, 12, 12],
            FourBlocks([Bytes64, Bytes16, Bytes32, Bytes32]).messages_per_buffer()
        );
        assert_eq!(
            128,
            FourBlocks([Bytes8, Bytes8, Bytes8, Bytes8]).total_messages()
        );
    }
}
