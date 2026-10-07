#[doc = "Register `QUALMODECLR` writer"]
pub type W = crate::W<QualmodeclrSpec>;
#[doc = "Field `VAL` writer - Qualifier mode clear"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Qualifier mode clear"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, QualmodeclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Qualifier mode clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qualmodeclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QualmodeclrSpec;
impl crate::RegisterSpec for QualmodeclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qualmodeclr::W`](W) writer structure"]
impl crate::Writable for QualmodeclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QUALMODECLR to value 0"]
impl crate::Resettable for QualmodeclrSpec {}
