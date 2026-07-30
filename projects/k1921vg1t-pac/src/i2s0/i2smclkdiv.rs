#[doc = "Register `I2SMCLKDIV` reader"]
pub type R = crate::R<I2smclkdivSpec>;
#[doc = "Register `I2SMCLKDIV` writer"]
pub type W = crate::W<I2smclkdivSpec>;
#[doc = "Field `VAL` reader - Master clock divider value"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Master clock divider value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Master clock divider value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Master clock divider value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, I2smclkdivSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "I2S Master Clock Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2smclkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2smclkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2smclkdivSpec;
impl crate::RegisterSpec for I2smclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2smclkdiv::R`](R) reader structure"]
impl crate::Readable for I2smclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`i2smclkdiv::W`](W) writer structure"]
impl crate::Writable for I2smclkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SMCLKDIV to value 0"]
impl crate::Resettable for I2smclkdivSpec {}
