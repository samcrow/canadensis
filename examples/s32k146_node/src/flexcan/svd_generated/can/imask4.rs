#[doc = "Register `IMASK4` reader"]
pub type R = crate::R<Imask4Spec>;
#[doc = "Register `IMASK4` writer"]
pub type W = crate::W<Imask4Spec>;
#[doc = "Field `BUF127TO96M` reader - Buffer MBi Mask"]
pub type Buf127to96mR = crate::FieldReader<u32>;
#[doc = "Field `BUF127TO96M` writer - Buffer MBi Mask"]
pub type Buf127to96mW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    pub fn buf127to96m(&self) -> Buf127to96mR {
        Buf127to96mR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    pub fn buf127to96m(&mut self) -> Buf127to96mW<Imask4Spec> {
        Buf127to96mW::new(self, 0)
    }
}
#[doc = "Interrupt Masks 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imask4Spec;
impl crate::RegisterSpec for Imask4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask4::R`](R) reader structure"]
impl crate::Readable for Imask4Spec {}
#[doc = "`write(|w| ..)` method takes [`imask4::W`](W) writer structure"]
impl crate::Writable for Imask4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMASK4 to value 0"]
impl crate::Resettable for Imask4Spec {}
