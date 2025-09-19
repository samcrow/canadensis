#[doc = "Register `HR_TIME_STAMP[%s]` reader"]
pub type R = crate::R<HrTimeStampSpec>;
#[doc = "Register `HR_TIME_STAMP[%s]` writer"]
pub type W = crate::W<HrTimeStampSpec>;
#[doc = "Field `TS` reader - High Resolution Time Stamp"]
pub type TsR = crate::FieldReader<u32>;
#[doc = "Field `TS` writer - High Resolution Time Stamp"]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - High Resolution Time Stamp"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - High Resolution Time Stamp"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<HrTimeStampSpec> {
        TsW::new(self, 0)
    }
}
#[doc = "High Resolution Time Stamp\n\nYou can [`read`](crate::Reg::read) this register and get [`hr_time_stamp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hr_time_stamp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrTimeStampSpec;
impl crate::RegisterSpec for HrTimeStampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hr_time_stamp::R`](R) reader structure"]
impl crate::Readable for HrTimeStampSpec {}
#[doc = "`write(|w| ..)` method takes [`hr_time_stamp::W`](W) writer structure"]
impl crate::Writable for HrTimeStampSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HR_TIME_STAMP[%s] to value 0"]
impl crate::Resettable for HrTimeStampSpec {}
