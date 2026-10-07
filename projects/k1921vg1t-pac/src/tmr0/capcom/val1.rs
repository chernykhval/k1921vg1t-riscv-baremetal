#[doc = "Register `VAL1` reader"]
pub type R = crate::R<Val1Spec>;
#[doc = "Register `VAL1` writer"]
pub type W = crate::W<Val1Spec>;
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
    pub fn val(&mut self) -> ValW<'_, Val1Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Capture/Compare Value 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`val1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`val1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Val1Spec;
impl crate::RegisterSpec for Val1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`val1::R`](R) reader structure"]
impl crate::Readable for Val1Spec {}
#[doc = "`write(|w| ..)` method takes [`val1::W`](W) writer structure"]
impl crate::Writable for Val1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VAL1 to value 0"]
impl crate::Resettable for Val1Spec {}
