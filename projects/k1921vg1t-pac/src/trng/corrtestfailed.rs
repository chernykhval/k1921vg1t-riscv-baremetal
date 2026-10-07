#[doc = "Register `CORRTESTFAILED` reader"]
pub type R = crate::R<CorrtestfailedSpec>;
#[doc = "Field `VAL` reader - Correlation test failing rings"]
pub type ValR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Correlation test failing rings"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "CorrTestFailed register\n\nYou can [`read`](crate::Reg::read) this register and get [`corrtestfailed::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CorrtestfailedSpec;
impl crate::RegisterSpec for CorrtestfailedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`corrtestfailed::R`](R) reader structure"]
impl crate::Readable for CorrtestfailedSpec {}
#[doc = "`reset()` method sets CORRTESTFAILED to value 0"]
impl crate::Resettable for CorrtestfailedSpec {}
