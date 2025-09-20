#[doc = "Register `RERRAR` reader"]
pub type R = crate::R<RerrarSpec>;
#[doc = "Field `ERRADDR` reader - Address Where Error Detected"]
pub type ErraddrR = crate::FieldReader<u16>;
#[doc = "Field `SAID` reader - SAID"]
pub type SaidR = crate::FieldReader;
#[doc = "Non-Correctable Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nce {
    #[doc = "0: Reporting a correctable error"]
    Correctable = 0,
    #[doc = "1: Reporting a non-correctable error"]
    NonCorrectable = 1,
}
impl From<Nce> for bool {
    #[inline(always)]
    fn from(variant: Nce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NCE` reader - Non-Correctable Error"]
pub type NceR = crate::BitReader<Nce>;
impl NceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nce {
        match self.bits {
            false => Nce::Correctable,
            true => Nce::NonCorrectable,
        }
    }
    #[doc = "Reporting a correctable error"]
    #[inline(always)]
    pub fn is_correctable(&self) -> bool {
        *self == Nce::Correctable
    }
    #[doc = "Reporting a non-correctable error"]
    #[inline(always)]
    pub fn is_non_correctable(&self) -> bool {
        *self == Nce::NonCorrectable
    }
}
impl R {
    #[doc = "Bits 0:13 - Address Where Error Detected"]
    #[inline(always)]
    pub fn erraddr(&self) -> ErraddrR {
        ErraddrR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:18 - SAID"]
    #[inline(always)]
    pub fn said(&self) -> SaidR {
        SaidR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 24 - Non-Correctable Error"]
    #[inline(always)]
    pub fn nce(&self) -> NceR {
        NceR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Error Report Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rerrar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RerrarSpec;
impl crate::RegisterSpec for RerrarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rerrar::R`](R) reader structure"]
impl crate::Readable for RerrarSpec {}
#[doc = "`reset()` method sets RERRAR to value 0"]
impl crate::Resettable for RerrarSpec {}
