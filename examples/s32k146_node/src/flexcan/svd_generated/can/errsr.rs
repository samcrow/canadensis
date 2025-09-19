#[doc = "Register `ERRSR` reader"]
pub type R = crate::R<ErrsrSpec>;
#[doc = "Register `ERRSR` writer"]
pub type W = crate::W<ErrsrSpec>;
#[doc = "Correctable Error Interrupt Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceiof {
    #[doc = "0: No overrun on correctable errors"]
    NoOverrun = 0,
    #[doc = "1: Overrun on correctable errors"]
    Overrun = 1,
}
impl From<Ceiof> for bool {
    #[inline(always)]
    fn from(variant: Ceiof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIOF` reader - Correctable Error Interrupt Overrun Flag"]
pub type CeiofR = crate::BitReader<Ceiof>;
impl CeiofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceiof {
        match self.bits {
            false => Ceiof::NoOverrun,
            true => Ceiof::Overrun,
        }
    }
    #[doc = "No overrun on correctable errors"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == Ceiof::NoOverrun
    }
    #[doc = "Overrun on correctable errors"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == Ceiof::Overrun
    }
}
#[doc = "Field `CEIOF` writer - Correctable Error Interrupt Overrun Flag"]
pub type CeiofW<'a, REG> = crate::BitWriter1C<'a, REG, Ceiof>;
impl<'a, REG> CeiofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun on correctable errors"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiof::NoOverrun)
    }
    #[doc = "Overrun on correctable errors"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Ceiof::Overrun)
    }
}
#[doc = "FlexCAN Access With Non-Correctable Error Interrupt Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fanceiof {
    #[doc = "0: No overrun on non-correctable errors in FlexCAN access"]
    NoOverrun = 0,
    #[doc = "1: Overrun on non-correctable errors in FlexCAN access"]
    Overrun = 1,
}
impl From<Fanceiof> for bool {
    #[inline(always)]
    fn from(variant: Fanceiof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FANCEIOF` reader - FlexCAN Access With Non-Correctable Error Interrupt Overrun Flag"]
pub type FanceiofR = crate::BitReader<Fanceiof>;
impl FanceiofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fanceiof {
        match self.bits {
            false => Fanceiof::NoOverrun,
            true => Fanceiof::Overrun,
        }
    }
    #[doc = "No overrun on non-correctable errors in FlexCAN access"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == Fanceiof::NoOverrun
    }
    #[doc = "Overrun on non-correctable errors in FlexCAN access"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == Fanceiof::Overrun
    }
}
#[doc = "Field `FANCEIOF` writer - FlexCAN Access With Non-Correctable Error Interrupt Overrun Flag"]
pub type FanceiofW<'a, REG> = crate::BitWriter1C<'a, REG, Fanceiof>;
impl<'a, REG> FanceiofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun on non-correctable errors in FlexCAN access"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Fanceiof::NoOverrun)
    }
    #[doc = "Overrun on non-correctable errors in FlexCAN access"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Fanceiof::Overrun)
    }
}
#[doc = "Host Access With Non-Correctable Error Interrupt Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hanceiof {
    #[doc = "0: No overrun on non-correctable errors in host access"]
    NoOverrun = 0,
    #[doc = "1: Overrun on non-correctable errors in host access"]
    Overrun = 1,
}
impl From<Hanceiof> for bool {
    #[inline(always)]
    fn from(variant: Hanceiof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HANCEIOF` reader - Host Access With Non-Correctable Error Interrupt Overrun Flag"]
pub type HanceiofR = crate::BitReader<Hanceiof>;
impl HanceiofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hanceiof {
        match self.bits {
            false => Hanceiof::NoOverrun,
            true => Hanceiof::Overrun,
        }
    }
    #[doc = "No overrun on non-correctable errors in host access"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == Hanceiof::NoOverrun
    }
    #[doc = "Overrun on non-correctable errors in host access"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == Hanceiof::Overrun
    }
}
#[doc = "Field `HANCEIOF` writer - Host Access With Non-Correctable Error Interrupt Overrun Flag"]
pub type HanceiofW<'a, REG> = crate::BitWriter1C<'a, REG, Hanceiof>;
impl<'a, REG> HanceiofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun on non-correctable errors in host access"]
    #[inline(always)]
    pub fn no_overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Hanceiof::NoOverrun)
    }
    #[doc = "Overrun on non-correctable errors in host access"]
    #[inline(always)]
    pub fn overrun(self) -> &'a mut crate::W<REG> {
        self.variant(Hanceiof::Overrun)
    }
}
#[doc = "Correctable Error Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ceif {
    #[doc = "0: No correctable errors were detected so far."]
    NoErrors = 0,
    #[doc = "1: A correctable error was detected."]
    Errors = 1,
}
impl From<Ceif> for bool {
    #[inline(always)]
    fn from(variant: Ceif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEIF` reader - Correctable Error Interrupt Flag"]
pub type CeifR = crate::BitReader<Ceif>;
impl CeifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ceif {
        match self.bits {
            false => Ceif::NoErrors,
            true => Ceif::Errors,
        }
    }
    #[doc = "No correctable errors were detected so far."]
    #[inline(always)]
    pub fn is_no_errors(&self) -> bool {
        *self == Ceif::NoErrors
    }
    #[doc = "A correctable error was detected."]
    #[inline(always)]
    pub fn is_errors(&self) -> bool {
        *self == Ceif::Errors
    }
}
#[doc = "Field `CEIF` writer - Correctable Error Interrupt Flag"]
pub type CeifW<'a, REG> = crate::BitWriter1C<'a, REG, Ceif>;
impl<'a, REG> CeifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No correctable errors were detected so far."]
    #[inline(always)]
    pub fn no_errors(self) -> &'a mut crate::W<REG> {
        self.variant(Ceif::NoErrors)
    }
    #[doc = "A correctable error was detected."]
    #[inline(always)]
    pub fn errors(self) -> &'a mut crate::W<REG> {
        self.variant(Ceif::Errors)
    }
}
#[doc = "FlexCAN Access With Non-Correctable Error Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fanceif {
    #[doc = "0: No non-correctable errors were detected in FlexCAN accesses so far."]
    NotFound = 0,
    #[doc = "1: A non-correctable error was detected in a FlexCAN access."]
    Found = 1,
}
impl From<Fanceif> for bool {
    #[inline(always)]
    fn from(variant: Fanceif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FANCEIF` reader - FlexCAN Access With Non-Correctable Error Interrupt Flag"]
pub type FanceifR = crate::BitReader<Fanceif>;
impl FanceifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fanceif {
        match self.bits {
            false => Fanceif::NotFound,
            true => Fanceif::Found,
        }
    }
    #[doc = "No non-correctable errors were detected in FlexCAN accesses so far."]
    #[inline(always)]
    pub fn is_not_found(&self) -> bool {
        *self == Fanceif::NotFound
    }
    #[doc = "A non-correctable error was detected in a FlexCAN access."]
    #[inline(always)]
    pub fn is_found(&self) -> bool {
        *self == Fanceif::Found
    }
}
#[doc = "Field `FANCEIF` writer - FlexCAN Access With Non-Correctable Error Interrupt Flag"]
pub type FanceifW<'a, REG> = crate::BitWriter1C<'a, REG, Fanceif>;
impl<'a, REG> FanceifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No non-correctable errors were detected in FlexCAN accesses so far."]
    #[inline(always)]
    pub fn not_found(self) -> &'a mut crate::W<REG> {
        self.variant(Fanceif::NotFound)
    }
    #[doc = "A non-correctable error was detected in a FlexCAN access."]
    #[inline(always)]
    pub fn found(self) -> &'a mut crate::W<REG> {
        self.variant(Fanceif::Found)
    }
}
#[doc = "Host Access With Non-Correctable Error Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hanceif {
    #[doc = "0: No non-correctable errors were detected in host accesses so far."]
    NotFound = 0,
    #[doc = "1: A non-correctable error was detected in a host access."]
    Found = 1,
}
impl From<Hanceif> for bool {
    #[inline(always)]
    fn from(variant: Hanceif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HANCEIF` reader - Host Access With Non-Correctable Error Interrupt Flag"]
pub type HanceifR = crate::BitReader<Hanceif>;
impl HanceifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hanceif {
        match self.bits {
            false => Hanceif::NotFound,
            true => Hanceif::Found,
        }
    }
    #[doc = "No non-correctable errors were detected in host accesses so far."]
    #[inline(always)]
    pub fn is_not_found(&self) -> bool {
        *self == Hanceif::NotFound
    }
    #[doc = "A non-correctable error was detected in a host access."]
    #[inline(always)]
    pub fn is_found(&self) -> bool {
        *self == Hanceif::Found
    }
}
#[doc = "Field `HANCEIF` writer - Host Access With Non-Correctable Error Interrupt Flag"]
pub type HanceifW<'a, REG> = crate::BitWriter1C<'a, REG, Hanceif>;
impl<'a, REG> HanceifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No non-correctable errors were detected in host accesses so far."]
    #[inline(always)]
    pub fn not_found(self) -> &'a mut crate::W<REG> {
        self.variant(Hanceif::NotFound)
    }
    #[doc = "A non-correctable error was detected in a host access."]
    #[inline(always)]
    pub fn found(self) -> &'a mut crate::W<REG> {
        self.variant(Hanceif::Found)
    }
}
impl R {
    #[doc = "Bit 0 - Correctable Error Interrupt Overrun Flag"]
    #[inline(always)]
    pub fn ceiof(&self) -> CeiofR {
        CeiofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - FlexCAN Access With Non-Correctable Error Interrupt Overrun Flag"]
    #[inline(always)]
    pub fn fanceiof(&self) -> FanceiofR {
        FanceiofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Host Access With Non-Correctable Error Interrupt Overrun Flag"]
    #[inline(always)]
    pub fn hanceiof(&self) -> HanceiofR {
        HanceiofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Correctable Error Interrupt Flag"]
    #[inline(always)]
    pub fn ceif(&self) -> CeifR {
        CeifR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - FlexCAN Access With Non-Correctable Error Interrupt Flag"]
    #[inline(always)]
    pub fn fanceif(&self) -> FanceifR {
        FanceifR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Host Access With Non-Correctable Error Interrupt Flag"]
    #[inline(always)]
    pub fn hanceif(&self) -> HanceifR {
        HanceifR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Correctable Error Interrupt Overrun Flag"]
    #[inline(always)]
    pub fn ceiof(&mut self) -> CeiofW<ErrsrSpec> {
        CeiofW::new(self, 0)
    }
    #[doc = "Bit 2 - FlexCAN Access With Non-Correctable Error Interrupt Overrun Flag"]
    #[inline(always)]
    pub fn fanceiof(&mut self) -> FanceiofW<ErrsrSpec> {
        FanceiofW::new(self, 2)
    }
    #[doc = "Bit 3 - Host Access With Non-Correctable Error Interrupt Overrun Flag"]
    #[inline(always)]
    pub fn hanceiof(&mut self) -> HanceiofW<ErrsrSpec> {
        HanceiofW::new(self, 3)
    }
    #[doc = "Bit 16 - Correctable Error Interrupt Flag"]
    #[inline(always)]
    pub fn ceif(&mut self) -> CeifW<ErrsrSpec> {
        CeifW::new(self, 16)
    }
    #[doc = "Bit 18 - FlexCAN Access With Non-Correctable Error Interrupt Flag"]
    #[inline(always)]
    pub fn fanceif(&mut self) -> FanceifW<ErrsrSpec> {
        FanceifW::new(self, 18)
    }
    #[doc = "Bit 19 - Host Access With Non-Correctable Error Interrupt Flag"]
    #[inline(always)]
    pub fn hanceif(&mut self) -> HanceifW<ErrsrSpec> {
        HanceifW::new(self, 19)
    }
}
#[doc = "Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrsrSpec;
impl crate::RegisterSpec for ErrsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errsr::R`](R) reader structure"]
impl crate::Readable for ErrsrSpec {}
#[doc = "`write(|w| ..)` method takes [`errsr::W`](W) writer structure"]
impl crate::Writable for ErrsrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000d_000d;
}
#[doc = "`reset()` method sets ERRSR to value 0"]
impl crate::Resettable for ErrsrSpec {}
