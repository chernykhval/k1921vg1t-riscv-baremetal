#[doc = "Register `I2SFRMCTRL1` reader"]
pub type R = crate::R<I2sfrmctrl1Spec>;
#[doc = "Register `I2SFRMCTRL1` writer"]
pub type W = crate::W<I2sfrmctrl1Spec>;
#[doc = "Field `VAL` reader - Frame Control 1 Value"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Frame Control 1 Value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Frame Control 1 Value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Frame Control 1 Value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, I2sfrmctrl1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "I2S Frame control 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sfrmctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sfrmctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2sfrmctrl1Spec;
impl crate::RegisterSpec for I2sfrmctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2sfrmctrl1::R`](R) reader structure"]
impl crate::Readable for I2sfrmctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`i2sfrmctrl1::W`](W) writer structure"]
impl crate::Writable for I2sfrmctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2SFRMCTRL1 to value 0"]
impl crate::Resettable for I2sfrmctrl1Spec {}
