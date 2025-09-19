use crate::flexcan::id::{EXTENDED_ID_MASK, STANDARD_ID_MASK};
use core::cmp::Ordering;

// Message buffer states for transmitting
pub(crate) const CODE_RX_INACTIVE: u8 = 0b0000;
pub(crate) const CODE_TX_INACTIVE: u8 = 0b1000;
pub(crate) const CODE_EMPTY: u8 = 0b0100;
pub(crate) const CODE_TX_ABORT: u8 = 0b1001;
pub(crate) const CODE_TX_DATA: u8 = 0b1100;
pub(crate) const CODE_TX_ANSWER: u8 = 0b1110;

/// The header of a message buffer
#[cfg_attr(test, derive(PartialEq, Debug))]
#[derive(Default)]
pub(crate) struct Header {
    // 32-bit word 0
    pub(crate) timestamp: u16,
    pub(crate) dlc: u8,
    pub(crate) rtr: bool,
    pub(crate) ide: bool,
    pub(crate) srr: bool,
    pub(crate) code: u8,
    pub(crate) esi: bool,
    pub(crate) brs: bool,
    pub(crate) edl: bool,
    // 32-bit word 1
    /// Message ID, right-aligned
    pub(crate) id: u32,
    pub(crate) priority: u8,
}

impl Header {
    /// Reads a header from the result of a 64-bit little-endian read from the beginning of a
    /// message buffer
    ///
    /// Section 73.6.3 of the S32K3xx reference manual defines the header layout.
    ///
    /// `control_status` and `id` should be in big-endian byte order.
    ///
    pub(crate) fn from_bits(control_status: u32, id: u32) -> Self {
        let ide = ((control_status >> 21) & 1) == 1;
        Header {
            timestamp: control_status as u16,
            dlc: ((control_status >> 16) & 0x0f) as u8,
            rtr: ((control_status >> 20) & 1) == 1,
            ide,
            srr: ((control_status >> 22) & 1) == 1,
            code: ((control_status >> 24) & 0x0f) as u8,
            esi: ((control_status >> 29) & 1) == 1,
            brs: ((control_status >> 30) & 1) == 1,
            edl: ((control_status >> 31) & 1) == 1,
            id: if ide {
                // Extended ID right-aligned in id
                id & EXTENDED_ID_MASK
            } else {
                // Standard ID 18 bits left
                (id >> 18) & STANDARD_ID_MASK
            },
            priority: ((id >> 29) & 0x7) as u8,
        }
    }
    /// Returns a control/status word and message ID word that can be written to the beginning of a
    /// message buffer to set the correct header fields
    pub(crate) fn as_bits(&self) -> (u32, u32) {
        let control_status = u32::from(self.timestamp)
            | u32::from(self.dlc & 0x0f) << 16
            | (self.rtr as u32) << 20
            | (self.ide as u32) << 21
            | (self.srr as u32) << 22
            | u32::from(self.code & 0x0f) << 24
            | (self.esi as u32) << 29
            | (self.brs as u32) << 30
            | (self.edl as u32) << 31;
        let id = if self.ide {
            self.id & EXTENDED_ID_MASK
        } else {
            (self.id & STANDARD_ID_MASK) << 18
        } | u32::from(self.priority & 0x7) << 29;

        (control_status, id)
    }

    pub fn timestamp(&self) -> u16 {
        self.timestamp
    }
    pub fn set_timestamp(&mut self, timestamp: u16) {
        self.timestamp = timestamp;
    }

    /// Calculates the transmit priority of this header relative to another one
    ///
    /// This returns `Ordering::Less` if this header has a lower transmit
    /// priority (higher CAN message ID) than `other`.
    ///
    /// This assumes that local priority is not enabled, so it ignores the priority bits.
    pub(crate) fn compare_transmit_priority(&self, other: &Header) -> Ordering {
        self.mailbox_arbitration_value()
            .cmp(&other.mailbox_arbitration_value())
            .reverse()
    }

