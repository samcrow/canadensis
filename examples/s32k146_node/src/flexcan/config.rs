use crate::flexcan::blocks::MESSAGE_BLOCK_SIZE;
use crate::flexcan::blocks::MessageBufferSize;
use crate::flexcan::blocks::{
    Blocks, MESSAGE_8_BUFFER_SIZE, MESSAGE_16_BUFFER_SIZE, MESSAGE_32_BUFFER_SIZE,
    MESSAGE_64_BUFFER_SIZE,
};
use core::num::NonZeroU8;
use core::ops::Range;

/// Offset from the beginning of the FlexCAN register block to the first message buffer
/// and the beginning of the legacy FIFO space
const MESSAGE_BUFFER_START_OFFSET: usize = 0x80;
/// Number of bytes that the legacy FIFO flags, ID, message data, and reserved space
/// (before the first filter) take up
const LEGACY_FIFO_FIXED_SIZE: usize = 0x60;

/// Type-level configuration for the features that a specific FlexCAN peripheral provides
pub trait Features {
    /// Blocks of memory available for message buffers
    type BufferBlocks: Blocks;
    /// CAN FD support
    const CAN_FD: bool;
    /// Enhanced receive FIFO with support for CAN FD
    const ENHANCED_RX_FIFO: bool;
}

/// A configuration of a FlexCAN peripheral
pub struct Config<F: Features> {
    pub blocks: F::BufferBlocks,
    pub mode: Mode,
}

pub struct Config2 {
    legacy_fifo: Option<Range<usize>>,
    enhanced_fifo: Option<Range<usize>>,
    blocks: &'static [Block],
    message_buffers: &'static [Range<usize>],
}

pub struct Block {
    message_size: MessageBufferSize,
    message_buffers: &'static [Range<usize>],
}

pub enum Mode {
    Can20 {
        legacy_fifo: Option<LegacyFifoConfig>,
    },
    CanFd {
        enhanced_fifo: Option<EnhancedFifoConfig>,
    },
}

pub struct LegacyFifoConfig {
    /// Number of filters to enable
    filters: u8,
}

pub struct EnhancedFifoConfig {}

/// Timing and bit rate settings for CAN
pub struct StandardTiming {
    /// Clock division between the protocol engine clock and the serial clock (which measures
    /// time quanta)
    ///
    /// CTRL1.PRESDIV is one less than this value.
    pub prescaler: NonZeroU8,
    pub resync_jump_width: ResyncJumpWidth,
    /// Length of the propagation segment in time quanta
    pub propagation: SegmentLength,
    /// Length of phase segment 1 in time quanta
    pub phase_1: SegmentLength,
    /// Length of phase segment 2 in time quanta
    pub phase_2: PhaseSegment2Length,
}

pub enum ResyncJumpWidth {
    Width1,
    Width2,
    Width3,
    Width4,
}

impl ResyncJumpWidth {
    pub(crate) fn bits(&self) -> u8 {
        match self {
            ResyncJumpWidth::Width1 => 0,
            ResyncJumpWidth::Width2 => 1,
            ResyncJumpWidth::Width3 => 2,
            ResyncJumpWidth::Width4 => 3,
        }
    }
}

/// Length of phase segment 1 or propagation segment in time quanta
pub enum SegmentLength {
    Length1,
    Length2,
    Length3,
    Length4,
    Length5,
    Length6,
    Length7,
    Length8,
}

impl SegmentLength {
    pub(crate) fn bits(&self) -> u8 {
        match self {
            SegmentLength::Length1 => 0,
            SegmentLength::Length2 => 1,
            SegmentLength::Length3 => 2,
            SegmentLength::Length4 => 3,
            SegmentLength::Length5 => 4,
            SegmentLength::Length6 => 5,
            SegmentLength::Length7 => 6,
            SegmentLength::Length8 => 7,
        }
    }
}

/// Length of phase segment 2 in time quanta
pub enum PhaseSegment2Length {
    Length2,
    Length3,
    Length4,
    Length5,
    Length6,
    Length7,
    Length8,
}

impl PhaseSegment2Length {
    pub(crate) fn bits(&self) -> u8 {
        match self {
            PhaseSegment2Length::Length2 => 1,
            PhaseSegment2Length::Length3 => 2,
            PhaseSegment2Length::Length4 => 3,
            PhaseSegment2Length::Length5 => 4,
            PhaseSegment2Length::Length6 => 5,
            PhaseSegment2Length::Length7 => 6,
            PhaseSegment2Length::Length8 => 7,
        }
    }
}

/// Timing and bit rate settings for CAN FD
pub struct FdTiming {
    // TODO
}

