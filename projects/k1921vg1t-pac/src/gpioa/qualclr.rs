#[doc = "Register `QUALCLR` writer"]
pub type W = crate::W<QualclrSpec>;
#[doc = "Field `VAL` writer - Qualifier disable"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Qualifier disable"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, QualclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Qualifier disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qualclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QualclrSpec;
impl crate::RegisterSpec for QualclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`qualclr::W`](W) writer structure"]
impl crate::Writable for QualclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QUALCLR to value 0"]
impl crate::Resettable for QualclrSpec {}
