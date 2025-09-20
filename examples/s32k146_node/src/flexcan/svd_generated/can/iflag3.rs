#[doc = "Register `IFLAG3` reader"]
pub type R = crate::R<Iflag3Spec>;
#[doc = "Register `IFLAG3` writer"]
pub type W = crate::W<Iflag3Spec>;
#[doc = "Field `BUF95TO64` reader - Buffer MBi Interrupt"]
pub type Buf95to64R = crate::FieldReader<u32>;
#[doc = "Field `BUF95TO64` writer - Buffer MBi Interrupt"]
pub type Buf95to64W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf95to64(&self) -> Buf95to64R {
        Buf95to64R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf95to64(&mut self) -> Buf95to64W<Iflag3Spec> {
        Buf95to64W::new(self, 0)
    }
}
#[doc = "Interrupt Flags 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iflag3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iflag3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iflag3Spec;
impl crate::RegisterSpec for Iflag3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iflag3::R`](R) reader structure"]
impl crate::Readable for Iflag3Spec {}
#[doc = "`write(|w| ..)` method takes [`iflag3::W`](W) writer structure"]
impl crate::Writable for Iflag3Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets IFLAG3 to value 0"]
impl crate::Resettable for Iflag3Spec {}
