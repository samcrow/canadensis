#[doc = "Register `FDCTRL` reader"]
pub type R = crate::R<FdctrlSpec>;
#[doc = "Register `FDCTRL` writer"]
pub type W = crate::W<FdctrlSpec>;
#[doc = "Field `TDCVAL` reader - Transceiver Delay Compensation Value"]
pub type TdcvalR = crate::FieldReader;
#[doc = "Field `TDCOFF` reader - Transceiver Delay Compensation Offset"]
pub type TdcoffR = crate::FieldReader;
#[doc = "Field `TDCOFF` writer - Transceiver Delay Compensation Offset"]
pub type TdcoffW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Transceiver Delay Compensation Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdcfail {
    #[doc = "0: Measured loop delay is in range."]
    InRange = 0,
    #[doc = "1: Measured loop delay is out of range."]
    OutOfRange = 1,
}
impl From<Tdcfail> for bool {
    #[inline(always)]
    fn from(variant: Tdcfail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDCFAIL` reader - Transceiver Delay Compensation Fail"]
pub type TdcfailR = crate::BitReader<Tdcfail>;
impl TdcfailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdcfail {
        match self.bits {
            false => Tdcfail::InRange,
            true => Tdcfail::OutOfRange,
        }
    }
    #[doc = "Measured loop delay is in range."]
    #[inline(always)]
    pub fn is_in_range(&self) -> bool {
        *self == Tdcfail::InRange
    }
    #[doc = "Measured loop delay is out of range."]
    #[inline(always)]
    pub fn is_out_of_range(&self) -> bool {
        *self == Tdcfail::OutOfRange
    }
}
#[doc = "Field `TDCFAIL` writer - Transceiver Delay Compensation Fail"]
pub type TdcfailW<'a, REG> = crate::BitWriter1C<'a, REG, Tdcfail>;
impl<'a, REG> TdcfailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Measured loop delay is in range."]
    #[inline(always)]
    pub fn in_range(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcfail::InRange)
    }
    #[doc = "Measured loop delay is out of range."]
    #[inline(always)]
    pub fn out_of_range(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcfail::OutOfRange)
    }
}
#[doc = "Transceiver Delay Compensation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdcen {
    #[doc = "0: TDC is disabled"]
    Disable = 0,
    #[doc = "1: TDC is enabled"]
    Enable = 1,
}
impl From<Tdcen> for bool {
    #[inline(always)]
    fn from(variant: Tdcen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDCEN` reader - Transceiver Delay Compensation Enable"]
pub type TdcenR = crate::BitReader<Tdcen>;
impl TdcenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdcen {
        match self.bits {
            false => Tdcen::Disable,
            true => Tdcen::Enable,
        }
    }
    #[doc = "TDC is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tdcen::Disable
    }
    #[doc = "TDC is enabled"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Tdcen::Enable
    }
}
#[doc = "Field `TDCEN` writer - Transceiver Delay Compensation Enable"]
pub type TdcenW<'a, REG> = crate::BitWriter<'a, REG, Tdcen>;
impl<'a, REG> TdcenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDC is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcen::Disable)
    }
    #[doc = "TDC is enabled"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Tdcen::Enable)
    }
}
#[doc = "Message Buffer Data Size for Region 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mbdsr {
    #[doc = "0: Selects 8 bytes per message buffer."]
    R8Bytes = 0,
    #[doc = "1: Selects 16 bytes per message buffer."]
    R16Bytes = 1,
    #[doc = "2: Selects 32 bytes per message buffer."]
    R32Bytes = 2,
    #[doc = "3: Selects 64 bytes per message buffer."]
    R64Bytes = 3,
}
impl From<Mbdsr> for u8 {
    #[inline(always)]
    fn from(variant: Mbdsr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mbdsr {
    type Ux = u8;
}
impl crate::IsEnum for Mbdsr {}
#[doc = "Field `MBDSR0` reader - Message Buffer Data Size for Region 0"]
pub type Mbdsr0R = crate::FieldReader<Mbdsr>;
impl Mbdsr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbdsr {
        match self.bits {
            0 => Mbdsr::R8Bytes,
            1 => Mbdsr::R16Bytes,
            2 => Mbdsr::R32Bytes,
            3 => Mbdsr::R64Bytes,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects 8 bytes per message buffer."]
    #[inline(always)]
    pub fn is_r_8_bytes(&self) -> bool {
        *self == Mbdsr::R8Bytes
    }
    #[doc = "Selects 16 bytes per message buffer."]
    #[inline(always)]
    pub fn is_r_16_bytes(&self) -> bool {
        *self == Mbdsr::R16Bytes
    }
    #[doc = "Selects 32 bytes per message buffer."]
    #[inline(always)]
    pub fn is_r_32_bytes(&self) -> bool {
        *self == Mbdsr::R32Bytes
    }
    #[doc = "Selects 64 bytes per message buffer."]
    #[inline(always)]
    pub fn is_r_64_bytes(&self) -> bool {
        *self == Mbdsr::R64Bytes
    }
}
#[doc = "Field `MBDSR0` writer - Message Buffer Data Size for Region 0"]
pub type Mbdsr0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Mbdsr, crate::Safe>;
impl<'a, REG> Mbdsr0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects 8 bytes per message buffer."]
    #[inline(always)]
    pub fn r_8_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Mbdsr::R8Bytes)
    }
    #[doc = "Selects 16 bytes per message buffer."]
    #[inline(always)]
    pub fn r_16_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Mbdsr::R16Bytes)
    }
    #[doc = "Selects 32 bytes per message buffer."]
    #[inline(always)]
    pub fn r_32_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Mbdsr::R32Bytes)
    }
    #[doc = "Selects 64 bytes per message buffer."]
    #[inline(always)]
    pub fn r_64_bytes(self) -> &'a mut crate::W<REG> {
        self.variant(Mbdsr::R64Bytes)
    }
}
#[doc = "Field `MBDSR1` reader - Message Buffer Data Size for Region 1"]
pub use Mbdsr0R as Mbdsr1R;
#[doc = "Field `MBDSR2` reader - Message Buffer Data Size for Region 2"]
pub use Mbdsr0R as Mbdsr2R;
#[doc = "Field `MBDSR3` reader - Message Buffer Data Size for Region 3"]
pub use Mbdsr0R as Mbdsr3R;
#[doc = "Field `MBDSR1` writer - Message Buffer Data Size for Region 1"]
pub use Mbdsr0W as Mbdsr1W;
#[doc = "Field `MBDSR2` writer - Message Buffer Data Size for Region 2"]
pub use Mbdsr0W as Mbdsr2W;
#[doc = "Field `MBDSR3` writer - Message Buffer Data Size for Region 3"]
pub use Mbdsr0W as Mbdsr3W;
#[doc = "Bit Rate Switch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdrate {
    #[doc = "0: Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    Nominal = 0,
    #[doc = "1: Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    BitRateSwitching = 1,
}
impl From<Fdrate> for bool {
    #[inline(always)]
    fn from(variant: Fdrate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDRATE` reader - Bit Rate Switch Enable"]
pub type FdrateR = crate::BitReader<Fdrate>;
impl FdrateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdrate {
        match self.bits {
            false => Fdrate::Nominal,
            true => Fdrate::BitRateSwitching,
        }
    }
    #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    #[inline(always)]
    pub fn is_nominal(&self) -> bool {
        *self == Fdrate::Nominal
    }
    #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    #[inline(always)]
    pub fn is_bit_rate_switching(&self) -> bool {
        *self == Fdrate::BitRateSwitching
    }
}
#[doc = "Field `FDRATE` writer - Bit Rate Switch Enable"]
pub type FdrateW<'a, REG> = crate::BitWriter<'a, REG, Fdrate>;
impl<'a, REG> FdrateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit a frame in nominal rate. The BRS bit in the Tx MB has no effect."]
    #[inline(always)]
    pub fn nominal(self) -> &'a mut crate::W<REG> {
        self.variant(Fdrate::Nominal)
    }
    #[doc = "Transmit a frame with bit rate switching if the BRS bit in the Tx MB is recessive."]
    #[inline(always)]
    pub fn bit_rate_switching(self) -> &'a mut crate::W<REG> {
        self.variant(Fdrate::BitRateSwitching)
    }
}
impl R {
    #[doc = "Bits 0:5 - Transceiver Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcval(&self) -> TdcvalR {
        TdcvalR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdcoff(&self) -> TdcoffR {
        TdcoffR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Transceiver Delay Compensation Fail"]
    #[inline(always)]
    pub fn tdcfail(&self) -> TdcfailR {
        TdcfailR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub fn tdcen(&self) -> TdcenR {
        TdcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Message Buffer Data Size for Region 0"]
    #[inline(always)]
    pub fn mbdsr0(&self) -> Mbdsr0R {
        Mbdsr0R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Message Buffer Data Size for Region 1"]
    #[inline(always)]
    pub fn mbdsr1(&self) -> Mbdsr1R {
        Mbdsr1R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Message Buffer Data Size for Region 2"]
    #[inline(always)]
    pub fn mbdsr2(&self) -> Mbdsr2R {
        Mbdsr2R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 25:26 - Message Buffer Data Size for Region 3"]
    #[inline(always)]
    pub fn mbdsr3(&self) -> Mbdsr3R {
        Mbdsr3R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 31 - Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn fdrate(&self) -> FdrateR {
        FdrateR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - Transceiver Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdcoff(&mut self) -> TdcoffW<FdctrlSpec> {
        TdcoffW::new(self, 8)
    }
    #[doc = "Bit 14 - Transceiver Delay Compensation Fail"]
    #[inline(always)]
    pub fn tdcfail(&mut self) -> TdcfailW<FdctrlSpec> {
        TdcfailW::new(self, 14)
    }
    #[doc = "Bit 15 - Transceiver Delay Compensation Enable"]
    #[inline(always)]
    pub fn tdcen(&mut self) -> TdcenW<FdctrlSpec> {
        TdcenW::new(self, 15)
    }
    #[doc = "Bits 16:17 - Message Buffer Data Size for Region 0"]
    #[inline(always)]
    pub fn mbdsr0(&mut self) -> Mbdsr0W<FdctrlSpec> {
        Mbdsr0W::new(self, 16)
    }
    #[doc = "Bits 19:20 - Message Buffer Data Size for Region 1"]
    #[inline(always)]
    pub fn mbdsr1(&mut self) -> Mbdsr1W<FdctrlSpec> {
        Mbdsr1W::new(self, 19)
    }
    #[doc = "Bits 22:23 - Message Buffer Data Size for Region 2"]
    #[inline(always)]
    pub fn mbdsr2(&mut self) -> Mbdsr2W<FdctrlSpec> {
        Mbdsr2W::new(self, 22)
    }
    #[doc = "Bits 25:26 - Message Buffer Data Size for Region 3"]
    #[inline(always)]
    pub fn mbdsr3(&mut self) -> Mbdsr3W<FdctrlSpec> {
        Mbdsr3W::new(self, 25)
    }
    #[doc = "Bit 31 - Bit Rate Switch Enable"]
    #[inline(always)]
    pub fn fdrate(&mut self) -> FdrateW<FdctrlSpec> {
        FdrateW::new(self, 31)
    }
}
#[doc = "CAN FD Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdctrlSpec;
impl crate::RegisterSpec for FdctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdctrl::R`](R) reader structure"]
impl crate::Readable for FdctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`fdctrl::W`](W) writer structure"]
impl crate::Writable for FdctrlSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x4000;
}
#[doc = "`reset()` method sets FDCTRL to value 0x8000_0100"]
impl crate::Resettable for FdctrlSpec {
    const RESET_VALUE: u32 = 0x8000_0100;
}
