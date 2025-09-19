#![allow(dead_code)]

pub mod blocks;
mod buffer;
pub mod config;
pub mod fifo_legacy;
pub mod frame;
pub mod id;
pub(crate) mod svd_generated;

use crate::flexcan::buffer::BufferBlock;
use crate::flexcan::buffer::header::{CODE_TX_ABORT, CODE_TX_DATA};
use blocks::{Blocks, MessageBufferSize};
use buffer::header::Header;
use config::StandardTiming;
use core::cmp::Ordering;
use core::convert::Infallible;
use core::{ptr, slice};
use fifo_legacy::LegacyFifo;
use frame::Frame;
use svd_generated::can::RegisterBlock;

/// Driver for a FlexCAN peripheral
///
/// The type `B` determines the amount of memory that the peripheral provides to store incoming
/// and outgoing messages.
///
/// When creating a driver, provide a value of type `B` that determines how to use the available
/// memory. Each block can split into message buffers with different maximum data lengths.
/// The legacy FIFO or enhanced FIFO (if available) can take the place of some buffers and
/// store received messages.
///
/// Other notes:
/// * Legacy FIFO (MCR.RFEN) and CAN FD (MCR.FDEN) can't both be active at the same time
/// * Each message buffer can hold incoming or outgoing messages, depending on the setup
///
/// # Memory map overview
///
/// <table>
/// <tr><th>Offset in register block</th><th>Default use</th><th>Legacy FIFO</th></tr>
/// <tr><td><code>0x0000..0x0080</code></td><td colspan="2">General registers</td></tr>
/// <tr><td><code>0x0080..0x0090</code></td><td rowspan="3">Message buffer block 0 (32x 16-byte buffers for 8-byte messages)</td><td>Legacy FIFO message access</td></tr>
/// <tr><td><code>0x0090..0x00e0</code></td><td>Reserved</td></tr>
/// <tr><td><code>0x00e0..0x0280</code></td><td>Legacy FIFO filter table (up 104 entries), and/or Message buffer block 1 (up to 25x 8-byte messages)</td></tr>
/// <tr><td><code>0x0280..0x02e0</code></td><td rowspan="2">Message buffer block 1 (32x 8-byte messages)</td><td>Legacy FIFO fiter table (up to 24 entries), and/or Message buffer block 1 (up to 8x 8-byte messages)</td></tr>
/// <tr><td><code>0x02e0..0x0480</code></td><td>Message buffer block 1 (24x 8-byte messages)</td></tr>
/// <tr><td><code>0x0480..0x0680</code></td><td colspan="2">Message buffer block 2 (32x 8-byte messages)</td></tr>
/// <tr><td><code>0x0680..0x0880</code></td><td colspan="2">Message buffer block 3 (32x 8-byte messages)</td></tr>
///
/// <tr><td><code>0x0880..0x0a80</code></td><td colspan="2">128x receive individual mask registers, one for each message buffer</td></tr>
/// <tr><td><code>0x0ae0..0x0c18</code></td><td colspan="2">CAN FD and enhanced FIFO control registers</td></tr>
/// <tr><td><code>0x0c30..0x0e30</code></td><td colspan="2">128x high-resolution timestamps, one for each message buffer</td></tr>
/// <tr><td><code>0x2000..0x2050</code></td><td colspan="2">Enhanced FIFO message access</td></tr>
/// <tr><td><code>0x3000..0x3200</code></td><td colspan="2">512x enhanced FIFO filter elements</td></tr>
/// </table>
///
/// # Interactions between message buffers and FIFOs
///
/// The enhanced FIFO does not overlap with message buffers, so all available message buffers
/// can work at the same time as the enhanced FIFO.
///
/// The legacy FIFO takes place of the first 6 message buffers (byte offsets 0x80..0xe0),
/// and up to 32 message buffers after that depending on the number of legacy FIFO filter elements.
/// Each message buffer uses 16 bytes and each filter element uses 4 bytes.
/// That means the legacy FIFO takes the place of `6 + ceil(filter elements / 4)` message buffers.
///
/// Note: CTRL2.RFFN should include the message buffers that the legacy FIFO supplants.
///
pub struct FlexCan<B> {
    /// Address of the beginning of the register block
    ///
    /// Invariant: alignment is at least REGISTER_BLOCK_MIN_ALIGNMENT
    regs: *mut RegisterBlock,
    buffer_config: B,
}

/// Offset from the beginning of the register block to the first block of message buffers
const BUFFER_BLOCKS_OFFSET: usize = 0x80;
const REGISTER_BLOCK_MIN_ALIGNMENT: usize = 4;

