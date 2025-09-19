#[doc = "Register `IMASK2` reader"]
pub type R = crate::R<Imask2Spec>;
#[doc = "Register `IMASK2` writer"]
pub type W = crate::W<Imask2Spec>;
#[doc = "Field `BUF63TO32M` reader - Buffer MBi Mask"]
pub type Buf63to32mR = crate::FieldReader<u32>;
#[doc = "Field `BUF63TO32M` writer - Buffer MBi Mask"]
pub type Buf63to32mW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    pub fn buf63to32m(&self) -> Buf63to32mR {
        Buf63to32mR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MBi Mask"]
    #[inline(always)]
    pub fn buf63to32m(&mut self) -> Buf63to32mW<Imask2Spec> {
        Buf63to32mW::new(self, 0)
    }
}
#[doc = "Interrupt Masks 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imask2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Imask2Spec;
impl crate::RegisterSpec for Imask2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask2::R`](R) reader structure"]
impl crate::Readable for Imask2Spec {}
#[doc = "`write(|w| ..)` method takes [`imask2::W`](W) writer structure"]
impl crate::Writable for Imask2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMASK2 to value 0"]
impl crate::Resettable for Imask2Spec {}
