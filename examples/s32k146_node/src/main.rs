#![no_std]
#![no_main]

extern crate alloc;

extern crate canadensis;
extern crate canadensis_can;
extern crate canadensis_data_types;
extern crate embedded_alloc;
extern crate heapless;
extern crate panic_halt;
extern crate s32k146_pac as s32k146;

mod flexcan;
// svd_generated requires this
pub(crate) use crate::flexcan::svd_generated::generic::{self, *};

use alloc::vec::Vec;
use core::convert::{Infallible, TryFrom};
use core::num::NonZeroU8;

use canadensis::core::subscription::Subscription;
use canadensis::core::time::{Clock, Microseconds32};
use canadensis::core::transfer::{MessageTransfer, ServiceTransfer};
use canadensis::core::transport::Transport;
use canadensis::core::OutOfMemoryError;
use canadensis::node::data_types::{GetInfoResponse, Version};
use canadensis::node::{BasicNode, CoreNode};
use canadensis::Node as _;
use canadensis::{nb, ResponseToken, TransferHandler};
use canadensis_can::driver::{ReceiveDriver, TransmitDriver};
use canadensis_can::queue::{ArrayQueue, SingleQueueDriver};
use canadensis_can::{CanNodeId, CanReceiver, CanTransferIdTracker, CanTransmitter, Frame, Mtu};
use canadensis_data_types::uavcan::node::health_1_0::Health;

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use embedded_alloc::LlffHeap as Heap;
use flexcan::blocks::{MessageBufferSize, OneBlock};
use flexcan::config::{PhaseSegment2Length, ResyncJumpWidth, SegmentLength, StandardTiming};
use flexcan::id::Id;
use flexcan::FlexCan;
use s32k146::{Peripherals, CAN0, SIM};

#[global_allocator]
static HEAP: Heap = Heap::empty();

const MAX_PUBLISH_TOPICS: usize = 4;
const MAX_REQUEST_SERVICES: usize = 4;
const TX_QUEUE_SIZE: usize = 32;
type Driver = SingleQueueDriver<TimerClock, ArrayQueue<TX_QUEUE_SIZE>, FlexCanDriver>;
type Node = BasicNode<
    CoreNode<
        TimerClock,
        CanTransmitter<TimerClock, Driver>,
        CanReceiver<TimerClock, Driver>,
        CanTransferIdTracker,
        Driver,
        MAX_PUBLISH_TOPICS,
        MAX_REQUEST_SERVICES,
    >,
