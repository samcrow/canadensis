#[doc = "Register `ERRIAR` reader"]
pub type R = crate::R<ErriarSpec>;
#[doc = "Register `ERRIAR` writer"]
pub type W = crate::W<ErriarSpec>;
#[doc = "Field `INJADDR_L` reader - Error Injection Address Low"]
pub type InjaddrLR = crate::FieldReader;
#[doc = "Field `INJADDR_H` reader - Error Injection Address High"]
pub type InjaddrHR = crate::FieldReader<u16>;
#[doc = "Field `INJADDR_H` writer - Error Injection Address High"]
pub type InjaddrHW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:1 - Error Injection Address Low"]
    #[inline(always)]
    pub fn injaddr_l(&self) -> InjaddrLR {
        InjaddrLR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:13 - Error Injection Address High"]
    #[inline(always)]
    pub fn injaddr_h(&self) -> InjaddrHR {
        InjaddrHR::new(((self.bits >> 2) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:13 - Error Injection Address High"]
    #[inline(always)]
    pub fn injaddr_h(&mut self) -> InjaddrHW<ErriarSpec> {
        InjaddrHW::new(self, 2)
    }
}
#[doc = "Error Injection Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`erriar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erriar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErriarSpec;
impl crate::RegisterSpec for ErriarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erriar::R`](R) reader structure"]
impl crate::Readable for ErriarSpec {}
#[doc = "`write(|w| ..)` method takes [`erriar::W`](W) writer structure"]
impl crate::Writable for ErriarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRIAR to value 0"]
impl crate::Resettable for ErriarSpec {}
