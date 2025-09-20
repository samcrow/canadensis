#[doc = "Register `ESR1` reader"]
pub type R = crate::R<Esr1Spec>;
#[doc = "Register `ESR1` writer"]
pub type W = crate::W<Esr1Spec>;
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errint {
    #[doc = "0: No such occurrence."]
    Disable = 0,
    #[doc = "1: Indicates setting of any error bit in the Error and Status register."]
    Enable = 1,
}
impl From<Errint> for bool {
    #[inline(always)]
    fn from(variant: Errint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRINT` reader - Error Interrupt"]
pub type ErrintR = crate::BitReader<Errint>;
impl ErrintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errint {
        match self.bits {
            false => Errint::Disable,
            true => Errint::Enable,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Errint::Disable
    }
    #[doc = "Indicates setting of any error bit in the Error and Status register."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Errint::Enable
    }
}
#[doc = "Field `ERRINT` writer - Error Interrupt"]
pub type ErrintW<'a, REG> = crate::BitWriter1C<'a, REG, Errint>;
impl<'a, REG> ErrintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Errint::Disable)
    }
    #[doc = "Indicates setting of any error bit in the Error and Status register."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Errint::Enable)
    }
}
#[doc = "Bus Off Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boffint {
    #[doc = "0: No such occurrence."]
    Disable = 0,
    #[doc = "1: FlexCAN module entered Bus Off state."]
    Enable = 1,
}
impl From<Boffint> for bool {
    #[inline(always)]
    fn from(variant: Boffint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFINT` reader - Bus Off Interrupt"]
pub type BoffintR = crate::BitReader<Boffint>;
impl BoffintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boffint {
        match self.bits {
            false => Boffint::Disable,
            true => Boffint::Enable,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Boffint::Disable
    }
    #[doc = "FlexCAN module entered Bus Off state."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Boffint::Enable
    }
}
#[doc = "Field `BOFFINT` writer - Bus Off Interrupt"]
pub type BoffintW<'a, REG> = crate::BitWriter1C<'a, REG, Boffint>;
impl<'a, REG> BoffintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Boffint::Disable)
    }
    #[doc = "FlexCAN module entered Bus Off state."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Boffint::Enable)
    }
}
#[doc = "FlexCAN In Reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rx {
    #[doc = "0: FlexCAN is not receiving a message."]
    Disable = 0,
    #[doc = "1: FlexCAN is receiving a message."]
    Enable = 1,
}
impl From<Rx> for bool {
    #[inline(always)]
    fn from(variant: Rx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RX` reader - FlexCAN In Reception"]
pub type RxR = crate::BitReader<Rx>;
impl RxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rx {
        match self.bits {
            false => Rx::Disable,
            true => Rx::Enable,
        }
    }
    #[doc = "FlexCAN is not receiving a message."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rx::Disable
    }
    #[doc = "FlexCAN is receiving a message."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rx::Enable
    }
}
#[doc = "Fault Confinement State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fltconf {
    #[doc = "0: Error Active"]
    ErrorActive = 0,
    #[doc = "1: Error Passive"]
    ErrorPassive = 1,
    #[doc = "2: Bus Off"]
    BusOff = 2,
}
impl From<Fltconf> for u8 {
    #[inline(always)]
    fn from(variant: Fltconf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fltconf {
    type Ux = u8;
}
impl crate::IsEnum for Fltconf {}
#[doc = "Field `FLTCONF` reader - Fault Confinement State"]
pub type FltconfR = crate::FieldReader<Fltconf>;
impl FltconfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fltconf> {
        match self.bits {
            0 => Some(Fltconf::ErrorActive),
            1 => Some(Fltconf::ErrorPassive),
            2 => Some(Fltconf::BusOff),
            _ => None,
        }
    }
    #[doc = "Error Active"]
    #[inline(always)]
    pub fn is_error_active(&self) -> bool {
        *self == Fltconf::ErrorActive
    }
    #[doc = "Error Passive"]
    #[inline(always)]
    pub fn is_error_passive(&self) -> bool {
        *self == Fltconf::ErrorPassive
    }
    #[doc = "Bus Off"]
    #[inline(always)]
    pub fn is_bus_off(&self) -> bool {
        *self == Fltconf::BusOff
    }
}
#[doc = "FlexCAN In Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: FlexCAN is not transmitting a message."]
    TransmitMessageNo = 0,
    #[doc = "1: FlexCAN is transmitting a message."]
    TransmitMessageYes = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - FlexCAN In Transmission"]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            false => Tx::TransmitMessageNo,
            true => Tx::TransmitMessageYes,
        }
    }
    #[doc = "FlexCAN is not transmitting a message."]
    #[inline(always)]
    pub fn is_transmit_message_no(&self) -> bool {
        *self == Tx::TransmitMessageNo
    }
    #[doc = "FlexCAN is transmitting a message."]
    #[inline(always)]
    pub fn is_transmit_message_yes(&self) -> bool {
        *self == Tx::TransmitMessageYes
    }
}
#[doc = "IDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: No such occurrence."]
    CanBusNotIdle = 0,
    #[doc = "1: CAN bus is now IDLE."]
    CanBusIdle = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - IDLE"]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::CanBusNotIdle,
            true => Idle::CanBusIdle,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_can_bus_not_idle(&self) -> bool {
        *self == Idle::CanBusNotIdle
    }
    #[doc = "CAN bus is now IDLE."]
    #[inline(always)]
    pub fn is_can_bus_idle(&self) -> bool {
        *self == Idle::CanBusIdle
    }
}
#[doc = "Rx Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxwrn {
    #[doc = "0: No such occurrence."]
    RxerrcntLt96 = 0,
    #[doc = "1: RXERRCNT is greater than or equal to 96."]
    RxerrcntGte96 = 1,
}
impl From<Rxwrn> for bool {
    #[inline(always)]
    fn from(variant: Rxwrn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXWRN` reader - Rx Error Warning"]
pub type RxwrnR = crate::BitReader<Rxwrn>;
impl RxwrnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxwrn {
        match self.bits {
            false => Rxwrn::RxerrcntLt96,
            true => Rxwrn::RxerrcntGte96,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_rxerrcnt_lt_96(&self) -> bool {
        *self == Rxwrn::RxerrcntLt96
    }
    #[doc = "RXERRCNT is greater than or equal to 96."]
    #[inline(always)]
    pub fn is_rxerrcnt_gte_96(&self) -> bool {
        *self == Rxwrn::RxerrcntGte96
    }
}
#[doc = "TX Error Warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txwrn {
    #[doc = "0: No such occurrence."]
    TxerrcntLt96 = 0,
    #[doc = "1: TXERRCNT is greater than or equal to 96."]
    TxerrcntGte96 = 1,
}
impl From<Txwrn> for bool {
    #[inline(always)]
    fn from(variant: Txwrn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXWRN` reader - TX Error Warning"]
pub type TxwrnR = crate::BitReader<Txwrn>;
impl TxwrnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txwrn {
        match self.bits {
            false => Txwrn::TxerrcntLt96,
            true => Txwrn::TxerrcntGte96,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_txerrcnt_lt_96(&self) -> bool {
        *self == Txwrn::TxerrcntLt96
    }
    #[doc = "TXERRCNT is greater than or equal to 96."]
    #[inline(always)]
    pub fn is_txerrcnt_gte_96(&self) -> bool {
        *self == Txwrn::TxerrcntGte96
    }
}
#[doc = "Stuffing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stferr {
    #[doc = "0: No such occurrence."]
    StuffingErrorNo = 0,
    #[doc = "1: A stuffing error occurred since last read of this register."]
    StuffingErrorYes = 1,
}
impl From<Stferr> for bool {
    #[inline(always)]
    fn from(variant: Stferr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STFERR` reader - Stuffing Error"]
pub type StferrR = crate::BitReader<Stferr>;
impl StferrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stferr {
        match self.bits {
            false => Stferr::StuffingErrorNo,
            true => Stferr::StuffingErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_stuffing_error_no(&self) -> bool {
        *self == Stferr::StuffingErrorNo
    }
    #[doc = "A stuffing error occurred since last read of this register."]
    #[inline(always)]
    pub fn is_stuffing_error_yes(&self) -> bool {
        *self == Stferr::StuffingErrorYes
    }
}
#[doc = "Form Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frmerr {
    #[doc = "0: No such occurrence."]
    FormErrorNo = 0,
    #[doc = "1: A Form Error occurred since last read of this register."]
    FormErrorYes = 1,
}
impl From<Frmerr> for bool {
    #[inline(always)]
    fn from(variant: Frmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMERR` reader - Form Error"]
pub type FrmerrR = crate::BitReader<Frmerr>;
impl FrmerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frmerr {
        match self.bits {
            false => Frmerr::FormErrorNo,
            true => Frmerr::FormErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_form_error_no(&self) -> bool {
        *self == Frmerr::FormErrorNo
    }
    #[doc = "A Form Error occurred since last read of this register."]
    #[inline(always)]
    pub fn is_form_error_yes(&self) -> bool {
        *self == Frmerr::FormErrorYes
    }
}
#[doc = "Cyclic Redundancy Check Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crcerr {
    #[doc = "0: No such occurrence."]
    CrcErrorNo = 0,
    #[doc = "1: A CRC error occurred since last read of this register."]
    CrcErrorYes = 1,
}
impl From<Crcerr> for bool {
    #[inline(always)]
    fn from(variant: Crcerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR` reader - Cyclic Redundancy Check Error"]
pub type CrcerrR = crate::BitReader<Crcerr>;
impl CrcerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crcerr {
        match self.bits {
            false => Crcerr::CrcErrorNo,
            true => Crcerr::CrcErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_crc_error_no(&self) -> bool {
        *self == Crcerr::CrcErrorNo
    }
    #[doc = "A CRC error occurred since last read of this register."]
    #[inline(always)]
    pub fn is_crc_error_yes(&self) -> bool {
        *self == Crcerr::CrcErrorYes
    }
}
#[doc = "Acknowledge Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackerr {
    #[doc = "0: No such occurrence."]
    AckErrorNo = 0,
    #[doc = "1: An ACK error occurred since last read of this register."]
    AckErrorYes = 1,
}
impl From<Ackerr> for bool {
    #[inline(always)]
    fn from(variant: Ackerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKERR` reader - Acknowledge Error"]
pub type AckerrR = crate::BitReader<Ackerr>;
impl AckerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackerr {
        match self.bits {
            false => Ackerr::AckErrorNo,
            true => Ackerr::AckErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_ack_error_no(&self) -> bool {
        *self == Ackerr::AckErrorNo
    }
    #[doc = "An ACK error occurred since last read of this register."]
    #[inline(always)]
    pub fn is_ack_error_yes(&self) -> bool {
        *self == Ackerr::AckErrorYes
    }
}
#[doc = "Bit0 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit0err {
    #[doc = "0: No such occurrence."]
    Bit0ErrorNo = 0,
    #[doc = "1: At least one bit sent as dominant is received as recessive."]
    Bit0ErrorYes = 1,
}
impl From<Bit0err> for bool {
    #[inline(always)]
    fn from(variant: Bit0err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT0ERR` reader - Bit0 Error"]
pub type Bit0errR = crate::BitReader<Bit0err>;
impl Bit0errR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit0err {
        match self.bits {
            false => Bit0err::Bit0ErrorNo,
            true => Bit0err::Bit0ErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_bit0_error_no(&self) -> bool {
        *self == Bit0err::Bit0ErrorNo
    }
    #[doc = "At least one bit sent as dominant is received as recessive."]
    #[inline(always)]
    pub fn is_bit0_error_yes(&self) -> bool {
        *self == Bit0err::Bit0ErrorYes
    }
}
#[doc = "Bit1 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit1err {
    #[doc = "0: No such occurrence."]
    Bit1ErrorNo = 0,
    #[doc = "1: At least one bit sent as recessive is received as dominant."]
    Bit1ErrorYes = 1,
}
impl From<Bit1err> for bool {
    #[inline(always)]
    fn from(variant: Bit1err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT1ERR` reader - Bit1 Error"]
pub type Bit1errR = crate::BitReader<Bit1err>;
impl Bit1errR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit1err {
        match self.bits {
            false => Bit1err::Bit1ErrorNo,
            true => Bit1err::Bit1ErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_bit1_error_no(&self) -> bool {
        *self == Bit1err::Bit1ErrorNo
    }
    #[doc = "At least one bit sent as recessive is received as dominant."]
    #[inline(always)]
    pub fn is_bit1_error_yes(&self) -> bool {
        *self == Bit1err::Bit1ErrorYes
    }
}
#[doc = "Rx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwrnint {
    #[doc = "0: No such occurrence."]
    RxWarningIntNo = 0,
    #[doc = "1: The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    RxWarningIntYes = 1,
}
impl From<Rwrnint> for bool {
    #[inline(always)]
    fn from(variant: Rwrnint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWRNINT` reader - Rx Warning Interrupt Flag"]
pub type RwrnintR = crate::BitReader<Rwrnint>;
impl RwrnintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwrnint {
        match self.bits {
            false => Rwrnint::RxWarningIntNo,
            true => Rwrnint::RxWarningIntYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_rx_warning_int_no(&self) -> bool {
        *self == Rwrnint::RxWarningIntNo
    }
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn is_rx_warning_int_yes(&self) -> bool {
        *self == Rwrnint::RxWarningIntYes
    }
}
#[doc = "Field `RWRNINT` writer - Rx Warning Interrupt Flag"]
pub type RwrnintW<'a, REG> = crate::BitWriter1C<'a, REG, Rwrnint>;
impl<'a, REG> RwrnintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn rx_warning_int_no(self) -> &'a mut crate::W<REG> {
        self.variant(Rwrnint::RxWarningIntNo)
    }
    #[doc = "The Rx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn rx_warning_int_yes(self) -> &'a mut crate::W<REG> {
        self.variant(Rwrnint::RxWarningIntYes)
    }
}
#[doc = "Tx Warning Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twrnint {
    #[doc = "0: No such occurrence."]
    TxWarningIntNo = 0,
    #[doc = "1: The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    TxWarningIntYes = 1,
}
impl From<Twrnint> for bool {
    #[inline(always)]
    fn from(variant: Twrnint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWRNINT` reader - Tx Warning Interrupt Flag"]
pub type TwrnintR = crate::BitReader<Twrnint>;
impl TwrnintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Twrnint {
        match self.bits {
            false => Twrnint::TxWarningIntNo,
            true => Twrnint::TxWarningIntYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_tx_warning_int_no(&self) -> bool {
        *self == Twrnint::TxWarningIntNo
    }
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn is_tx_warning_int_yes(&self) -> bool {
        *self == Twrnint::TxWarningIntYes
    }
}
#[doc = "Field `TWRNINT` writer - Tx Warning Interrupt Flag"]
pub type TwrnintW<'a, REG> = crate::BitWriter1C<'a, REG, Twrnint>;
impl<'a, REG> TwrnintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn tx_warning_int_no(self) -> &'a mut crate::W<REG> {
        self.variant(Twrnint::TxWarningIntNo)
    }
    #[doc = "The Tx error counter transitioned from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn tx_warning_int_yes(self) -> &'a mut crate::W<REG> {
        self.variant(Twrnint::TxWarningIntYes)
    }
}
#[doc = "CAN Synchronization Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Synch {
    #[doc = "0: FlexCAN is not synchronized to the CAN bus."]
    CanBusSyncNo = 0,
    #[doc = "1: FlexCAN is synchronized to the CAN bus."]
    CanBusSyncYes = 1,
}
impl From<Synch> for bool {
    #[inline(always)]
    fn from(variant: Synch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNCH` reader - CAN Synchronization Status"]
pub type SynchR = crate::BitReader<Synch>;
impl SynchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Synch {
        match self.bits {
            false => Synch::CanBusSyncNo,
            true => Synch::CanBusSyncYes,
        }
    }
    #[doc = "FlexCAN is not synchronized to the CAN bus."]
    #[inline(always)]
    pub fn is_can_bus_sync_no(&self) -> bool {
        *self == Synch::CanBusSyncNo
    }
    #[doc = "FlexCAN is synchronized to the CAN bus."]
    #[inline(always)]
    pub fn is_can_bus_sync_yes(&self) -> bool {
        *self == Synch::CanBusSyncYes
    }
}
#[doc = "Bus Off Done Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boffdoneint {
    #[doc = "0: No such occurrence."]
    BusOffNotDone = 0,
    #[doc = "1: FlexCAN module has completed Bus Off process."]
    BusOffDone = 1,
}
impl From<Boffdoneint> for bool {
    #[inline(always)]
    fn from(variant: Boffdoneint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFDONEINT` reader - Bus Off Done Interrupt"]
pub type BoffdoneintR = crate::BitReader<Boffdoneint>;
impl BoffdoneintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boffdoneint {
        match self.bits {
            false => Boffdoneint::BusOffNotDone,
            true => Boffdoneint::BusOffDone,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_bus_off_not_done(&self) -> bool {
        *self == Boffdoneint::BusOffNotDone
    }
    #[doc = "FlexCAN module has completed Bus Off process."]
    #[inline(always)]
    pub fn is_bus_off_done(&self) -> bool {
        *self == Boffdoneint::BusOffDone
    }
}
#[doc = "Field `BOFFDONEINT` writer - Bus Off Done Interrupt"]
pub type BoffdoneintW<'a, REG> = crate::BitWriter1C<'a, REG, Boffdoneint>;
impl<'a, REG> BoffdoneintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn bus_off_not_done(self) -> &'a mut crate::W<REG> {
        self.variant(Boffdoneint::BusOffNotDone)
    }
    #[doc = "FlexCAN module has completed Bus Off process."]
    #[inline(always)]
    pub fn bus_off_done(self) -> &'a mut crate::W<REG> {
        self.variant(Boffdoneint::BusOffDone)
    }
}
#[doc = "Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrintFast {
    #[doc = "0: No such occurrence."]
    ErrorsDataPhaseNo = 0,
    #[doc = "1: Indicates setting of any error bit detected in the data phase of CAN FD frames with the BRS bit set."]
    ErrorsDataPhaseYes = 1,
}
impl From<ErrintFast> for bool {
    #[inline(always)]
    fn from(variant: ErrintFast) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRINT_FAST` reader - Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
pub type ErrintFastR = crate::BitReader<ErrintFast>;
impl ErrintFastR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrintFast {
        match self.bits {
            false => ErrintFast::ErrorsDataPhaseNo,
            true => ErrintFast::ErrorsDataPhaseYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_errors_data_phase_no(&self) -> bool {
        *self == ErrintFast::ErrorsDataPhaseNo
    }
    #[doc = "Indicates setting of any error bit detected in the data phase of CAN FD frames with the BRS bit set."]
    #[inline(always)]
    pub fn is_errors_data_phase_yes(&self) -> bool {
        *self == ErrintFast::ErrorsDataPhaseYes
    }
}
#[doc = "Field `ERRINT_FAST` writer - Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
pub type ErrintFastW<'a, REG> = crate::BitWriter1C<'a, REG, ErrintFast>;
impl<'a, REG> ErrintFastW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn errors_data_phase_no(self) -> &'a mut crate::W<REG> {
        self.variant(ErrintFast::ErrorsDataPhaseNo)
    }
    #[doc = "Indicates setting of any error bit detected in the data phase of CAN FD frames with the BRS bit set."]
    #[inline(always)]
    pub fn errors_data_phase_yes(self) -> &'a mut crate::W<REG> {
        self.variant(ErrintFast::ErrorsDataPhaseYes)
    }
}
#[doc = "Error Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Errovr {
    #[doc = "0: Overrun has not occurred."]
    OverrunNotOccurred = 0,
    #[doc = "1: Overrun has occurred."]
    OverrunOccurred = 1,
}
impl From<Errovr> for bool {
    #[inline(always)]
    fn from(variant: Errovr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROVR` reader - Error Overrun"]
pub type ErrovrR = crate::BitReader<Errovr>;
impl ErrovrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Errovr {
        match self.bits {
            false => Errovr::OverrunNotOccurred,
            true => Errovr::OverrunOccurred,
        }
    }
    #[doc = "Overrun has not occurred."]
    #[inline(always)]
    pub fn is_overrun_not_occurred(&self) -> bool {
        *self == Errovr::OverrunNotOccurred
    }
    #[doc = "Overrun has occurred."]
    #[inline(always)]
    pub fn is_overrun_occurred(&self) -> bool {
        *self == Errovr::OverrunOccurred
    }
}
#[doc = "Field `ERROVR` writer - Error Overrun"]
pub type ErrovrW<'a, REG> = crate::BitWriter1C<'a, REG, Errovr>;
impl<'a, REG> ErrovrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Overrun has not occurred."]
    #[inline(always)]
    pub fn overrun_not_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(Errovr::OverrunNotOccurred)
    }
    #[doc = "Overrun has occurred."]
    #[inline(always)]
    pub fn overrun_occurred(self) -> &'a mut crate::W<REG> {
        self.variant(Errovr::OverrunOccurred)
    }
}
#[doc = "Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StferrFast {
    #[doc = "0: No such occurrence."]
    StuffingErrorNo = 0,
    #[doc = "1: A stuffing error occurred since last read of this register."]
    StuffingErrorYes = 1,
}
impl From<StferrFast> for bool {
    #[inline(always)]
    fn from(variant: StferrFast) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STFERR_FAST` reader - Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set"]
pub type StferrFastR = crate::BitReader<StferrFast>;
impl StferrFastR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StferrFast {
        match self.bits {
            false => StferrFast::StuffingErrorNo,
            true => StferrFast::StuffingErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_stuffing_error_no(&self) -> bool {
        *self == StferrFast::StuffingErrorNo
    }
    #[doc = "A stuffing error occurred since last read of this register."]
    #[inline(always)]
    pub fn is_stuffing_error_yes(&self) -> bool {
        *self == StferrFast::StuffingErrorYes
    }
}
#[doc = "Form Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FrmerrFast {
    #[doc = "0: No such occurrence."]
    FormErrorNo = 0,
    #[doc = "1: A form error occurred since last read of this register."]
    FormErrorYes = 1,
}
impl From<FrmerrFast> for bool {
    #[inline(always)]
    fn from(variant: FrmerrFast) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRMERR_FAST` reader - Form Error in the Data Phase of CAN FD frames with the BRS bit set"]
pub type FrmerrFastR = crate::BitReader<FrmerrFast>;
impl FrmerrFastR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FrmerrFast {
        match self.bits {
            false => FrmerrFast::FormErrorNo,
            true => FrmerrFast::FormErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_form_error_no(&self) -> bool {
        *self == FrmerrFast::FormErrorNo
    }
    #[doc = "A form error occurred since last read of this register."]
    #[inline(always)]
    pub fn is_form_error_yes(&self) -> bool {
        *self == FrmerrFast::FormErrorYes
    }
}
#[doc = "Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcerrFast {
    #[doc = "0: No such occurrence."]
    CrcErrorNo = 0,
    #[doc = "1: A CRC error occurred since last read of this register."]
    CrcErrorYes = 1,
}
impl From<CrcerrFast> for bool {
    #[inline(always)]
    fn from(variant: CrcerrFast) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCERR_FAST` reader - Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set"]
pub type CrcerrFastR = crate::BitReader<CrcerrFast>;
impl CrcerrFastR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcerrFast {
        match self.bits {
            false => CrcerrFast::CrcErrorNo,
            true => CrcerrFast::CrcErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_crc_error_no(&self) -> bool {
        *self == CrcerrFast::CrcErrorNo
    }
    #[doc = "A CRC error occurred since last read of this register."]
    #[inline(always)]
    pub fn is_crc_error_yes(&self) -> bool {
        *self == CrcerrFast::CrcErrorYes
    }
}
#[doc = "Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit0errFast {
    #[doc = "0: No such occurrence."]
    Bit0ErrorNo = 0,
    #[doc = "1: At least one bit sent as dominant is received as recessive."]
    Bit0ErrorYes = 1,
}
impl From<Bit0errFast> for bool {
    #[inline(always)]
    fn from(variant: Bit0errFast) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT0ERR_FAST` reader - Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set"]
pub type Bit0errFastR = crate::BitReader<Bit0errFast>;
impl Bit0errFastR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit0errFast {
        match self.bits {
            false => Bit0errFast::Bit0ErrorNo,
            true => Bit0errFast::Bit0ErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_bit0_error_no(&self) -> bool {
        *self == Bit0errFast::Bit0ErrorNo
    }
    #[doc = "At least one bit sent as dominant is received as recessive."]
    #[inline(always)]
    pub fn is_bit0_error_yes(&self) -> bool {
        *self == Bit0errFast::Bit0ErrorYes
    }
}
#[doc = "Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bit1errFast {
    #[doc = "0: No such occurrence."]
    Bit1ErrorNo = 0,
    #[doc = "1: At least one bit sent as recessive is received as dominant."]
    Bit1ErrorYes = 1,
}
impl From<Bit1errFast> for bool {
    #[inline(always)]
    fn from(variant: Bit1errFast) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT1ERR_FAST` reader - Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set"]
pub type Bit1errFastR = crate::BitReader<Bit1errFast>;
impl Bit1errFastR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bit1errFast {
        match self.bits {
            false => Bit1errFast::Bit1ErrorNo,
            true => Bit1errFast::Bit1ErrorYes,
        }
    }
    #[doc = "No such occurrence."]
    #[inline(always)]
    pub fn is_bit1_error_no(&self) -> bool {
        *self == Bit1errFast::Bit1ErrorNo
    }
    #[doc = "At least one bit sent as recessive is received as dominant."]
    #[inline(always)]
    pub fn is_bit1_error_yes(&self) -> bool {
        *self == Bit1errFast::Bit1ErrorYes
    }
}
impl R {
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&self) -> ErrintR {
        ErrintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    pub fn boffint(&self) -> BoffintR {
        BoffintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FlexCAN In Reception"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Fault Confinement State"]
    #[inline(always)]
    pub fn fltconf(&self) -> FltconfR {
        FltconfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - FlexCAN In Transmission"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IDLE"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rx Error Warning"]
    #[inline(always)]
    pub fn rxwrn(&self) -> RxwrnR {
        RxwrnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX Error Warning"]
    #[inline(always)]
    pub fn txwrn(&self) -> TxwrnR {
        TxwrnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stuffing Error"]
    #[inline(always)]
    pub fn stferr(&self) -> StferrR {
        StferrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Form Error"]
    #[inline(always)]
    pub fn frmerr(&self) -> FrmerrR {
        FrmerrR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cyclic Redundancy Check Error"]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Acknowledge Error"]
    #[inline(always)]
    pub fn ackerr(&self) -> AckerrR {
        AckerrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit0 Error"]
    #[inline(always)]
    pub fn bit0err(&self) -> Bit0errR {
        Bit0errR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bit1 Error"]
    #[inline(always)]
    pub fn bit1err(&self) -> Bit1errR {
        Bit1errR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn rwrnint(&self) -> RwrnintR {
        RwrnintR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn twrnint(&self) -> TwrnintR {
        TwrnintR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CAN Synchronization Status"]
    #[inline(always)]
    pub fn synch(&self) -> SynchR {
        SynchR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus Off Done Interrupt"]
    #[inline(always)]
    pub fn boffdoneint(&self) -> BoffdoneintR {
        BoffdoneintR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
    #[inline(always)]
    pub fn errint_fast(&self) -> ErrintFastR {
        ErrintFastR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Error Overrun"]
    #[inline(always)]
    pub fn errovr(&self) -> ErrovrR {
        ErrovrR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - Stuffing Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn stferr_fast(&self) -> StferrFastR {
        StferrFastR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Form Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn frmerr_fast(&self) -> FrmerrFastR {
        FrmerrFastR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Cyclic Redundancy Check Error in the CRC field of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn crcerr_fast(&self) -> CrcerrFastR {
        CrcerrFastR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Bit0 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn bit0err_fast(&self) -> Bit0errFastR {
        Bit0errFastR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bit1 Error in the Data Phase of CAN FD frames with the BRS bit set"]
    #[inline(always)]
    pub fn bit1err_fast(&self) -> Bit1errFastR {
        Bit1errFastR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Error Interrupt"]
    #[inline(always)]
    pub fn errint(&mut self) -> ErrintW<Esr1Spec> {
        ErrintW::new(self, 1)
    }
    #[doc = "Bit 2 - Bus Off Interrupt"]
    #[inline(always)]
    pub fn boffint(&mut self) -> BoffintW<Esr1Spec> {
        BoffintW::new(self, 2)
    }
    #[doc = "Bit 16 - Rx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn rwrnint(&mut self) -> RwrnintW<Esr1Spec> {
        RwrnintW::new(self, 16)
    }
    #[doc = "Bit 17 - Tx Warning Interrupt Flag"]
    #[inline(always)]
    pub fn twrnint(&mut self) -> TwrnintW<Esr1Spec> {
        TwrnintW::new(self, 17)
    }
    #[doc = "Bit 19 - Bus Off Done Interrupt"]
    #[inline(always)]
    pub fn boffdoneint(&mut self) -> BoffdoneintW<Esr1Spec> {
        BoffdoneintW::new(self, 19)
    }
    #[doc = "Bit 20 - Error interrupt for errors detected in Data Phase of CAN FD frames with BRS bit set"]
    #[inline(always)]
    pub fn errint_fast(&mut self) -> ErrintFastW<Esr1Spec> {
        ErrintFastW::new(self, 20)
    }
    #[doc = "Bit 21 - Error Overrun"]
    #[inline(always)]
    pub fn errovr(&mut self) -> ErrovrW<Esr1Spec> {
        ErrovrW::new(self, 21)
    }
}
#[doc = "Error and Status 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`esr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Esr1Spec;
impl crate::RegisterSpec for Esr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`esr1::R`](R) reader structure"]
impl crate::Readable for Esr1Spec {}
#[doc = "`write(|w| ..)` method takes [`esr1::W`](W) writer structure"]
impl crate::Writable for Esr1Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x003b_0006;
}
#[doc = "`reset()` method sets ESR1 to value 0"]
impl crate::Resettable for Esr1Spec {}