>;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap_or_else(|| loop {});
    let dp = Peripherals::take().unwrap_or_else(|| loop {});
    // Disable watchdog
    dp.WDOG.cnt.write(|w| unsafe { w.bits(0xd928c520) });
    dp.WDOG.toval.write(|w| unsafe { w.bits(0xffff) });
    dp.WDOG.cs.write(|w| w.en().clear_bit());

    // Begin clock configuration
    // Enable SOSCDIV2, which the FlexCAN peripherals need when CTRL1.CLKSRC is 0
    // 001 = divide by 1
    dp.SCG.soscdiv.write(|w| w.soscdiv2()._001());
    // Configure and enable external clock
    // The UCANS32K1SIC board has an 8 MHz crystal connected to PTB6 and PTB7
    // Range needs to be 0b11 (high) to work with the PLL.
    dp.SCG.sosccfg.write(|w| w.erefs().set_bit().range()._11());
    dp.SCG.sosccsr.write(|w| w.soscen().set_bit());
    // Wait for clock to be valid
    while dp.SCG.sosccsr.read().soscvld().bit_is_clear() {}
    // Configure PLL to double the 8 MHz from the external crystal to 16 MHz
    // VCO_CLK should be 32 MHz = 8 MHz / (PREDIV + 1) * (MULT + 16)
    // PREDIV = 3, MULT = 0
    dp.SCG
        .spllcfg
        .write(|w| unsafe { w.prediv().bits(3).mult().bits(0) });
    // Enable PLL
    dp.SCG.spllcsr.write(|w| w.spllen().set_bit());
    // Wait for PLL output to be valid
    while dp.SCG.spllcsr.read().spllvld().bit_is_clear() {}

    // Switch the system clock following the steps in section 28.1.4 of the reference manual
    // Set all sources to reset
    dp.RCM.srie.write(|w| w);
    // Set all sources to interrupt
    // TODO: SVD is missing SRE.CMU_LOC (bit 4)
    dp.RCM.srie.write(|w| unsafe { w.bits(0x00002ffc) });
    // Wait 10 LPO (low-power oscillator) cycles at 128 kHz
    // Assume the core is initially running at 48 MHz from the fast internal RC oscillator
    cortex_m::asm::delay(10 * 48_000_000 / 128_000);
    // Switch clock source to system oscillator (from the external crystal)
    // Core, bus, and slow clocks are all divided by 1
    dp.SCG.rccr.write(|w| {
        w.scs()
            ._0110() // 0110 = system PLL
            .divcore()
            ._0000()
            .divbus()
            ._0000()
            .divslow()
            ._0000()
    });
    // Wait for the change to take effect
    while dp.SCG.spllcsr.read().spllsel().bit_is_clear() {}
    // Set all sources to reset
    dp.RCM.srie.write(|w| w);
    // End clock configuration
    // Now we're running on the external crystal. SYS_CLK and most other clocks are 16 MHz.
    // Performance is not ideal, but this works for now.
    let mut delay = Delay::new(cp.SYST, 16_000_000);

    // Begin LED blink setup
    // Enable clocks to GPIO port D
    dp.PCC.pcc_portd.write(|w| w.cgc().set_bit());

    // GPIO: PTD16 controls the green LED
    let gpiod = dp.PTD;
    // Output
    gpiod.pddr.write(|w| unsafe { w.pdd().bits(1 << 16) });
    dp.PORTD.pcr16.write(|w| w.mux()._001());
    // End LED blink setup

    // Begin CAN setup
    // Enable clocks to CAN0
    // Module clock, CHI clock, and PE clock are all 8 MHz
    dp.PCC.pcc_flex_can0.write(|w| w.cgc().set_bit());

    // The UCANS32K1SIC board has transceivers connected to FlexCAN0 and FlexCAN1.
    // Enable clocks to GPIO ports A and E
    dp.PCC.pcc_porta.write(|w| w.cgc().set_bit());
    dp.PCC.pcc_porte.write(|w| w.cgc().set_bit());
    // CAN0_TX: PTE5 alternate function 5 (0b101)
    dp.PORTE.pcr5.write(|w| w.mux()._101());
    // CAN0_RX: PTE4 alternate function 5 (0b101)
    dp.PORTE.pcr4.write(|w| w.mux()._101());
    // CAN1_TX: PTA13 alternate function 3 (0b011)
    dp.PORTA.pcr13.write(|w| w.mux()._011());
    // CAN1_RX: PTA12 alternate function 3 (0b011)
    dp.PORTA.pcr12.write(|w| w.mux()._011());

    // Extra CAN transceiver pins
    // Outputs are initially low
    // CAN0_EN: PTA10 GPIO output
    dp.PTA.pcor.write(|w| unsafe { w.ptco().bits(1 << 10) });
    dp.PORTA.pcr10.write(|w| w.mux()._001());
    dp.PTA
        .pddr
        .modify(|r, w| unsafe { w.pdd().bits(r.pdd().bits() | 1 << 10) });
    // CAN0_STB: PTE11 GPIO output
    dp.PTE.pcor.write(|w| unsafe { w.ptco().bits(1 << 11) });
    dp.PORTE.pcr11.write(|w| w.mux()._001());
    dp.PTE
        .pddr
        .modify(|r, w| unsafe { w.pdd().bits(r.pdd().bits() | 1 << 11) });

    // Wait for transceiver to reset
    delay.delay_ms(1);
    // Set STB and EN high
    dp.PTA.psor.write(|w| unsafe { w.bits(1 << 10) });
    dp.PTE.psor.write(|w| unsafe { w.bits(1 << 11) });
    // Wait for transceiver to start up
    delay.delay_ms(10);

    // With a 16 MHz protocol engine clock, this generates 1 megabit/second.
    // http://www.bittiming.can-wiki.info
    let can_timing = StandardTiming {
        prescaler: NonZeroU8::new(1).unwrap(),
        resync_jump_width: ResyncJumpWidth::Width1,
        propagation: SegmentLength::Length5,
        phase_1: SegmentLength::Length8,
        phase_2: PhaseSegment2Length::Length2,
    };

    let can = unsafe {
        FlexCan::from_address(
            CAN0::ptr() as *mut (),
            OneBlock(MessageBufferSize::Bytes8),
            can_timing,
        )
    };
    // End CAN setup

    // Use all of SRAM_L for the heap
    // The linker does not use SRAM_L.
    unsafe { HEAP.init(0x1fff_0000, 65536) }
    // End heap

    // Cyphal
    let node_info = GetInfoResponse {
        protocol_version: Version { major: 1, minor: 0 },
        hardware_version: Version { major: 1, minor: 0 },
        software_version: Version { major: 0, minor: 1 },
        software_vcs_revision_id: 0,
        unique_id: get_unique_id(&dp.SIM),
        name: heapless::Vec::from_slice(b"canadensis_s32k_demo").unwrap(),
        software_image_crc: Default::default(),
        certificate_of_authenticity: Default::default(),
    };

    let node_id = CanNodeId::try_from(32u8).unwrap();
    // TODO: Real timer
    let clock = TimerClock;
    let transmitter = CanTransmitter::new(Mtu::Can8);
    let receiver = CanReceiver::new(node_id);
    let driver = FlexCanDriver(can);
    let queue_driver = SingleQueueDriver::new(ArrayQueue::new(), driver);
    let node = CoreNode::new(clock, node_id, transmitter, receiver, queue_driver);
    let mut node: Node = BasicNode::new(node, node_info).unwrap();
    node.set_health(Health {
        value: Health::CAUTION,
    });
    // End Cyphal

    let mut counter = 0u16;
    // Set SysTick to wrap around every second
    let mut syst = delay.free();
    syst.set_reload(16_000_000 - 1);
    syst.clear_current();
    syst.enable_counter();

    loop {
        if syst.has_wrapped() {
            // Toggle output
            gpiod.ptor.write(|w| unsafe { w.ptto().bits(1 << 16) });
            counter = counter.wrapping_add(1);
            let _ = node.run_per_second_tasks();
        }

        let _ = node.receive(&mut EmptyHandler);
        // Flush the queue of outgoing frames
        let _ = node.node_mut().flush();
    }
}

