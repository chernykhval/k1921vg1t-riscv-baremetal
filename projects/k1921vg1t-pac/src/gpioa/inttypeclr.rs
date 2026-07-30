#[doc = "Register `INTTYPECLR` writer"]
pub type W = crate::W<InttypeclrSpec>;
#[doc = "Field `VAL` writer - Interrupt type clear"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Interrupt type clear"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, InttypeclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Interrupt type clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttypeclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InttypeclrSpec;
impl crate::RegisterSpec for InttypeclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inttypeclr::W`](W) writer structure"]
impl crate::Writable for InttypeclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTTYPECLR to value 0"]
impl crate::Resettable for InttypeclrSpec {}
