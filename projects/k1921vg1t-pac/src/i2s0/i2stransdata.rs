#[doc = "Register `I2STRANSDATA` writer"]
pub type W = crate::W<I2stransdataSpec>;
#[doc = "Field `VAL` writer - Data to transmit value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Data to transmit value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, I2stransdataSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "I2S Transmitter Data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2stransdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2stransdataSpec;
impl crate::RegisterSpec for I2stransdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2stransdata::W`](W) writer structure"]
impl crate::Writable for I2stransdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2STRANSDATA to value 0"]
impl crate::Resettable for I2stransdataSpec {}
