use canadensis_core::session::SessionDynamicMap;
use canadensis_linux::SystemClock;
use canadensis_udp::driver::StdUdpSocket;
use canadensis_udp::{UdpNodeId, UdpReceiver, UdpSessionData, UdpTransferId};
use log::LevelFilter;
use simplelog::{ColorChoice, TermLogger, TerminalMode};
use std::sync::Once;

#[allow(dead_code)]
pub type TestUdpReceiver<const MTU: usize> = UdpReceiver<
    SystemClock,
    SessionDynamicMap<UdpNodeId, UdpTransferId, UdpSessionData>,
    StdUdpSocket,
    MTU,
>;

pub fn init_test_logging() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let _ = TermLogger::init(
            LevelFilter::Trace,
            Default::default(),
            TerminalMode::Stderr,
            ColorChoice::Auto,
        );
    })
}
