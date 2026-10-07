#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Field `VAL` reader - Current value counter"]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current value counter"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "Watchdog Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`reset()` method sets VALUE to value 0xffff_ffff"]
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
