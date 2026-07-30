#[doc = "Register `I2SCLKDIV2` reader"]
pub type R = crate::R<I2sclkdiv2Spec>;
#[doc = "Register `I2SCLKDIV2` writer"]
pub type W = crate::W<I2sclkdiv2Spec>;
#[doc = "Field `VAL` reader - Divider 2 value"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Divider 2 value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Divider 2 value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Divider 2 value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, I2sclkdiv2Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "I2S Clock Divider 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sclkdiv2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sclkdiv2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sclkdiv2Spec;
impl crate::RegisterSpec for I2sclkdiv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sclkdiv2::R`](R) reader structure"]
impl crate::Readable for I2sclkdiv2Spec {}
#[doc = "`write(|w| ..)` method takes [`i2sclkdiv2::W`](W) writer structure"]
impl crate::Writable for I2sclkdiv2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SCLKDIV2 to value 0"]
impl crate::Resettable for I2sclkdiv2Spec {}
