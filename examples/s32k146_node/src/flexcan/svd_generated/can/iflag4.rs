#[doc = "Register `IFLAG4` reader"]
pub type R = crate::R<Iflag4Spec>;
#[doc = "Register `IFLAG4` writer"]
pub type W = crate::W<Iflag4Spec>;
#[doc = "Field `BUF127TO96` reader - Buffer MBi Interrupt"]
pub type Buf127to96R = crate::FieldReader<u32>;
#[doc = "Field `BUF127TO96` writer - Buffer MBi Interrupt"]
pub type Buf127to96W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf127to96(&self) -> Buf127to96R {
        Buf127to96R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer MBi Interrupt"]
    #[inline(always)]
    pub fn buf127to96(&mut self) -> Buf127to96W<Iflag4Spec> {
        Buf127to96W::new(self, 0)
    }
}
#[doc = "Interrupt Flags 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iflag4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iflag4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iflag4Spec;
impl crate::RegisterSpec for Iflag4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iflag4::R`](R) reader structure"]
impl crate::Readable for Iflag4Spec {}
#[doc = "`write(|w| ..)` method takes [`iflag4::W`](W) writer structure"]
impl crate::Writable for Iflag4Spec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets IFLAG4 to value 0"]
impl crate::Resettable for Iflag4Spec {}
