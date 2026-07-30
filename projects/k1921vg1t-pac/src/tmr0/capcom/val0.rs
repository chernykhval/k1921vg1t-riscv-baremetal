#[doc = "Register `VAL0` reader"]
pub type R = crate::R<Val0Spec>;
#[doc = "Register `VAL0` writer"]
pub type W = crate::W<Val0Spec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, Val0Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Capture/Compare Value 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`val0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`val0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Val0Spec;
impl crate::RegisterSpec for Val0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`val0::R`](R) reader structure"]
impl crate::Readable for Val0Spec {}
#[doc = "`write(|w| ..)` method takes [`val0::W`](W) writer structure"]
impl crate::Writable for Val0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VAL0 to value 0"]
impl crate::Resettable for Val0Spec {}
