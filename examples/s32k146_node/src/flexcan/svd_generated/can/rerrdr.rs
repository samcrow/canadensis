#[doc = "Register `RERRDR` reader"]
pub type R = crate::R<RerrdrSpec>;
#[doc = "Field `RDATA` reader - Raw data word read from memory with error"]
pub type RdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Raw data word read from memory with error"]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new(self.bits)
    }
}
#[doc = "Error Report Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rerrdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RerrdrSpec;
impl crate::RegisterSpec for RerrdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rerrdr::R`](R) reader structure"]
impl crate::Readable for RerrdrSpec {}
#[doc = "`reset()` method sets RERRDR to value 0"]
impl crate::Resettable for RerrdrSpec {}
