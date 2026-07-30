#[doc = "Register `RXEVCLR` writer"]
pub type W = crate::W<RxevclrSpec>;
#[doc = "Field `VAL` writer - RXEV disable"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - RXEV disable"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, RxevclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Core RXEV request disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxevclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxevclrSpec;
impl crate::RegisterSpec for RxevclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rxevclr::W`](W) writer structure"]
impl crate::Writable for RxevclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXEVCLR to value 0"]
impl crate::Resettable for RxevclrSpec {}
