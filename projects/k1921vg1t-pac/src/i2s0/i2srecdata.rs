#[doc = "Register `I2SRECDATA` reader"]
pub type R = crate::R<I2srecdataSpec>;
#[doc = "Field `VAL` reader - Received data value"]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Received data value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "I2S Receiver Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2srecdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2srecdataSpec;
impl crate::RegisterSpec for I2srecdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2srecdata::R`](R) reader structure"]
impl crate::Readable for I2srecdataSpec {}
#[doc = "`reset()` method sets I2SRECDATA to value 0"]
impl crate::Resettable for I2srecdataSpec {}
