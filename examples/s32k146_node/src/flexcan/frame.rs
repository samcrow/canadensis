use crate::flexcan::id::Id;
use static_assertions::const_assert_eq;

/// Data length code values allowed for CAN and CAN FD frames
#[derive(Copy, Clone, Debug)]
pub(crate) struct Dlc(DlcInner);

/// Data length code values allowed for CAN and CAN FD frames
///
/// Using this, instead of a u8, lets the compiler optimize out some panics.
#[derive(Copy, Clone, Debug)]
enum DlcInner {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
}

impl Dlc {
    pub(crate) fn length(&self) -> usize {
        match self.0 {
            DlcInner::D0 => 0,
            DlcInner::D1 => 1,
            DlcInner::D2 => 2,
            DlcInner::D3 => 3,
            DlcInner::D4 => 4,
            DlcInner::D5 => 5,
            DlcInner::D6 => 6,
            DlcInner::D7 => 7,
            DlcInner::D8 => 8,
            DlcInner::D9 => 12,
            DlcInner::D10 => 16,
            DlcInner::D11 => 20,
            DlcInner::D12 => 24,
            DlcInner::D13 => 32,
            DlcInner::D14 => 48,
            DlcInner::D15 => 64,
        }
    }
    fn from_length(length: usize) -> Option<Self> {
        use DlcInner::*;
        Some(Dlc(match length {
            0 => D0,
            1 => D1,
            2 => D2,
            3 => D3,
            4 => D4,
            5 => D5,
            6 => D6,
            7 => D7,
            8 => D8,
            12 => D9,
            16 => D10,
            20 => D11,
            24 => D12,
            32 => D13,
            48 => D14,
            64 => D15,
            _ => return None,
        }))
    }
    pub(crate) fn as_u8(&self) -> u8 {
        match self.0 {
            DlcInner::D0 => 0,
            DlcInner::D1 => 1,
            DlcInner::D2 => 2,
            DlcInner::D3 => 3,
            DlcInner::D4 => 4,
            DlcInner::D5 => 5,
            DlcInner::D6 => 6,
            DlcInner::D7 => 7,
            DlcInner::D8 => 8,
            DlcInner::D9 => 9,
            DlcInner::D10 => 10,
            DlcInner::D11 => 11,
            DlcInner::D12 => 12,
            DlcInner::D13 => 13,
            DlcInner::D14 => 14,
            DlcInner::D15 => 15,
        }
    }
    pub(crate) fn from_u8(dlc: u8) -> Option<Self> {
        use DlcInner::*;
        Some(Dlc(match dlc {
            0 => D0,
            1 => D1,
            2 => D2,
            3 => D3,
            4 => D4,
            5 => D5,
            6 => D6,
            7 => D7,
            8 => D8,
            9 => D9,
            10 => D10,
            11 => D11,
            12 => D12,
            13 => D13,
            14 => D14,
            15 => D15,
            _ => return None,
        }))
    }
}

/// A CAN FD frame (also compatible with CAN 2.0)
#[derive(Debug)]
pub struct Frame {
    id: Id,
    dlc: Dlc,
    remote: bool,
    data: [u8; 64],
}

const_assert_eq!(72, size_of::<Frame>());

impl Frame {
    pub fn new(id: Id, data: &[u8]) -> Option<Self> {
        let dlc = Dlc::from_length(data.len())?;
        let mut copy_data = [0u8; 64];
        copy_data[..data.len()].copy_from_slice(data);
        Some(Frame {
            id,
            dlc,
            remote: false,
            data: copy_data,
        })
    }
    pub(crate) fn new_remote(id: Id, dlc: u8) -> Option<Self> {
        let dlc = Dlc::from_u8(dlc)?;
        Some(Frame {
            id,
            dlc,
            remote: true,
            data: [0; 64],
        })
    }
    pub(crate) fn is_extended(&self) -> bool {
        self.id.is_extended()
    }
    pub(crate) fn is_remote(&self) -> bool {
        self.remote
    }
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn dlc(&self) -> u8 {
        self.dlc.as_u8()
    }
    pub fn data(&self) -> &[u8] {
        &self.data[..self.dlc.length()]
    }
}
