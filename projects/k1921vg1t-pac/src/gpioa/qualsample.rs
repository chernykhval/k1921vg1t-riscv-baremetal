#[doc = "Register `QUALSAMPLE` reader"]
pub type R = crate::R<QualsampleSpec>;
#[doc = "Register `QUALSAMPLE` writer"]
pub type W = crate::W<QualsampleSpec>;
#[doc = "Field `VAL` reader - Qualifier sample period"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Qualifier sample period"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Qualifier sample period"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - Qualifier sample period"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, QualsampleSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Qualifier sample period register\n\nYou can [`read`](crate::Reg::read) this register and get [`qualsample::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qualsample::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QualsampleSpec;
impl crate::RegisterSpec for QualsampleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qualsample::R`](R) reader structure"]
impl crate::Readable for QualsampleSpec {}
#[doc = "`write(|w| ..)` method takes [`qualsample::W`](W) writer structure"]
impl crate::Writable for QualsampleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QUALSAMPLE to value 0"]
impl crate::Resettable for QualsampleSpec {}
