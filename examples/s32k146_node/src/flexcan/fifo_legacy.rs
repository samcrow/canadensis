use self::filter::FilterTable;
use crate::flexcan::buffer::shift_index;
use crate::flexcan::frame::Dlc;
use crate::flexcan::id::{EXTENDED_ID_MASK, Id};
use core::mem::offset_of;
use static_assertions::const_assert_eq;
use vcell::VolatileCell;

pub mod filter;

/// The memory layout of the legacy FIFO block, starting at offset 0x80 from the beginning
/// of the FlexCAN register block
///
/// Notes:
/// * The legacy FIFO can't receive CAN FD frames.
#[repr(C)]
pub(crate) struct LegacyFifo {
    /// Flags, data length code, legacy timestamp
    flags: VolatileCell<u32>,
    /// Message ID
    id: VolatileCell<u32>,
    /// Data from the frame
    data: [u8; 8],
    _reserved: [u8; 80],
    /// 128 filters that determine which messages the FIFO accepts
    // TODO: The actual number of filters depends on CTRL2.RFFN, so some of these may be normal message buffers instead.
    filters: FilterTable,
}

impl LegacyFifo {
    /// Reads and decodes the header
    pub fn read_header(&self) -> Header {
        let flags = self.flags.get();
        let id = self.id.get();

        let timestamp = flags as u16;
        let dlc_raw = ((flags >> 16) & 0xf) as u8;
        let rtr = ((flags >> 20) & 1) == 1;
        let ide = ((flags >> 21) & 1) == 1;
        let srr = ((flags >> 22) & 1) == 1;
        let id_hit = (flags >> 23) as u16;

        let id = if ide {
            Id::extended(id & EXTENDED_ID_MASK)
        } else {
            Id::standard((id >> 18) as u16)
        };

        // THe compiler should optimize out this panic because of the `& 0xf` above
        let dlc = Dlc::from_u8(dlc_raw).unwrap();

        Header {
            id,
            remote: rtr,
            substitute_remote_request: srr,
            dlc,
            timestamp,
            filter_index: id_hit,
        }
    }
    /// Reads the data bytes for this message into a buffer
    ///
    /// Caution: This function does not check the data length code (DLC). The caller must provide
    /// a buffer of the correct length based on the message's DLC. If the buffer is longer than
    /// the message, some of the buffer will have unspecified values.
    ///
    /// This function never copies more than 8 bytes.
    ///
    /// This function returns the actual number of bytes copied.
    pub fn read_data(&self, buffer: &mut [u8]) -> usize {
        let size = buffer.len().min(8);
        // The compiler should optimize this out because of the previous line
        assert!(size <= 8);
        // The layout of the data in the message buffer is unusual. In each group of four bytes,
        // the bytes are reversed.
        for i in 0..size {
            let source_offset = shift_index(i);
            buffer[i] = self.data[source_offset];
        }
        size
    }
}

/// The header read from a legacy FIFIO
#[non_exhaustive]
pub(crate) struct Header {
    /// Message ID
    pub id: Id,
    /// If this frame is a remote transmission request
    pub remote: bool,
    pub substitute_remote_request: bool,
    /// Data length code
    pub dlc: Dlc,
    /// 16-bit timestamp
    pub timestamp: u16,
    /// The index of the filter that matched this message
    pub filter_index: u16,
}

// Size and alignment checks
const_assert_eq!(0x2e0 - 0x80, size_of::<LegacyFifo>());
const_assert_eq!(4, align_of::<LegacyFifo>());
const_assert_eq!(0, offset_of!(LegacyFifo, flags));
const_assert_eq!(4, offset_of!(LegacyFifo, id));
const_assert_eq!(96, offset_of!(LegacyFifo, filters));