impl<B> FlexCan<B>
where
    B: Blocks,
{
    /// Creates a driver that will access a FlexCAN register block starting at the provided address
    ///
    /// # Safety
    /// * `block` must be the address of a FlexCAN register block, starting with the module
    /// configuration register and ending 16384 bytes later
    /// * Nothing else may read or write to the block while this driver exists
    ///
    /// # Panics
    /// This function panics if block is not aligned to a multiple of 4 bytes.
    pub unsafe fn from_address(block: *mut (), buffer_blocks: B, timing: StandardTiming) -> Self {
        assert_eq!(
            block
                .cast::<u8>()
                .align_offset(REGISTER_BLOCK_MIN_ALIGNMENT),
            0,
            "Block start address not aligned"
        );
        let mut can = FlexCan {
            regs: block.cast(),
            buffer_config: buffer_blocks,
        };
        can.init(timing);
        can
    }

    fn init(&mut self, timing: StandardTiming) {
        // The reference manual says that "asynchronous operation with a 1:1 ratio between peripheral
        // and oscillator clocks is not allowed".
        // Set the protocol engine clock to be the same as the peripheral clock
        unsafe {
            (*self.regs).ctrl1().write(|w| w.clksrc().peripheral());
        }

        self.enter_freeze_mode_no_timeout();
        unsafe {
            init_memory(self.regs, B::BUFFER_BYTES);
        }
        unsafe {
            (*self.regs).fdctrl().write(|w| {
                let buffer_message_sizes = self.buffer_config.buffer_sizes();
                w.mbdsr0()
                    .variant(
                        buffer_message_sizes
                            .get(0)
                            .unwrap_or(&MessageBufferSize::Bytes8)
                            .as_bits(),
                    )
                    .mbdsr1()
                    .variant(
                        buffer_message_sizes
                            .get(1)
                            .unwrap_or(&MessageBufferSize::Bytes8)
                            .as_bits(),
                    )
                    .mbdsr2()
                    .variant(
                        buffer_message_sizes
                            .get(2)
                            .unwrap_or(&MessageBufferSize::Bytes8)
                            .as_bits(),
                    )
                    .mbdsr3()
                    .variant(
                        buffer_message_sizes
                            .get(3)
                            .unwrap_or(&MessageBufferSize::Bytes8)
                            .as_bits(),
                    )
            });

            // Bit rate and things
            (*self.regs).ctrl1().modify(|_, w| {
                w.presdiv()
                    .bits(timing.prescaler.get() - 1)
                    .rjw()
                    .bits(timing.resync_jump_width.bits())
                    .pseg1()
                    .bits(timing.phase_1.bits())
                    .pseg2()
                    .bits(timing.phase_2.bits())
                    .propseg()
                    .bits(timing.propagation.bits())
            });
            // Enable receive FIFO, enable individual receive masking and queues, enable abort
            (*self.regs).mcr().modify(|_, w| {
                w.irmq()
                    .individual_rx_masking_enabled()
                    .rfen()
                    .set_bit()
                    .aen()
                    .abort_enabled()
                    .srxdis()
                    .self_reception_disabled()
            });
            // Basic FIFO receive configuration
            // Set all bits on FIFO receive filter 0 to don't care
            // This lets the receive FIFO get all possible frames.
            (*self.regs).rximr(0).write(|w| w.bits(0));

            // TODO: More configuring
            self.enter_active_mode_no_timeout();
        }
    }

    // Overview of modes:
    // Disable: All clocks off, no access to some registers, minimum power consumption
    // Stop: All clocks off, no access to some registers (Disable and Stop are low-power modes)
    // Freeze: Prescaler off, full access to registers, no transmit/receive
    // Normal modes: Normal, FD active, loopback, listen-only

    fn enter_freeze_mode_no_timeout(&mut self) {
        self.request_freeze_mode();
        while !self.in_freeze_mode() {}
    }

    fn request_freeze_mode(&mut self) {
        unsafe {
            (*self.regs).mcr().modify(|_, w| {
                w.mdis()
                    .flexcan_enabled()
                    .frz()
                    .freeze_mode_enabled()
                    .halt()
                    .halt_enable()
            });
        }
    }

    fn in_freeze_mode(&self) -> bool {
        unsafe { (*self.regs).mcr().read().frzack().is_freeze_mode_yes() }
    }

    fn enter_active_mode_no_timeout(&mut self) {
        self.request_active_mode();
        while !self.in_active_mode() {}
    }

    fn request_active_mode(&mut self) {
        unsafe {
            (*self.regs).mcr().modify(|_, w| {
                w.mdis()
                    .flexcan_enabled()
                    .frz()
                    .freeze_mode_disabled()
                    .halt()
                    .halt_disable()
            });
        }
    }

    fn in_active_mode(&self) -> bool {
        let mcr = unsafe { (*self.regs).mcr().read() };
        mcr.notrdy().bit_is_clear()
    }

    pub fn send(&mut self, frame: &Frame) -> nb::Result<Option<Frame>, Infallible> {
        let new_header = Header {
            timestamp: 0,
            dlc: frame.dlc(),
            rtr: frame.is_remote(),
            ide: frame.id().is_extended(),
            srr: frame.id().is_extended(),
            code: CODE_TX_DATA,
            esi: false,
            brs: false,
            edl: false,
            id: frame.id().bits(),
            priority: 0,
        };
        // To keep this simple, always use message buffer 8.
        unsafe {
            let buffers = buffer_blocks_mut::<B>(self.regs);
            // TODO: Is creating a reference safe?
            let buffer = &mut buffers
                .cast::<BufferBlock>()
                .as_mut()
                .unwrap()
                // Message buffers 0 through 7 are unavailable because the RX FIFO is using that space
                .messages_8_mut()[8];

            // Clear interrupt flag
            (*self.regs).iflag1().write(|w| w.bits(1 << 8));
            let mut removed_frame = None;
            let buffer_header = buffer.read_header();
            if buffer_header.code == CODE_TX_DATA {
                // Buffer already has a frame
                match buffer_header.compare_transmit_priority(&new_header) {
                    Ordering::Less => {
                        // Frame in buffer has lower priority than the current frame
                        // Abort transmission
                        buffer.write_header(&Header {
                            code: CODE_TX_ABORT,
                            ..Header::default()
                        });
                        // Wait for outcome
                        while ((*self.regs).iflag1().read().bits() & (1 << 8)) != 0 {}
                        let aborted_code = buffer.read_header().code;
                        if aborted_code == CODE_TX_ABORT {
                            // Successfully aborted frame
                            // Copy out
                            let id = if buffer_header.ide {
                                id::Id::extended(buffer_header.id)
                            } else {
                                id::Id::standard(buffer_header.id as u16)
                            };
                            removed_frame = Some(if buffer_header.rtr {
                                Frame::new_remote(id, buffer_header.dlc).unwrap()
                            } else {
                                let mut data_buffer = [0u8; 8];
                                let length = usize::from(buffer_header.dlc);
                                buffer.read_data(&mut data_buffer[..length]);
                                Frame::new(id, &data_buffer[..length]).unwrap()
                            });
                        } else {
                            // Frame got sent anyway
                            // Continue with this frame
                        }
                        // Clear interrupt flags
                        (*self.regs).iflag1().write(|w| w.bits(1 << 8));
                    }
                    Ordering::Equal | Ordering::Greater => {
                        // Frame in buffer has equal or higher priority compared to current frame
                        // Keep waiting
                        return Err(nb::Error::WouldBlock);
                    }
                }
            }

            buffer.write_data(frame.data());
            buffer.write_header(&new_header);
            Ok(removed_frame)
        }
    }

    pub fn receive(&mut self) -> Option<Frame> {
        unsafe {
            if (*self.regs).iflag1().read().buf5i().bit_is_clear() {
                // No message
                return None;
            }
            let fifo: *const LegacyFifo = self.regs.byte_add(0x80).cast();
            let header = (*fifo).read_header();
            let mut data_buffer = [0u8; 8];
            (*fifo).read_data(&mut data_buffer[..header.dlc.length()]);

            // Clear flag and advance the FIFO to the next message, if one exists
            (*self.regs)
                .iflag1()
                .write(|w| w.buf5i().clear_bit_by_one());

            Some(if header.remote {
                Frame::new_remote(header.id, header.dlc.as_u8()).unwrap()
            } else {
                Frame::new(header.id, &data_buffer[..header.dlc.length()]).unwrap()
            })
        }
    }
}

