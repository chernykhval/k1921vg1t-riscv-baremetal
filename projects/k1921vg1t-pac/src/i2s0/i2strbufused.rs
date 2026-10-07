#[doc = "Register `I2STRBUFUSED` reader"]
pub type R = crate::R<I2strbufusedSpec>;
#[doc = "Field `VAL` reader - Buffer counter value"]
pub type ValR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Buffer counter value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "I2S Transmitter Buffer Counts register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2strbufused::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2strbufusedSpec;
impl crate::RegisterSpec for I2strbufusedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2strbufused::R`](R) reader structure"]
impl crate::Readable for I2strbufusedSpec {}
#[doc = "`reset()` method sets I2STRBUFUSED to value 0"]
impl crate::Resettable for I2strbufusedSpec {}
