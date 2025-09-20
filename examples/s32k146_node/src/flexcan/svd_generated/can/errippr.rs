#[doc = "Register `ERRIPPR` reader"]
pub type R = crate::R<ErripprSpec>;
#[doc = "Register `ERRIPPR` writer"]
pub type W = crate::W<ErripprSpec>;
#[doc = "Field `PFLIP0` reader - Parity Flip Pattern For Byte 0 (Least Significant)"]
pub type Pflip0R = crate::FieldReader;
#[doc = "Field `PFLIP0` writer - Parity Flip Pattern For Byte 0 (Least Significant)"]
pub type Pflip0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PFLIP1` reader - Parity Flip Pattern For Byte 1"]
pub type Pflip1R = crate::FieldReader;
#[doc = "Field `PFLIP1` writer - Parity Flip Pattern For Byte 1"]
pub type Pflip1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PFLIP2` reader - Parity Flip Pattern For Byte 2"]
pub type Pflip2R = crate::FieldReader;
#[doc = "Field `PFLIP2` writer - Parity Flip Pattern For Byte 2"]
pub type Pflip2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PFLIP3` reader - Parity Flip Pattern For Byte 3 (most significant)"]
pub type Pflip3R = crate::FieldReader;
#[doc = "Field `PFLIP3` writer - Parity Flip Pattern For Byte 3 (most significant)"]
pub type Pflip3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Parity Flip Pattern For Byte 0 (Least Significant)"]
    #[inline(always)]
    pub fn pflip0(&self) -> Pflip0R {
        Pflip0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Parity Flip Pattern For Byte 1"]
    #[inline(always)]
    pub fn pflip1(&self) -> Pflip1R {
        Pflip1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Parity Flip Pattern For Byte 2"]
    #[inline(always)]
    pub fn pflip2(&self) -> Pflip2R {
        Pflip2R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Parity Flip Pattern For Byte 3 (most significant)"]
    #[inline(always)]
    pub fn pflip3(&self) -> Pflip3R {
        Pflip3R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Parity Flip Pattern For Byte 0 (Least Significant)"]
    #[inline(always)]
    pub fn pflip0(&mut self) -> Pflip0W<ErripprSpec> {
        Pflip0W::new(self, 0)
    }
    #[doc = "Bits 8:12 - Parity Flip Pattern For Byte 1"]
    #[inline(always)]
    pub fn pflip1(&mut self) -> Pflip1W<ErripprSpec> {
        Pflip1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Parity Flip Pattern For Byte 2"]
    #[inline(always)]
    pub fn pflip2(&mut self) -> Pflip2W<ErripprSpec> {
        Pflip2W::new(self, 16)
    }
    #[doc = "Bits 24:28 - Parity Flip Pattern For Byte 3 (most significant)"]
    #[inline(always)]
    pub fn pflip3(&mut self) -> Pflip3W<ErripprSpec> {
        Pflip3W::new(self, 24)
    }
}
#[doc = "Error Injection Parity Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`errippr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errippr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErripprSpec;
impl crate::RegisterSpec for ErripprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errippr::R`](R) reader structure"]
impl crate::Readable for ErripprSpec {}
#[doc = "`write(|w| ..)` method takes [`errippr::W`](W) writer structure"]
impl crate::Writable for ErripprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRIPPR to value 0"]
impl crate::Resettable for ErripprSpec {}
