#[doc = "Register `POST` reader"]
pub type R = crate::R<PostSpec>;
#[doc = "Field `VAL` reader - Post-processing CRC value"]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Post-processing CRC value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "CRC post register\n\nYou can [`read`](crate::Reg::read) this register and get [`post::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PostSpec;
impl crate::RegisterSpec for PostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`post::R`](R) reader structure"]
impl crate::Readable for PostSpec {}
#[doc = "`reset()` method sets POST to value 0"]
impl crate::Resettable for PostSpec {}
