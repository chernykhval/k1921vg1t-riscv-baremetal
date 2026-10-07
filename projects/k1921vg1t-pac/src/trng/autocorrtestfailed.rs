#[doc = "Register `AUTOCORRTESTFAILED` reader"]
pub type R = crate::R<AutocorrtestfailedSpec>;
#[doc = "Field `VAL` reader - Auto-correlation test failing ring(s)"]
pub type ValR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Auto-correlation test failing ring(s)"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AutoCorrTestFailed register\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorrtestfailed::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AutocorrtestfailedSpec;
impl crate::RegisterSpec for AutocorrtestfailedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`autocorrtestfailed::R`](R) reader structure"]
impl crate::Readable for AutocorrtestfailedSpec {}
#[doc = "`reset()` method sets AUTOCORRTESTFAILED to value 0"]
impl crate::Resettable for AutocorrtestfailedSpec {}
