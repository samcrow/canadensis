#[doc = "Register `CTRL2` reader"]
pub type R = crate::R<Ctrl2Spec>;
#[doc = "Register `CTRL2` writer"]
pub type W = crate::W<Ctrl2Spec>;
#[doc = "Time Stamp Capture Point\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstampcap {
    #[doc = "0: The high resolution time stamp capture is disabled"]
    TimeStampCaptureDisabled = 0,
    #[doc = "1: The high resolution time stamp is captured in the end of the CAN frame"]
    TimeStampCaptureFrameEndEnabled = 1,
    #[doc = "2: The high resolution time stamp is captured in the start of the CAN frame"]
    TimeStampCaptureFrameStartEnabled = 2,
    #[doc = "3: The high resolution time stamp is captured in the start of frame for classical CAN frames and in res bit for CAN FD frames"]
    TimeStampCapture2TypesEnabled = 3,
}
impl From<Tstampcap> for u8 {
    #[inline(always)]
    fn from(variant: Tstampcap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstampcap {
    type Ux = u8;
}
impl crate::IsEnum for Tstampcap {}
#[doc = "Field `TSTAMPCAP` reader - Time Stamp Capture Point"]
pub type TstampcapR = crate::FieldReader<Tstampcap>;
impl TstampcapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstampcap {
        match self.bits {
            0 => Tstampcap::TimeStampCaptureDisabled,
            1 => Tstampcap::TimeStampCaptureFrameEndEnabled,
            2 => Tstampcap::TimeStampCaptureFrameStartEnabled,
            3 => Tstampcap::TimeStampCapture2TypesEnabled,
            _ => unreachable!(),
        }
    }
    #[doc = "The high resolution time stamp capture is disabled"]
    #[inline(always)]
    pub fn is_time_stamp_capture_disabled(&self) -> bool {
        *self == Tstampcap::TimeStampCaptureDisabled
    }
    #[doc = "The high resolution time stamp is captured in the end of the CAN frame"]
    #[inline(always)]
    pub fn is_time_stamp_capture_frame_end_enabled(&self) -> bool {
        *self == Tstampcap::TimeStampCaptureFrameEndEnabled
    }
    #[doc = "The high resolution time stamp is captured in the start of the CAN frame"]
    #[inline(always)]
    pub fn is_time_stamp_capture_frame_start_enabled(&self) -> bool {
        *self == Tstampcap::TimeStampCaptureFrameStartEnabled
    }
    #[doc = "The high resolution time stamp is captured in the start of frame for classical CAN frames and in res bit for CAN FD frames"]
    #[inline(always)]
    pub fn is_time_stamp_capture_2_types_enabled(&self) -> bool {
        *self == Tstampcap::TimeStampCapture2TypesEnabled
    }
}
#[doc = "Field `TSTAMPCAP` writer - Time Stamp Capture Point"]
pub type TstampcapW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tstampcap, crate::Safe>;
impl<'a, REG> TstampcapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The high resolution time stamp capture is disabled"]
    #[inline(always)]
    pub fn time_stamp_capture_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tstampcap::TimeStampCaptureDisabled)
    }
    #[doc = "The high resolution time stamp is captured in the end of the CAN frame"]
    #[inline(always)]
    pub fn time_stamp_capture_frame_end_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tstampcap::TimeStampCaptureFrameEndEnabled)
    }
    #[doc = "The high resolution time stamp is captured in the start of the CAN frame"]
    #[inline(always)]
    pub fn time_stamp_capture_frame_start_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tstampcap::TimeStampCaptureFrameStartEnabled)
    }
    #[doc = "The high resolution time stamp is captured in the start of frame for classical CAN frames and in res bit for CAN FD frames"]
    #[inline(always)]
    pub fn time_stamp_capture_2_types_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tstampcap::TimeStampCapture2TypesEnabled)
    }
}
#[doc = "Message Buffer Time Stamp Base\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mbtsbase {
    #[doc = "0: Message buffer time stamp base is TIMER"]
    BaseTimer = 0,
    #[doc = "1: Message buffer time stamp base is lower 16 bits of high resolution timer"]
    BaseLower16 = 1,
    #[doc = "2: Message buffer time stamp base is upper 16 bits of high resolution timer"]
    BaseUpper16 = 2,
}
impl From<Mbtsbase> for u8 {
    #[inline(always)]
    fn from(variant: Mbtsbase) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mbtsbase {
    type Ux = u8;
}
impl crate::IsEnum for Mbtsbase {}
#[doc = "Field `MBTSBASE` reader - Message Buffer Time Stamp Base"]
pub type MbtsbaseR = crate::FieldReader<Mbtsbase>;
impl MbtsbaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mbtsbase> {
        match self.bits {
            0 => Some(Mbtsbase::BaseTimer),
            1 => Some(Mbtsbase::BaseLower16),
            2 => Some(Mbtsbase::BaseUpper16),
            _ => None,
        }
    }
    #[doc = "Message buffer time stamp base is TIMER"]
    #[inline(always)]
    pub fn is_base_timer(&self) -> bool {
        *self == Mbtsbase::BaseTimer
    }
    #[doc = "Message buffer time stamp base is lower 16 bits of high resolution timer"]
    #[inline(always)]
    pub fn is_base_lower_16(&self) -> bool {
        *self == Mbtsbase::BaseLower16
    }
    #[doc = "Message buffer time stamp base is upper 16 bits of high resolution timer"]
    #[inline(always)]
    pub fn is_base_upper_16(&self) -> bool {
        *self == Mbtsbase::BaseUpper16
    }
}
#[doc = "Field `MBTSBASE` writer - Message Buffer Time Stamp Base"]
pub type MbtsbaseW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mbtsbase>;
impl<'a, REG> MbtsbaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Message buffer time stamp base is TIMER"]
    #[inline(always)]
    pub fn base_timer(self) -> &'a mut crate::W<REG> {
        self.variant(Mbtsbase::BaseTimer)
    }
    #[doc = "Message buffer time stamp base is lower 16 bits of high resolution timer"]
    #[inline(always)]
    pub fn base_lower_16(self) -> &'a mut crate::W<REG> {
        self.variant(Mbtsbase::BaseLower16)
    }
    #[doc = "Message buffer time stamp base is upper 16 bits of high resolution timer"]
    #[inline(always)]
    pub fn base_upper_16(self) -> &'a mut crate::W<REG> {
        self.variant(Mbtsbase::BaseUpper16)
    }
}
#[doc = "Edge Filter Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edfltdis {
    #[doc = "0: Edge filter is enabled"]
    Enable = 0,
    #[doc = "1: Edge filter is disabled"]
    Disable = 1,
}
impl From<Edfltdis> for bool {
    #[inline(always)]
    fn from(variant: Edfltdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDFLTDIS` reader - Edge Filter Disable"]
pub type EdfltdisR = crate::BitReader<Edfltdis>;
impl EdfltdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edfltdis {
        match self.bits {
            false => Edfltdis::Enable,
            true => Edfltdis::Disable,
        }
    }
    #[doc = "Edge filter is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Edfltdis::Enable
    }
    #[doc = "Edge filter is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Edfltdis::Disable
    }
}
#[doc = "Field `EDFLTDIS` writer - Edge Filter Disable"]
pub type EdfltdisW<'a, REG> = crate::BitWriter<'a, REG, Edfltdis>;
impl<'a, REG> EdfltdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Edge filter is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Edfltdis::Enable)
    }
    #[doc = "Edge filter is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Edfltdis::Disable)
    }
}
#[doc = "ISO CAN FD Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isocanfden {
    #[doc = "0: FlexCAN operates using the non-ISO CAN FD protocol."]
    NonIso = 0,
    #[doc = "1: FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    Iso = 1,
}
impl From<Isocanfden> for bool {
    #[inline(always)]
    fn from(variant: Isocanfden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISOCANFDEN` reader - ISO CAN FD Enable"]
pub type IsocanfdenR = crate::BitReader<Isocanfden>;
impl IsocanfdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isocanfden {
        match self.bits {
            false => Isocanfden::NonIso,
            true => Isocanfden::Iso,
        }
    }
    #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
    #[inline(always)]
    pub fn is_non_iso(&self) -> bool {
        *self == Isocanfden::NonIso
    }
    #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == Isocanfden::Iso
    }
}
#[doc = "Field `ISOCANFDEN` writer - ISO CAN FD Enable"]
pub type IsocanfdenW<'a, REG> = crate::BitWriter<'a, REG, Isocanfden>;
impl<'a, REG> IsocanfdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FlexCAN operates using the non-ISO CAN FD protocol."]
    #[inline(always)]
    pub fn non_iso(self) -> &'a mut crate::W<REG> {
        self.variant(Isocanfden::NonIso)
    }
    #[doc = "FlexCAN operates using the ISO CAN FD protocol (ISO 11898-1)."]
    #[inline(always)]
    pub fn iso(self) -> &'a mut crate::W<REG> {
        self.variant(Isocanfden::Iso)
    }
}
#[doc = "Bit Timing Expansion enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bte {
    #[doc = "0: CAN Bit timing expansion is disabled."]
    Disable = 0,
    #[doc = "1: CAN bit timing expansion is enabled."]
    Enable = 1,
}
impl From<Bte> for bool {
    #[inline(always)]
    fn from(variant: Bte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTE` reader - Bit Timing Expansion enable"]
pub type BteR = crate::BitReader<Bte>;
impl BteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bte {
        match self.bits {
            false => Bte::Disable,
            true => Bte::Enable,
        }
    }
    #[doc = "CAN Bit timing expansion is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Bte::Disable
    }
    #[doc = "CAN bit timing expansion is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Bte::Enable
    }
}
#[doc = "Field `BTE` writer - Bit Timing Expansion enable"]
pub type BteW<'a, REG> = crate::BitWriter<'a, REG, Bte>;
impl<'a, REG> BteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN Bit timing expansion is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Bte::Disable)
    }
    #[doc = "CAN bit timing expansion is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Bte::Enable)
    }
}
#[doc = "Protocol Exception Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prexcen {
    #[doc = "0: Protocol exception is disabled."]
    Disable = 0,
    #[doc = "1: Protocol exception is enabled."]
    Enable = 1,
}
impl From<Prexcen> for bool {
    #[inline(always)]
    fn from(variant: Prexcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREXCEN` reader - Protocol Exception Enable"]
pub type PrexcenR = crate::BitReader<Prexcen>;
impl PrexcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prexcen {
        match self.bits {
            false => Prexcen::Disable,
            true => Prexcen::Enable,
        }
    }
    #[doc = "Protocol exception is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prexcen::Disable
    }
    #[doc = "Protocol exception is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Prexcen::Enable
    }
}
#[doc = "Field `PREXCEN` writer - Protocol Exception Enable"]
pub type PrexcenW<'a, REG> = crate::BitWriter<'a, REG, Prexcen>;
impl<'a, REG> PrexcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Protocol exception is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prexcen::Disable)
    }
    #[doc = "Protocol exception is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Prexcen::Enable)
    }
}
#[doc = "Timer Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerSrc {
    #[doc = "0: The free running timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
    CanBitClock = 0,
    #[doc = "1: The free running timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device-specific section for details about the external time tick."]
    ExternalClock = 1,
}
impl From<TimerSrc> for bool {
    #[inline(always)]
    fn from(variant: TimerSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMER_SRC` reader - Timer Source"]
pub type TimerSrcR = crate::BitReader<TimerSrc>;
impl TimerSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimerSrc {
        match self.bits {
            false => TimerSrc::CanBitClock,
            true => TimerSrc::ExternalClock,
        }
    }
    #[doc = "The free running timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
    #[inline(always)]
    pub fn is_can_bit_clock(&self) -> bool {
        *self == TimerSrc::CanBitClock
    }
    #[doc = "The free running timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device-specific section for details about the external time tick."]
    #[inline(always)]
    pub fn is_external_clock(&self) -> bool {
        *self == TimerSrc::ExternalClock
    }
}
#[doc = "Field `TIMER_SRC` writer - Timer Source"]
pub type TimerSrcW<'a, REG> = crate::BitWriter<'a, REG, TimerSrc>;
impl<'a, REG> TimerSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The free running timer is clocked by the CAN bit clock, which defines the baud rate on the CAN bus."]
    #[inline(always)]
    pub fn can_bit_clock(self) -> &'a mut crate::W<REG> {
        self.variant(TimerSrc::CanBitClock)
    }
    #[doc = "The free running timer is clocked by an external time tick. The period can be either adjusted to be equal to the baud rate on the CAN bus, or a different value as required. See the device-specific section for details about the external time tick."]
    #[inline(always)]
    pub fn external_clock(self) -> &'a mut crate::W<REG> {
        self.variant(TimerSrc::ExternalClock)
    }
}
#[doc = "Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eacen {
    #[doc = "0: Rx mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    RtrCompareNo = 0,
    #[doc = "1: Enables the comparison of both Rx mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    RtrCompareYes = 1,
}
impl From<Eacen> for bool {
    #[inline(always)]
    fn from(variant: Eacen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EACEN` reader - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
pub type EacenR = crate::BitReader<Eacen>;
impl EacenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eacen {
        match self.bits {
            false => Eacen::RtrCompareNo,
            true => Eacen::RtrCompareYes,
        }
    }
    #[doc = "Rx mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline(always)]
    pub fn is_rtr_compare_no(&self) -> bool {
        *self == Eacen::RtrCompareNo
    }
    #[doc = "Enables the comparison of both Rx mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline(always)]
    pub fn is_rtr_compare_yes(&self) -> bool {
        *self == Eacen::RtrCompareYes
    }
}
#[doc = "Field `EACEN` writer - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
pub type EacenW<'a, REG> = crate::BitWriter<'a, REG, Eacen>;
impl<'a, REG> EacenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline(always)]
    pub fn rtr_compare_no(self) -> &'a mut crate::W<REG> {
        self.variant(Eacen::RtrCompareNo)
    }
    #[doc = "Enables the comparison of both Rx mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline(always)]
    pub fn rtr_compare_yes(self) -> &'a mut crate::W<REG> {
        self.variant(Eacen::RtrCompareYes)
    }
}
#[doc = "Remote Request Storing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrs {
    #[doc = "0: Remote response frame is generated."]
    RemoteResponseFrameNotGenerated = 0,
    #[doc = "1: Remote request frame is stored."]
    RemoteResponseFrameGenerated = 1,
}
impl From<Rrs> for bool {
    #[inline(always)]
    fn from(variant: Rrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRS` reader - Remote Request Storing"]
pub type RrsR = crate::BitReader<Rrs>;
impl RrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rrs {
        match self.bits {
            false => Rrs::RemoteResponseFrameNotGenerated,
            true => Rrs::RemoteResponseFrameGenerated,
        }
    }
    #[doc = "Remote response frame is generated."]
    #[inline(always)]
    pub fn is_remote_response_frame_not_generated(&self) -> bool {
        *self == Rrs::RemoteResponseFrameNotGenerated
    }
    #[doc = "Remote request frame is stored."]
    #[inline(always)]
    pub fn is_remote_response_frame_generated(&self) -> bool {
        *self == Rrs::RemoteResponseFrameGenerated
    }
}
#[doc = "Field `RRS` writer - Remote Request Storing"]
pub type RrsW<'a, REG> = crate::BitWriter<'a, REG, Rrs>;
impl<'a, REG> RrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Remote response frame is generated."]
    #[inline(always)]
    pub fn remote_response_frame_not_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::RemoteResponseFrameNotGenerated)
    }
    #[doc = "Remote request frame is stored."]
    #[inline(always)]
    pub fn remote_response_frame_generated(self) -> &'a mut crate::W<REG> {
        self.variant(Rrs::RemoteResponseFrameGenerated)
    }
}
#[doc = "Mailboxes Reception Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mrp {
    #[doc = "0: Matching starts from Legacy Rx FIFO or Enhanced Rx FIFO and continues on mailboxes."]
    Id1 = 0,
    #[doc = "1: Matching starts from mailboxes and continues on Legacy Rx FIFO or Enhanced Rx FIFO."]
    Id3 = 1,
}
impl From<Mrp> for bool {
    #[inline(always)]
    fn from(variant: Mrp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRP` reader - Mailboxes Reception Priority"]
pub type MrpR = crate::BitReader<Mrp>;
impl MrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mrp {
        match self.bits {
            false => Mrp::Id1,
            true => Mrp::Id3,
        }
    }
    #[doc = "Matching starts from Legacy Rx FIFO or Enhanced Rx FIFO and continues on mailboxes."]
    #[inline(always)]
    pub fn is_id1(&self) -> bool {
        *self == Mrp::Id1
    }
    #[doc = "Matching starts from mailboxes and continues on Legacy Rx FIFO or Enhanced Rx FIFO."]
    #[inline(always)]
    pub fn is_id3(&self) -> bool {
        *self == Mrp::Id3
    }
}
#[doc = "Field `MRP` writer - Mailboxes Reception Priority"]
pub type MrpW<'a, REG> = crate::BitWriter<'a, REG, Mrp>;
impl<'a, REG> MrpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Matching starts from Legacy Rx FIFO or Enhanced Rx FIFO and continues on mailboxes."]
    #[inline(always)]
    pub fn id1(self) -> &'a mut crate::W<REG> {
        self.variant(Mrp::Id1)
    }
    #[doc = "Matching starts from mailboxes and continues on Legacy Rx FIFO or Enhanced Rx FIFO."]
    #[inline(always)]
    pub fn id3(self) -> &'a mut crate::W<REG> {
        self.variant(Mrp::Id3)
    }
}
#[doc = "Field `TASD` reader - Tx Arbitration Start Delay"]
pub type TasdR = crate::FieldReader;
#[doc = "Field `TASD` writer - Tx Arbitration Start Delay"]
pub type TasdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RFFN` reader - Number Of Legacy Rx FIFO Filters"]
pub type RffnR = crate::FieldReader;
#[doc = "Field `RFFN` writer - Number Of Legacy Rx FIFO Filters"]
pub type RffnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Write-Access To Memory In Freeze Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wrmfrz {
    #[doc = "0: Maintain the write access restrictions."]
    Disable = 0,
    #[doc = "1: Enable unrestricted write access to FlexCAN memory."]
    Enable = 1,
}
impl From<Wrmfrz> for bool {
    #[inline(always)]
    fn from(variant: Wrmfrz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WRMFRZ` reader - Write-Access To Memory In Freeze Mode"]
