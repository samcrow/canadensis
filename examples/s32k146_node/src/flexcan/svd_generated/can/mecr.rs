#[doc = "Register `MECR` reader"]
pub type R = crate::R<MecrSpec>;
#[doc = "Register `MECR` writer"]
pub type W = crate::W<MecrSpec>;
#[doc = "Non-Correctable Errors In FlexCAN Access Put Device In Freeze Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ncefafrz {
    #[doc = "0: Keep normal operation."]
    Normal = 0,
    #[doc = "1: Put FlexCAN in Freeze mode (see section \"Freeze mode\")."]
    Freeze = 1,
}
impl From<Ncefafrz> for bool {
    #[inline(always)]
    fn from(variant: Ncefafrz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCEFAFRZ` reader - Non-Correctable Errors In FlexCAN Access Put Device In Freeze Mode"]
pub type NcefafrzR = crate::BitReader<Ncefafrz>;
impl NcefafrzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ncefafrz {
        match self.bits {
            false => Ncefafrz::Normal,
            true => Ncefafrz::Freeze,
        }
    }
    #[doc = "Keep normal operation."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Ncefafrz::Normal
    }
    #[doc = "Put FlexCAN in Freeze mode (see section \"Freeze mode\")."]
    #[inline(always)]
    pub fn is_freeze(&self) -> bool {
        *self == Ncefafrz::Freeze
    }
}
#[doc = "Field `NCEFAFRZ` writer - Non-Correctable Errors In FlexCAN Access Put Device In Freeze Mode"]
pub type NcefafrzW<'a, REG> = crate::BitWriter<'a, REG, Ncefafrz>;
impl<'a, REG> NcefafrzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Keep normal operation."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Ncefafrz::Normal)
    }
    #[doc = "Put FlexCAN in Freeze mode (see section \"Freeze mode\")."]
    #[inline(always)]
    pub fn freeze(self) -> &'a mut crate::W<REG> {
        self.variant(Ncefafrz::Freeze)
    }
}
#[doc = "Error Correction Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eccdis {
    #[doc = "0: Enable memory error correction."]
    Enable = 0,
    #[doc = "1: Disable memory error correction."]
    Disable = 1,
}
impl From<Eccdis> for bool {
    #[inline(always)]
    fn from(variant: Eccdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECCDIS` reader - Error Correction Disable"]
pub type EccdisR = crate::BitReader<Eccdis>;
impl EccdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eccdis {
        match self.bits {
            false => Eccdis::Enable,
            true => Eccdis::Disable,
        }
    }
    #[doc = "Enable memory error correction."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Eccdis::Enable
    }
    #[doc = "Disable memory error correction."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Eccdis::Disable
    }
}
#[doc = "Field `ECCDIS` writer - Error Correction Disable"]
pub type EccdisW<'a, REG> = crate::BitWriter<'a, REG, Eccdis>;
impl<'a, REG> EccdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable memory error correction."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Eccdis::Enable)
    }
    #[doc = "Disable memory error correction."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Eccdis::Disable)
    }
}
#[doc = "Error Report Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rerrdis {
    #[doc = "0: Enable updates of the error report registers."]
    Enable = 0,
    #[doc = "1: Disable updates of the error report registers."]
    Disable = 1,
}
impl From<Rerrdis> for bool {
    #[inline(always)]
    fn from(variant: Rerrdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RERRDIS` reader - Error Report Disable"]
pub type RerrdisR = crate::BitReader<Rerrdis>;
impl RerrdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rerrdis {
        match self.bits {
            false => Rerrdis::Enable,
            true => Rerrdis::Disable,
        }
    }
    #[doc = "Enable updates of the error report registers."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rerrdis::Enable
    }
    #[doc = "Disable updates of the error report registers."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rerrdis::Disable
    }
}
#[doc = "Field `RERRDIS` writer - Error Report Disable"]
pub type RerrdisW<'a, REG> = crate::BitWriter<'a, REG, Rerrdis>;
impl<'a, REG> RerrdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable updates of the error report registers."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rerrdis::Enable)
    }
    #[doc = "Disable updates of the error report registers."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rerrdis::Disable)
    }
}
#[doc = "Extended Error Injection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exterrie {
    #[doc = "0: Error injection is applied only to the 32-bit word."]
    Inject32Bit = 0,
    #[doc = "1: Error injection is applied to the 64-bit word."]
    Inject64Bit = 1,
}
impl From<Exterrie> for bool {
    #[inline(always)]
    fn from(variant: Exterrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTERRIE` reader - Extended Error Injection Enable"]
pub type ExterrieR = crate::BitReader<Exterrie>;
impl ExterrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Exterrie {
        match self.bits {
            false => Exterrie::Inject32Bit,
            true => Exterrie::Inject64Bit,
        }
    }
    #[doc = "Error injection is applied only to the 32-bit word."]
    #[inline(always)]
    pub fn is_inject_32_bit(&self) -> bool {
        *self == Exterrie::Inject32Bit
    }
    #[doc = "Error injection is applied to the 64-bit word."]
    #[inline(always)]
    pub fn is_inject_64_bit(&self) -> bool {
        *self == Exterrie::Inject64Bit
    }
}
#[doc = "Field `EXTERRIE` writer - Extended Error Injection Enable"]
pub type ExterrieW<'a, REG> = crate::BitWriter<'a, REG, Exterrie>;
impl<'a, REG> ExterrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error injection is applied only to the 32-bit word."]
    #[inline(always)]
    pub fn inject_32_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Exterrie::Inject32Bit)
    }
    #[doc = "Error injection is applied to the 64-bit word."]
    #[inline(always)]
    pub fn inject_64_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Exterrie::Inject64Bit)
    }
}
#[doc = "FlexCAN Access Error Injection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Faerrie {
    #[doc = "0: Injection is disabled."]
    Disable = 0,
    #[doc = "1: Injection is enabled."]
    Enable = 1,
}
impl From<Faerrie> for bool {
    #[inline(always)]
    fn from(variant: Faerrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAERRIE` reader - FlexCAN Access Error Injection Enable"]
pub type FaerrieR = crate::BitReader<Faerrie>;
impl FaerrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Faerrie {
        match self.bits {
            false => Faerrie::Disable,
            true => Faerrie::Enable,
        }
    }
    #[doc = "Injection is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Faerrie::Disable
    }
    #[doc = "Injection is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Faerrie::Enable
    }
}
#[doc = "Field `FAERRIE` writer - FlexCAN Access Error Injection Enable"]
pub type FaerrieW<'a, REG> = crate::BitWriter<'a, REG, Faerrie>;
impl<'a, REG> FaerrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injection is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Faerrie::Disable)
    }
    #[doc = "Injection is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Faerrie::Enable)
    }
}
#[doc = "Host Access Error Injection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Haerrie {
    #[doc = "0: Injection is disabled."]
    Disable = 0,
    #[doc = "1: Injection is enabled."]
    Enable = 1,
}
impl From<Haerrie> for bool {
    #[inline(always)]
    fn from(variant: Haerrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HAERRIE` reader - Host Access Error Injection Enable"]
pub type HaerrieR = crate::BitReader<Haerrie>;
impl HaerrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Haerrie {
        match self.bits {
            false => Haerrie::Disable,
            true => Haerrie::Enable,
        }
    }
    #[doc = "Injection is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Haerrie::Disable
    }
    #[doc = "Injection is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Haerrie::Enable
    }
}
#[doc = "Field `HAERRIE` writer - Host Access Error Injection Enable"]
pub type HaerrieW<'a, REG> = crate::BitWriter<'a, REG, Haerrie>;
impl<'a, REG> HaerrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Injection is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Haerrie::Disable)
    }
    #[doc = "Injection is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Haerrie::Enable)
    }
}
#[doc = "Correctable Errors Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CeiMsk {
    #[doc = "0: Interrupt is disabled."]
    Disable = 0,
    #[doc = "1: Interrupt is enabled."]
    Enable = 1,
}
impl From<CeiMsk> for bool {
    #[inline(always)]
    fn from(variant: CeiMsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEI_MSK` reader - Correctable Errors Interrupt Mask"]
pub type CeiMskR = crate::BitReader<CeiMsk>;
impl CeiMskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CeiMsk {
        match self.bits {
            false => CeiMsk::Disable,
            true => CeiMsk::Enable,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CeiMsk::Disable
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CeiMsk::Enable
    }
}
#[doc = "Field `CEI_MSK` writer - Correctable Errors Interrupt Mask"]
pub type CeiMskW<'a, REG> = crate::BitWriter<'a, REG, CeiMsk>;
impl<'a, REG> CeiMskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CeiMsk::Disable)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CeiMsk::Enable)
    }
}
#[doc = "FlexCAN Access With Non-Correctable Errors Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FanceiMsk {
    #[doc = "0: Interrupt is disabled."]
    Disable = 0,
    #[doc = "1: Interrupt is enabled."]
    Enable = 1,
}
impl From<FanceiMsk> for bool {
    #[inline(always)]
    fn from(variant: FanceiMsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FANCEI_MSK` reader - FlexCAN Access With Non-Correctable Errors Interrupt Mask"]
pub type FanceiMskR = crate::BitReader<FanceiMsk>;
impl FanceiMskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FanceiMsk {
        match self.bits {
            false => FanceiMsk::Disable,
            true => FanceiMsk::Enable,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FanceiMsk::Disable
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FanceiMsk::Enable
    }
}
#[doc = "Field `FANCEI_MSK` writer - FlexCAN Access With Non-Correctable Errors Interrupt Mask"]
pub type FanceiMskW<'a, REG> = crate::BitWriter<'a, REG, FanceiMsk>;
impl<'a, REG> FanceiMskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(FanceiMsk::Disable)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(FanceiMsk::Enable)
    }
}
#[doc = "Host Access With Non-Correctable Errors Interrupt Mask\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HanceiMsk {
    #[doc = "0: Interrupt is disabled."]
    Disable = 0,
    #[doc = "1: Interrupt is enabled."]
    Enable = 1,
}
impl From<HanceiMsk> for bool {
    #[inline(always)]
    fn from(variant: HanceiMsk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HANCEI_MSK` reader - Host Access With Non-Correctable Errors Interrupt Mask"]
pub type HanceiMskR = crate::BitReader<HanceiMsk>;
impl HanceiMskR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HanceiMsk {
        match self.bits {
            false => HanceiMsk::Disable,
            true => HanceiMsk::Enable,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == HanceiMsk::Disable
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == HanceiMsk::Enable
    }
}
#[doc = "Field `HANCEI_MSK` writer - Host Access With Non-Correctable Errors Interrupt Mask"]
pub type HanceiMskW<'a, REG> = crate::BitWriter<'a, REG, HanceiMsk>;
impl<'a, REG> HanceiMskW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(HanceiMsk::Disable)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(HanceiMsk::Enable)
    }
}
#[doc = "Error Configuration Register Write Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ecrwrdis {
    #[doc = "0: Write is enabled."]
    Enable = 0,
    #[doc = "1: Write is disabled."]
    Disable = 1,
}
impl From<Ecrwrdis> for bool {
    #[inline(always)]
    fn from(variant: Ecrwrdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ECRWRDIS` reader - Error Configuration Register Write Disable"]
pub type EcrwrdisR = crate::BitReader<Ecrwrdis>;
impl EcrwrdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ecrwrdis {
        match self.bits {
            false => Ecrwrdis::Enable,
            true => Ecrwrdis::Disable,
        }
    }
    #[doc = "Write is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ecrwrdis::Enable
    }
    #[doc = "Write is disabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ecrwrdis::Disable
    }
}
#[doc = "Field `ECRWRDIS` writer - Error Configuration Register Write Disable"]
pub type EcrwrdisW<'a, REG> = crate::BitWriter<'a, REG, Ecrwrdis>;
impl<'a, REG> EcrwrdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ecrwrdis::Enable)
    }
    #[doc = "Write is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ecrwrdis::Disable)
    }
}
impl R {
    #[doc = "Bit 7 - Non-Correctable Errors In FlexCAN Access Put Device In Freeze Mode"]
    #[inline(always)]
    pub fn ncefafrz(&self) -> NcefafrzR {
        NcefafrzR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Error Correction Disable"]
    #[inline(always)]
    pub fn eccdis(&self) -> EccdisR {
        EccdisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error Report Disable"]
    #[inline(always)]
    pub fn rerrdis(&self) -> RerrdisR {
        RerrdisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 13 - Extended Error Injection Enable"]
    #[inline(always)]
    pub fn exterrie(&self) -> ExterrieR {
        ExterrieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FlexCAN Access Error Injection Enable"]
    #[inline(always)]
    pub fn faerrie(&self) -> FaerrieR {
        FaerrieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Host Access Error Injection Enable"]
    #[inline(always)]
    pub fn haerrie(&self) -> HaerrieR {
        HaerrieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Correctable Errors Interrupt Mask"]
    #[inline(always)]
    pub fn cei_msk(&self) -> CeiMskR {
        CeiMskR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - FlexCAN Access With Non-Correctable Errors Interrupt Mask"]
    #[inline(always)]
    pub fn fancei_msk(&self) -> FanceiMskR {
        FanceiMskR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Host Access With Non-Correctable Errors Interrupt Mask"]
    #[inline(always)]
    pub fn hancei_msk(&self) -> HanceiMskR {
        HanceiMskR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - Error Configuration Register Write Disable"]
    #[inline(always)]
    pub fn ecrwrdis(&self) -> EcrwrdisR {
        EcrwrdisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Non-Correctable Errors In FlexCAN Access Put Device In Freeze Mode"]
    #[inline(always)]
    pub fn ncefafrz(&mut self) -> NcefafrzW<MecrSpec> {
        NcefafrzW::new(self, 7)
    }
    #[doc = "Bit 8 - Error Correction Disable"]
    #[inline(always)]
    pub fn eccdis(&mut self) -> EccdisW<MecrSpec> {
        EccdisW::new(self, 8)
    }
    #[doc = "Bit 9 - Error Report Disable"]
    #[inline(always)]
    pub fn rerrdis(&mut self) -> RerrdisW<MecrSpec> {
        RerrdisW::new(self, 9)
    }
    #[doc = "Bit 13 - Extended Error Injection Enable"]
    #[inline(always)]
    pub fn exterrie(&mut self) -> ExterrieW<MecrSpec> {
        ExterrieW::new(self, 13)
    }
    #[doc = "Bit 14 - FlexCAN Access Error Injection Enable"]
    #[inline(always)]
    pub fn faerrie(&mut self) -> FaerrieW<MecrSpec> {
        FaerrieW::new(self, 14)
    }
    #[doc = "Bit 15 - Host Access Error Injection Enable"]
    #[inline(always)]
    pub fn haerrie(&mut self) -> HaerrieW<MecrSpec> {
        HaerrieW::new(self, 15)
    }
    #[doc = "Bit 16 - Correctable Errors Interrupt Mask"]
    #[inline(always)]
    pub fn cei_msk(&mut self) -> CeiMskW<MecrSpec> {
        CeiMskW::new(self, 16)
    }
    #[doc = "Bit 18 - FlexCAN Access With Non-Correctable Errors Interrupt Mask"]
    #[inline(always)]
    pub fn fancei_msk(&mut self) -> FanceiMskW<MecrSpec> {
        FanceiMskW::new(self, 18)
    }
    #[doc = "Bit 19 - Host Access With Non-Correctable Errors Interrupt Mask"]
    #[inline(always)]
    pub fn hancei_msk(&mut self) -> HanceiMskW<MecrSpec> {
        HanceiMskW::new(self, 19)
    }
    #[doc = "Bit 31 - Error Configuration Register Write Disable"]
    #[inline(always)]
    pub fn ecrwrdis(&mut self) -> EcrwrdisW<MecrSpec> {
        EcrwrdisW::new(self, 31)
    }
}
#[doc = "Memory Error Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MecrSpec;
impl crate::RegisterSpec for MecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mecr::R`](R) reader structure"]
impl crate::Readable for MecrSpec {}
#[doc = "`write(|w| ..)` method takes [`mecr::W`](W) writer structure"]
impl crate::Writable for MecrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MECR to value 0x800c_0080"]
impl crate::Resettable for MecrSpec {
    const RESET_VALUE: u32 = 0x800c_0080;
}
