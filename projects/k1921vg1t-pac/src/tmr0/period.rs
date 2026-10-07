#[doc = "Register `PERIOD` reader"]
pub type R = crate::R<PeriodSpec>;
#[doc = "Register `PERIOD` writer"]
pub type W = crate::W<PeriodSpec>;
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
    pub fn val(&mut self) -> ValW<'_, PeriodSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Timer period register\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriodSpec;
impl crate::RegisterSpec for PeriodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`period::R`](R) reader structure"]
impl crate::Readable for PeriodSpec {}
#[doc = "`write(|w| ..)` method takes [`period::W`](W) writer structure"]
impl crate::Writable for PeriodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIOD to value 0"]
impl crate::Resettable for PeriodSpec {}
