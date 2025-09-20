#[doc = "Register `ERRIDPR` reader"]
pub type R = crate::R<ErridprSpec>;
#[doc = "Register `ERRIDPR` writer"]
pub type W = crate::W<ErridprSpec>;
#[doc = "Field `DFLIP` reader - Data flip pattern"]
pub type DflipR = crate::FieldReader<u32>;
#[doc = "Field `DFLIP` writer - Data flip pattern"]
pub type DflipW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data flip pattern"]
    #[inline(always)]
    pub fn dflip(&self) -> DflipR {
        DflipR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data flip pattern"]
    #[inline(always)]
    pub fn dflip(&mut self) -> DflipW<ErridprSpec> {
        DflipW::new(self, 0)
    }
}
#[doc = "Error Injection Data Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erridpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erridpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErridprSpec;
impl crate::RegisterSpec for ErridprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erridpr::R`](R) reader structure"]
impl crate::Readable for ErridprSpec {}
#[doc = "`write(|w| ..)` method takes [`erridpr::W`](W) writer structure"]
impl crate::Writable for ErridprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRIDPR to value 0"]
impl crate::Resettable for ErridprSpec {}
