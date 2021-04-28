#![no_std]

extern crate alloc;

extern crate bxcan;
extern crate canadensis;
extern crate nb;

pub mod rx;
pub mod tx;

use core::convert::TryFrom;

/// Converts a Canadensis frame into a bxCAN frame
///
/// # Panics
///
/// This function panics if the provided frame has more than 8 bits of data.
fn uavcan_frame_to_bxcan<I>(frame: &canadensis::Frame<I>) -> bxcan::Frame {
    let bxcan_id = bxcan::ExtendedId::new(frame.id().into()).unwrap();
    let bxcan_data = bxcan::Data::new(frame.data()).expect("Frame data more than 8 bytes");
    bxcan::Frame::new_data(bxcan_id, bxcan_data)
}

/// Converts a bxCAN frame into a Canadensis frame
///
/// This function returns an error if the frame does not have an extended ID, has an ID with an
/// invalid format, or does not have any data.
fn bxcan_frame_to_uavcan<I>(
    frame: &bxcan::Frame,
    deadline: I,
) -> Result<canadensis::Frame<I>, InvalidFrameFormat> {
    let id_bits = match frame.id() {
        bxcan::Id::Extended(extended_id) => extended_id.as_raw(),
        bxcan::Id::Standard(_) => return Err(InvalidFrameFormat),
    };
    let uavcan_id = canadensis::CanId::try_from(id_bits).map_err(|_| InvalidFrameFormat)?;
    let uavcan_data = frame.data().ok_or(InvalidFrameFormat)?;
    Ok(canadensis::Frame::new(
        deadline,
        uavcan_id,
        uavcan_data.as_ref(),
    ))
}

#[derive(Debug)]
struct InvalidFrameFormat;