pub type WrmfrzR = crate::BitReader<Wrmfrz>;
impl WrmfrzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wrmfrz {
        match self.bits {
            false => Wrmfrz::Disable,
            true => Wrmfrz::Enable,
        }
    }
    #[doc = "Maintain the write access restrictions."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Wrmfrz::Disable
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Wrmfrz::Enable
    }
}
#[doc = "Field `WRMFRZ` writer - Write-Access To Memory In Freeze Mode"]
pub type WrmfrzW<'a, REG> = crate::BitWriter<'a, REG, Wrmfrz>;
impl<'a, REG> WrmfrzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Maintain the write access restrictions."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Wrmfrz::Disable)
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Wrmfrz::Enable)
    }
}
#[doc = "Error-correction Configuration Register Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecrwre {
    #[doc = "0: Disable update."]
    Disable = 0,
    #[doc = "1: Enable update."]
    Enable = 1,
}
impl From<Ecrwre> for bool {
    #[inline(always)]
    fn from(variant: Ecrwre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECRWRE` reader - Error-correction Configuration Register Write Enable"]
pub type EcrwreR = crate::BitReader<Ecrwre>;
impl EcrwreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecrwre {
        match self.bits {
            false => Ecrwre::Disable,
            true => Ecrwre::Enable,
        }
    }
    #[doc = "Disable update."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ecrwre::Disable
    }
    #[doc = "Enable update."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ecrwre::Enable
    }
}
#[doc = "Field `ECRWRE` writer - Error-correction Configuration Register Write Enable"]
pub type EcrwreW<'a, REG> = crate::BitWriter<'a, REG, Ecrwre>;
impl<'a, REG> EcrwreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable update."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ecrwre::Disable)
    }
    #[doc = "Enable update."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ecrwre::Enable)
    }
}
#[doc = "Bus Off Done Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boffdonemsk {
    #[doc = "0: Bus off done interrupt disabled."]
    Disable = 0,
    #[doc = "1: Bus off done interrupt enabled."]
    Enable = 1,
}
impl From<Boffdonemsk> for bool {
    #[inline(always)]
    fn from(variant: Boffdonemsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOFFDONEMSK` reader - Bus Off Done Interrupt Mask"]
pub type BoffdonemskR = crate::BitReader<Boffdonemsk>;
impl BoffdonemskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Boffdonemsk {
        match self.bits {
            false => Boffdonemsk::Disable,
            true => Boffdonemsk::Enable,
        }
    }
    #[doc = "Bus off done interrupt disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Boffdonemsk::Disable
    }
    #[doc = "Bus off done interrupt enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Boffdonemsk::Enable
    }
}
#[doc = "Field `BOFFDONEMSK` writer - Bus Off Done Interrupt Mask"]
pub type BoffdonemskW<'a, REG> = crate::BitWriter<'a, REG, Boffdonemsk>;
impl<'a, REG> BoffdonemskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bus off done interrupt disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Boffdonemsk::Disable)
    }
    #[doc = "Bus off done interrupt enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Boffdonemsk::Enable)
    }
}
#[doc = "Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrmskFast {
    #[doc = "0: ERRINT_FAST error interrupt disabled."]
    Disable = 0,
    #[doc = "1: ERRINT_FAST error interrupt enabled."]
    Enable = 1,
}
impl From<ErrmskFast> for bool {
    #[inline(always)]
    fn from(variant: ErrmskFast) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRMSK_FAST` reader - Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames"]
pub type ErrmskFastR = crate::BitReader<ErrmskFast>;
impl ErrmskFastR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ErrmskFast {
        match self.bits {
            false => ErrmskFast::Disable,
            true => ErrmskFast::Enable,
        }
    }
    #[doc = "ERRINT_FAST error interrupt disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ErrmskFast::Disable
    }
    #[doc = "ERRINT_FAST error interrupt enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ErrmskFast::Enable
    }
}
#[doc = "Field `ERRMSK_FAST` writer - Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames"]
pub type ErrmskFastW<'a, REG> = crate::BitWriter<'a, REG, ErrmskFast>;
impl<'a, REG> ErrmskFastW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERRINT_FAST error interrupt disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrmskFast::Disable)
    }
    #[doc = "ERRINT_FAST error interrupt enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ErrmskFast::Enable)
    }
}
impl R {
    #[doc = "Bits 6:7 - Time Stamp Capture Point"]
    #[inline(always)]
    pub fn tstampcap(&self) -> TstampcapR {
        TstampcapR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Message Buffer Time Stamp Base"]
    #[inline(always)]
    pub fn mbtsbase(&self) -> MbtsbaseR {
        MbtsbaseR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Edge Filter Disable"]
    #[inline(always)]
    pub fn edfltdis(&self) -> EdfltdisR {
        EdfltdisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ISO CAN FD Enable"]
    #[inline(always)]
    pub fn isocanfden(&self) -> IsocanfdenR {
        IsocanfdenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bit Timing Expansion enable"]
    #[inline(always)]
    pub fn bte(&self) -> BteR {
        BteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Enable"]
    #[inline(always)]
    pub fn prexcen(&self) -> PrexcenR {
        PrexcenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timer Source"]
    #[inline(always)]
    pub fn timer_src(&self) -> TimerSrcR {
        TimerSrcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&self) -> EacenR {
        EacenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&self) -> RrsR {
        RrsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&self) -> MrpR {
        MrpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&self) -> TasdR {
        TasdR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Number Of Legacy Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&self) -> RffnR {
        RffnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Write-Access To Memory In Freeze Mode"]
    #[inline(always)]
    pub fn wrmfrz(&self) -> WrmfrzR {
        WrmfrzR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Error-correction Configuration Register Write Enable"]
    #[inline(always)]
    pub fn ecrwre(&self) -> EcrwreR {
        EcrwreR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Bus Off Done Interrupt Mask"]
    #[inline(always)]
    pub fn boffdonemsk(&self) -> BoffdonemskR {
        BoffdonemskR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames"]
    #[inline(always)]
    pub fn errmsk_fast(&self) -> ErrmskFastR {
        ErrmskFastR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:7 - Time Stamp Capture Point"]
    #[inline(always)]
    pub fn tstampcap(&mut self) -> TstampcapW<Ctrl2Spec> {
        TstampcapW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Message Buffer Time Stamp Base"]
    #[inline(always)]
    pub fn mbtsbase(&mut self) -> MbtsbaseW<Ctrl2Spec> {
        MbtsbaseW::new(self, 8)
    }
    #[doc = "Bit 11 - Edge Filter Disable"]
    #[inline(always)]
    pub fn edfltdis(&mut self) -> EdfltdisW<Ctrl2Spec> {
        EdfltdisW::new(self, 11)
    }
    #[doc = "Bit 12 - ISO CAN FD Enable"]
    #[inline(always)]
    pub fn isocanfden(&mut self) -> IsocanfdenW<Ctrl2Spec> {
        IsocanfdenW::new(self, 12)
    }
    #[doc = "Bit 13 - Bit Timing Expansion enable"]
    #[inline(always)]
    pub fn bte(&mut self) -> BteW<Ctrl2Spec> {
        BteW::new(self, 13)
    }
    #[doc = "Bit 14 - Protocol Exception Enable"]
    #[inline(always)]
    pub fn prexcen(&mut self) -> PrexcenW<Ctrl2Spec> {
        PrexcenW::new(self, 14)
    }
    #[doc = "Bit 15 - Timer Source"]
    #[inline(always)]
    pub fn timer_src(&mut self) -> TimerSrcW<Ctrl2Spec> {
        TimerSrcW::new(self, 15)
    }
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&mut self) -> EacenW<Ctrl2Spec> {
        EacenW::new(self, 16)
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&mut self) -> RrsW<Ctrl2Spec> {
        RrsW::new(self, 17)
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&mut self) -> MrpW<Ctrl2Spec> {
        MrpW::new(self, 18)
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&mut self) -> TasdW<Ctrl2Spec> {
        TasdW::new(self, 19)
    }
    #[doc = "Bits 24:27 - Number Of Legacy Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&mut self) -> RffnW<Ctrl2Spec> {
        RffnW::new(self, 24)
    }
    #[doc = "Bit 28 - Write-Access To Memory In Freeze Mode"]
    #[inline(always)]
    pub fn wrmfrz(&mut self) -> WrmfrzW<Ctrl2Spec> {
        WrmfrzW::new(self, 28)
    }
    #[doc = "Bit 29 - Error-correction Configuration Register Write Enable"]
    #[inline(always)]
    pub fn ecrwre(&mut self) -> EcrwreW<Ctrl2Spec> {
        EcrwreW::new(self, 29)
    }
    #[doc = "Bit 30 - Bus Off Done Interrupt Mask"]
    #[inline(always)]
    pub fn boffdonemsk(&mut self) -> BoffdonemskW<Ctrl2Spec> {
        BoffdonemskW::new(self, 30)
    }
    #[doc = "Bit 31 - Error Interrupt Mask for errors detected in the data phase of fast CAN FD frames"]
    #[inline(always)]
    pub fn errmsk_fast(&mut self) -> ErrmskFastW<Ctrl2Spec> {
        ErrmskFastW::new(self, 31)
    }
}
#[doc = "Control 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl2Spec;
impl crate::RegisterSpec for Ctrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl2::R`](R) reader structure"]
impl crate::Readable for Ctrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl2::W`](W) writer structure"]
impl crate::Writable for Ctrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL2 to value 0x0060_0000"]
impl crate::Resettable for Ctrl2Spec {
    const RESET_VALUE: u32 = 0x0060_0000;
}