fn get_unique_id(sim: &SIM) -> [u8; 16] {
    let mut bytes = [0u8; 16];
    bytes[0..4].copy_from_slice(&sim.uidl.read().bits().to_ne_bytes());
    bytes[4..8].copy_from_slice(&sim.uidml.read().bits().to_ne_bytes());
    bytes[8..12].copy_from_slice(&sim.uidmh.read().bits().to_ne_bytes());
    bytes[12..16].copy_from_slice(&sim.uidh.read().bits().to_ne_bytes());
    bytes
}

struct TimerClock;

impl Clock for TimerClock {
    fn now(&mut self) -> Microseconds32 {
        // TODO
        Microseconds32::from_ticks(0)
    }
}

struct FlexCanDriver(FlexCan<OneBlock>);

impl TransmitDriver<TimerClock> for FlexCanDriver {
    type Error = Infallible;

    fn try_reserve(&mut self, _frames: usize) -> Result<(), OutOfMemoryError> {
        // TODO
        Ok(())
    }

    fn transmit(
        &mut self,
        frame: Frame,
        _clock: &mut TimerClock,
    ) -> canadensis::nb::Result<Option<Frame>, Self::Error> {
        let frame =
            flexcan::frame::Frame::new(Id::extended(u32::from(frame.id())), frame.data()).unwrap();
        self.0.send(&frame).map(|returned_frame| {
            returned_frame.and_then(|driver_frame| {
                if driver_frame.id().is_extended() {
                    Some(canadensis_can::Frame::new(
                        Microseconds32::from_ticks(0),
                        canadensis_can::CanId::try_from(driver_frame.id().bits()).unwrap(),
                        driver_frame.data(),
                    ))
                } else {
                    // Ignore standard-ID frames
                    None
                }
            })
        })
    }

    fn flush(&mut self, _clock: &mut TimerClock) -> canadensis::nb::Result<(), Self::Error> {
        // Nothing to do
        Ok(())
    }
}

impl ReceiveDriver<TimerClock> for FlexCanDriver {
    type Error = Infallible;

    fn receive(&mut self, clock: &mut TimerClock) -> canadensis::nb::Result<Frame, Self::Error> {
        let now = clock.now();
        match self.0.receive() {
            Some(frame) => {
                if !frame.id().is_extended() {
                    // Cyphal ignores frames with non-extended IDs
                    return Err(nb::Error::WouldBlock);
                }
                let extended_id_bits = frame.id().bits();
                let converted_frame = canadensis_can::Frame::new(
                    now,
                    canadensis_can::CanId::try_from(extended_id_bits).unwrap(),
                    frame.data(),
                );
                Ok(converted_frame)
            }
            None => Err(nb::Error::WouldBlock),
        }
    }

    fn apply_filters<S>(&mut self, _local_node: Option<CanNodeId>, _subscriptions: S)
    where
        S: IntoIterator<Item = Subscription>,
    {
        // Not implemented, this driver always accepts all frames
    }

    fn apply_accept_all(&mut self) {
        // Not implemented, this driver always accepts all frames
    }
}

struct EmptyHandler;

impl<T: Transport> TransferHandler<T> for EmptyHandler {
    fn handle_message<N>(&mut self, _node: &mut N, _transfer: &MessageTransfer<Vec<u8>, T>) -> bool
    where
        N: canadensis::Node<Transport = T>,
    {
        false
    }

    fn handle_request<N>(
        &mut self,
        _node: &mut N,
        _token: ResponseToken<T>,
        _transfer: &ServiceTransfer<Vec<u8>, T>,
    ) -> bool
    where
        N: canadensis::Node<Transport = T>,
    {
        false
    }

    fn handle_response<N>(&mut self, _node: &mut N, _transfer: &ServiceTransfer<Vec<u8>, T>) -> bool
    where
        N: canadensis::Node<Transport = T>,
    {
        false
    }
}
