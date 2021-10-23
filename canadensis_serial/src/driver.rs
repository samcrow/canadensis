//! Serial driver definitions

use canadensis_core::nb;

/// A driver that can send bytes
pub trait TransmitDriver {
    type Error;
    /// Attempts to send a byte without blocking
    fn send_byte(&mut self, byte: u8) -> nb::Result<(), Self::Error>;
}

/// A driver that can receive bytes
pub trait ReceiveDriver {
    type Error;
    /// Attempts to receive a byte without blocking
    fn receive_byte(&mut self) -> nb::Result<u8, Self::Error>;
}
