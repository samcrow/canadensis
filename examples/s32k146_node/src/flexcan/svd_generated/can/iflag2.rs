#[doc = "Register `IFLAG2` reader"]
pub type R = crate::R<Iflag2Spec>;
#[doc = "Register `IFLAG2` writer"]
pub type W = crate::W<Iflag2Spec>;
#[doc = "Field `BUF63TO32I` reader - Buffer MBi Interrupt"]
pub type Buf63to32iR = crate::FieldReader<u32>;
#[doc = "Field `BUF63TO32I` writer - Buffer MBi Interrupt"]
pub type Buf63to32iW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf63to32i(&self) -> Buf63to32iR {
        Buf63to32iR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf63to32i(&mut self) -> Buf63to32iW<Iflag2Spec> {
        Buf63to32iW::new(self, 0)
    }
}
#[doc = "Interrupt Flags 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iflag2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iflag2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iflag2Spec;
impl crate::RegisterSpec for Iflag2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iflag2::R`](R) reader structure"]
impl crate::Readable for Iflag2Spec {}
#[doc = "`write(|w| ..)` method takes [`iflag2::W`](W) writer structure"]
impl crate::Writable for Iflag2Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets IFLAG2 to value 0"]
impl crate::Resettable for Iflag2Spec {}
