#[doc = "Register `INTEDGECLR` writer"]
pub type W = crate::W<IntedgeclrSpec>;
#[doc = "Field `VAL` writer - Interrupt every edge clear"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Interrupt every edge clear"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, IntedgeclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Interrupt every edge clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intedgeclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntedgeclrSpec;
impl crate::RegisterSpec for IntedgeclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intedgeclr::W`](W) writer structure"]
impl crate::Writable for IntedgeclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEDGECLR to value 0"]
impl crate::Resettable for IntedgeclrSpec {}