/// Returns a unique slice of message buffer blocks
///
/// The length of the slice is equal to `B::NUM_BLOCKS`.
unsafe fn buffer_blocks_mut<B: Blocks>(regs: *mut RegisterBlock) -> *mut [BufferBlock] {
    let ptr: *mut BufferBlock = regs.wrapping_byte_add(BUFFER_BLOCKS_OFFSET).cast();
    // Safety: See `buffer_blocks` above
    unsafe { slice::from_raw_parts_mut(ptr, B::NUM_BLOCKS) }
}

/// Fills the required parts of the register block with zeros
unsafe fn init_memory(registers: *mut RegisterBlock, buffer_bytes: usize) {
    // From section 73.6.1 of the S32K3xx reference manual:
    // Set CTRL2.WRMFRZ
    // Initialize offsets 0x80-0xadf (inclusive), 0xc20-31ff (inclusive)
    // (that includes the buffer blocks)
    // Initialize RXMGMASK, RX14MASK, RX15MASK, RXFGMASK
    unsafe {
        (*registers).ctrl2().write(|w| w.wrmfrz().enable());

        // Zero message buffers
        ptr::write_bytes::<u8>(registers.wrapping_byte_add(0x80).cast(), 0x0, buffer_bytes);

        (*registers).rxmgmask().write(|w| w.mg().bits(0x0));
        (*registers).rx14mask().write(|w| w.rx14m().bits(0x0));
        (*registers).rx15mask().write(|w| w.rx15m().bits(0x0));
        (*registers).rxfgmask().write(|w| w.fgm().bits(0x0));

        (*registers).ctrl2().write(|w| w.wrmfrz().disable());
    }
}