/// Returns a range of offsets, in bytes relative to the beginning of the FlexCAN register block,
/// that the legacy receive FIFO uses
const fn legacy_fifo_space(filters: usize) -> Range<usize> {
    let size = LEGACY_FIFO_FIXED_SIZE + 4 * filters;
    MESSAGE_BUFFER_START_OFFSET..MESSAGE_BUFFER_START_OFFSET + size
}

/// Returns the range of offsets, in bytes relative to the beginning of the FlexCAN register block,
/// that one message buffer with capacity for 8 data bytes uses
const fn message_buffer_8_byte_space(index: usize) -> Range<usize> {
    message_buffer_space(index, MESSAGE_8_BUFFER_SIZE)
}

/// Returns the range of offsets, in bytes relative to the beginning of the FlexCAN register block,
/// that one message buffer with capacity for 16 data bytes uses
const fn message_buffer_16_byte_space(index: usize) -> Range<usize> {
    message_buffer_space(index, MESSAGE_16_BUFFER_SIZE)
}

/// Returns the range of offsets, in bytes relative to the beginning of the FlexCAN register block,
/// that one message buffer with capacity for 32 data bytes uses
const fn message_buffer_32_byte_space(index: usize) -> Range<usize> {
    message_buffer_space(index, MESSAGE_32_BUFFER_SIZE)
}

/// Returns the range of offsets, in bytes relative to the beginning of the FlexCAN register block,
/// that one message buffer with capacity for 64 data bytes uses
const fn message_buffer_64_byte_space(index: usize) -> Range<usize> {
    message_buffer_space(index, MESSAGE_64_BUFFER_SIZE)
}

const fn message_buffer_space(index: usize, buffer_size: usize) -> Range<usize> {
    let messages_per_block = MESSAGE_BLOCK_SIZE / buffer_size;
    let block = index / messages_per_block;
    let index_in_block = index % messages_per_block;
    let start =
        MESSAGE_BUFFER_START_OFFSET + MESSAGE_BLOCK_SIZE * block + buffer_size * index_in_block;
    start..start + buffer_size
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn message_buffers_8() {
        assert_eq!(0x80..0x90, message_buffer_8_byte_space(0));
        assert_eq!(0x90..0xa0, message_buffer_8_byte_space(1));
        assert_eq!(0x270..0x280, message_buffer_8_byte_space(31));
        assert_eq!(0x280..0x290, message_buffer_8_byte_space(32));
        assert_eq!(0x670..0x680, message_buffer_8_byte_space(95));
    }

    #[test]
    fn message_buffers_16() {
        assert_eq!(0x80..0x98, message_buffer_16_byte_space(0));
        assert_eq!(0x98..0xb0, message_buffer_16_byte_space(1));
        assert_eq!(0x260..0x278, message_buffer_16_byte_space(20));
        // Gap at end of block 0
        assert_eq!(0x280..0x298, message_buffer_16_byte_space(21));
        assert_eq!(0x298..0x2b0, message_buffer_16_byte_space(22));
        assert_eq!(0x460..0x478, message_buffer_16_byte_space(41));
        // Gap at end of block 1
        assert_eq!(0x480..0x498, message_buffer_16_byte_space(42));
        assert_eq!(0x660..0x678, message_buffer_16_byte_space(62));
        // Gap at end of block 2
        assert_eq!(0x680..0x698, message_buffer_16_byte_space(63));
    }

    #[test]
    fn message_buffers_32() {
        assert_eq!(0x80..0xa8, message_buffer_32_byte_space(0));
        assert_eq!(0xa8..0xd0, message_buffer_32_byte_space(1));
        assert_eq!(0x238..0x260, message_buffer_32_byte_space(11));
        // Gap at end of block 0
        assert_eq!(0x280..0x2a8, message_buffer_32_byte_space(12));
        assert_eq!(0x438..0x460, message_buffer_32_byte_space(23));
        // Gap at end of block 1
        assert_eq!(0x480..0x4a8, message_buffer_32_byte_space(24));
        assert_eq!(0x638..0x660, message_buffer_32_byte_space(35));
        // Gap at end of block 2
        assert_eq!(0x680..0x6a8, message_buffer_32_byte_space(36));
    }

    #[test]
    fn message_buffers_64() {
        assert_eq!(0x80..0xc8, message_buffer_64_byte_space(0));
        assert_eq!(0xc8..0x110, message_buffer_64_byte_space(1));
        assert_eq!(0x230..0x278, message_buffer_64_byte_space(6));
        // Gap at end of block 0
        assert_eq!(0x280..0x2c8, message_buffer_64_byte_space(7));
        assert_eq!(0x430..0x478, message_buffer_64_byte_space(13));
        // Gap at end of block 1
        assert_eq!(0x480..0x4c8, message_buffer_64_byte_space(14));
        assert_eq!(0x630..0x678, message_buffer_64_byte_space(20));
        // Gap at end of block 2
        assert_eq!(0x680..0x6c8, message_buffer_64_byte_space(21));
    }
}
