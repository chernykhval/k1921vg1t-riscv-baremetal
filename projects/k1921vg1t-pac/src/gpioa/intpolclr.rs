#[doc = "Register `INTPOLCLR` writer"]
pub type W = crate::W<IntpolclrSpec>;
#[doc = "Field `VAL` writer - Interrupt polarity clear"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Interrupt polarity clear"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, IntpolclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Interrupt polarity clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intpolclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntpolclrSpec;
impl crate::RegisterSpec for IntpolclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intpolclr::W`](W) writer structure"]
impl crate::Writable for IntpolclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTPOLCLR to value 0"]
impl crate::Resettable for IntpolclrSpec {}
