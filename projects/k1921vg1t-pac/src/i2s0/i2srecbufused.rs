#[doc = "Register `I2SRECBUFUSED` reader"]
pub type R = crate::R<I2srecbufusedSpec>;
#[doc = "Field `VAL` reader - Buffer counter value"]
pub type ValR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Buffer counter value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "I2S Receiver Buffer Counts register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2srecbufused::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2srecbufusedSpec;
impl crate::RegisterSpec for I2srecbufusedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2srecbufused::R`](R) reader structure"]
impl crate::Readable for I2srecbufusedSpec {}
#[doc = "`reset()` method sets I2SRECBUFUSED to value 0"]
impl crate::Resettable for I2srecbufusedSpec {}
