#[doc = "Register `RERRSYNR` reader"]
pub type R = crate::R<RerrsynrSpec>;
#[doc = "Field `SYND0` reader - Error Syndrome For Byte 0 (least significant)"]
pub type Synd0R = crate::FieldReader;
#[doc = "Byte Enabled For Byte 0 (least significant)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Be0 {
    #[doc = "0: The byte was not read."]
    NotRead = 0,
    #[doc = "1: The byte was read."]
    Read = 1,
}
impl From<Be0> for bool {
    #[inline(always)]
    fn from(variant: Be0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BE0` reader - Byte Enabled For Byte 0 (least significant)"]
pub type Be0R = crate::BitReader<Be0>;
impl Be0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Be0 {
        match self.bits {
            false => Be0::NotRead,
            true => Be0::Read,
        }
    }
    #[doc = "The byte was not read."]
    #[inline(always)]
    pub fn is_not_read(&self) -> bool {
        *self == Be0::NotRead
    }
    #[doc = "The byte was read."]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Be0::Read
    }
}
#[doc = "Field `SYND1` reader - Error Syndrome for Byte 1"]
pub type Synd1R = crate::FieldReader;
#[doc = "Byte Enabled For Byte 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Be1 {
    #[doc = "0: The byte was not read."]
    NotRead = 0,
    #[doc = "1: The byte was read."]
    Read = 1,
}
impl From<Be1> for bool {
    #[inline(always)]
    fn from(variant: Be1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BE1` reader - Byte Enabled For Byte 1"]
pub type Be1R = crate::BitReader<Be1>;
impl Be1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Be1 {
        match self.bits {
            false => Be1::NotRead,
            true => Be1::Read,
        }
    }
    #[doc = "The byte was not read."]
    #[inline(always)]
    pub fn is_not_read(&self) -> bool {
        *self == Be1::NotRead
    }
    #[doc = "The byte was read."]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Be1::Read
    }
}
#[doc = "Field `SYND2` reader - Error Syndrome For Byte 2"]
pub type Synd2R = crate::FieldReader;
#[doc = "Byte Enabled For Byte 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Be2 {
    #[doc = "0: The byte was not read."]
    NotRead = 0,
    #[doc = "1: The byte was read."]
    Read = 1,
}
impl From<Be2> for bool {
    #[inline(always)]
    fn from(variant: Be2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BE2` reader - Byte Enabled For Byte 2"]
pub type Be2R = crate::BitReader<Be2>;
impl Be2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Be2 {
        match self.bits {
            false => Be2::NotRead,
            true => Be2::Read,
        }
    }
    #[doc = "The byte was not read."]
    #[inline(always)]
    pub fn is_not_read(&self) -> bool {
        *self == Be2::NotRead
    }
    #[doc = "The byte was read."]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Be2::Read
    }
}
#[doc = "Field `SYND3` reader - Error Syndrome For Byte 3 (most significant)"]
pub type Synd3R = crate::FieldReader;
#[doc = "Byte Enabled For Byte 3 (most significant)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Be3 {
    #[doc = "0: The byte was not read."]
    NotRead = 0,
    #[doc = "1: The byte was read."]
    Read = 1,
}
impl From<Be3> for bool {
    #[inline(always)]
    fn from(variant: Be3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BE3` reader - Byte Enabled For Byte 3 (most significant)"]
pub type Be3R = crate::BitReader<Be3>;
impl Be3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Be3 {
        match self.bits {
            false => Be3::NotRead,
            true => Be3::Read,
        }
    }
    #[doc = "The byte was not read."]
    #[inline(always)]
    pub fn is_not_read(&self) -> bool {
        *self == Be3::NotRead
    }
    #[doc = "The byte was read."]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == Be3::Read
    }
}
impl R {
    #[doc = "Bits 0:4 - Error Syndrome For Byte 0 (least significant)"]
    #[inline(always)]
    pub fn synd0(&self) -> Synd0R {
        Synd0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Byte Enabled For Byte 0 (least significant)"]
    #[inline(always)]
    pub fn be0(&self) -> Be0R {
        Be0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Error Syndrome for Byte 1"]
    #[inline(always)]
    pub fn synd1(&self) -> Synd1R {
        Synd1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Byte Enabled For Byte 1"]
    #[inline(always)]
    pub fn be1(&self) -> Be1R {
        Be1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Error Syndrome For Byte 2"]
    #[inline(always)]
    pub fn synd2(&self) -> Synd2R {
        Synd2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Byte Enabled For Byte 2"]
    #[inline(always)]
    pub fn be2(&self) -> Be2R {
        Be2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Error Syndrome For Byte 3 (most significant)"]
    #[inline(always)]
    pub fn synd3(&self) -> Synd3R {
        Synd3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Byte Enabled For Byte 3 (most significant)"]
    #[inline(always)]
    pub fn be3(&self) -> Be3R {
        Be3R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Error Report Syndrome Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rerrsynr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RerrsynrSpec;
impl crate::RegisterSpec for RerrsynrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rerrsynr::R`](R) reader structure"]
impl crate::Readable for RerrsynrSpec {}
#[doc = "`reset()` method sets RERRSYNR to value 0"]
impl crate::Resettable for RerrsynrSpec {}