    fn mailbox_arbitration_value(&self) -> u32 {
        // Assemble 32-bit priority values following table 5-55 in the S32K1 reference manual
        if self.ide {
            // Extended ID
            ((self.id >> 18) << 21)
                | ((self.srr as u32) << 20)
                | ((self.ide as u32) << 19)
                | ((self.id & 0x3ffff) << 1)
                | (self.rtr as u32)
        } else {
            // Standard ID
            (self.id << 21) | ((self.rtr as u32) << 20)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Header;
    use core::cmp::Ordering;
    use core::fmt::{Debug, Formatter, Result};

    #[test]
    fn test_transmit_priority_standard() {
        let low_id_standard = Header {
            timestamp: 0,
            dlc: 0,
            rtr: false,
            ide: false,
            srr: false,
            code: 0,
            esi: false,
            brs: false,
            edl: false,
            id: 0x10,
            priority: 0,
        };
        assert_eq!(0x02000000, low_id_standard.mailbox_arbitration_value());
        let high_id_standard = Header {
            timestamp: 0,
            dlc: 0,
            rtr: false,
            ide: false,
            srr: false,
            code: 0,
            esi: false,
            brs: false,
            edl: false,
            id: 0x11,
            priority: 0,
        };
        assert_eq!(0x02200000, high_id_standard.mailbox_arbitration_value());
        assert_eq!(
            Ordering::Less,
            high_id_standard.compare_transmit_priority(&low_id_standard)
        );
    }
    #[test]
    fn test_transmit_priority_extended() {
        let low_id_extended = Header {
            timestamp: 0,
            dlc: 0,
            rtr: false,
            ide: true,
            srr: true,
            code: 0,
            esi: false,
            brs: false,
            edl: false,
            id: 0x1e3399f0,
            priority: 0,
        };
        assert_eq!(0xf19f33e0, low_id_extended.mailbox_arbitration_value());
        let high_id_extended = Header {
            timestamp: 0,
            dlc: 0,
            rtr: false,
            ide: true,
            srr: true,
            code: 0,
            esi: false,
            brs: false,
            edl: false,
            id: 0x1e3399f1,
            priority: 0,
        };
        assert_eq!(0xf19f33e2, high_id_extended.mailbox_arbitration_value());
        assert_eq!(
            Ordering::Less,
            high_id_extended.compare_transmit_priority(&low_id_extended)
        );
    }
    #[test]
    fn test_transmit_priority_different_rtr() {
        let rtr_extended = Header {
            timestamp: 0,
            dlc: 0,
            rtr: true,
            ide: true,
            srr: true,
            code: 0,
            esi: false,
            brs: false,
            edl: false,
            id: 0x1e3399f1,
            priority: 0,
        };
        assert_eq!(0xf19f33e3, rtr_extended.mailbox_arbitration_value());
        let non_rtr_extended = Header {
            timestamp: 0,
            dlc: 0,
            rtr: false,
            ide: true,
            srr: true,
            code: 0,
            esi: false,
            brs: false,
            edl: false,
            id: 0x1e3399f1,
            priority: 0,
        };
        assert_eq!(0xf19f33e2, non_rtr_extended.mailbox_arbitration_value());
        assert_eq!(
            Ordering::Greater,
            non_rtr_extended.compare_transmit_priority(&rtr_extended)
        );
    }

    #[test]
    fn read_basic() {
        check_read_header(
            [0x0, 0x0],
            &Header {
                timestamp: 0,
                dlc: 0,
                rtr: false,
                ide: false,
                srr: false,
                code: 0,
                esi: false,
                brs: false,
                edl: false,
                id: 0,
                priority: 0,
            },
        );
        check_read_header(
            [0xe539_13fb, 0x314b_feed],
            &Header {
                timestamp: 0x13fb,
                dlc: 9,
                rtr: true,
                ide: true,
                srr: false,
                code: 5,
                esi: true,
                brs: true,
                edl: true,
                id: 0x114b_feed,
                priority: 1,
            },
        );
        check_read_header(
            [0x0804_8001, 0x9f04_0000],
            &Header {
                timestamp: 0x8001,
                dlc: 4,
                rtr: false,
                ide: false,
                srr: false,
                code: 8,
                esi: false,
                brs: false,
                edl: false,
                id: 0b111_1100_0001,
                priority: 4,
            },
        )
    }
    #[test]
    fn write_basic() {
        check_write_header(
            &Header {
                timestamp: 0,
                dlc: 0,
                rtr: false,
                ide: false,
                srr: false,
                code: 0,
                esi: false,
                brs: false,
                edl: false,
                id: 0,
                priority: 0,
            },
            [0x0, 0x0],
        );
        check_write_header(
            &Header {
                timestamp: 0x8055,
                dlc: 15,
                rtr: false,
                ide: true,
                srr: false,
                code: 4,
                esi: true,
                brs: false,
                edl: false,
                id: 0x3234_abdc,
                priority: 7,
            },
            [0x242f_8055, 0xf234_abdc],
        );
        check_write_header(
            &Header {
                timestamp: 9,
                dlc: 1,
                rtr: false,
                ide: false,
                srr: false,
                code: 0,
                esi: false,
                brs: false,
                edl: false,
                id: 0x400,
                priority: 2,
            },
            [0x0001_0009, 0x5000_0000],
        );
    }

    fn check_write_header(header: &Header, expected_words: [u32; 2]) {
        let (actual_control_status, actual_id) = header.as_bits();
        assert_eq!(
            HexDebugPair(expected_words),
            HexDebugPair([actual_control_status, actual_id]),
        );

        struct HexDebug(u32);
        impl Debug for HexDebug {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{:#010x}", self.0)
            }
        }
        #[derive(PartialEq)]
        struct HexDebugPair([u32; 2]);
        impl Debug for HexDebugPair {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                f.debug_list()
                    .entry(&HexDebug(self.0[0]))
                    .entry(&HexDebug(self.0[1]))
                    .finish()
            }
        }
    }

    fn check_read_header(words: [u32; 2], expected_header: &Header) {
        let header = Header::from_bits(words[0], words[1]);
        assert_eq!(expected_header, &header);
    }
}
