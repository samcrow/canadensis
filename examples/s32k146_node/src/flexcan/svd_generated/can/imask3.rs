#[doc = "Register `IMASK3` reader"]
pub type R = crate::R<Imask3Spec>;
#[doc = "Register `IMASK3` writer"]
pub type W = crate::W<Imask3Spec>;
#[doc = "Field `BUF95TO64M` reader - Buffer MBi Mask"]
pub type Buf95to64mR = crate::FieldReader<u32>;
#[doc = "Field `BUF95TO64M` writer - Buffer MBi Mask"]
pub type Buf95to64mW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    pub fn buf95to64m(&self) -> Buf95to64mR {
        Buf95to64mR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    pub fn buf95to64m(&mut self) -> Buf95to64mW<Imask3Spec> {
        Buf95to64mW::new(self, 0)
    }
}
#[doc = "Interrupt Masks 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imask3Spec;
impl crate::RegisterSpec for Imask3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask3::R`](R) reader structure"]
impl crate::Readable for Imask3Spec {}
#[doc = "`write(|w| ..)` method takes [`imask3::W`](W) writer structure"]
impl crate::Writable for Imask3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMASK3 to value 0"]
impl crate::Resettable for Imask3Spec {}
