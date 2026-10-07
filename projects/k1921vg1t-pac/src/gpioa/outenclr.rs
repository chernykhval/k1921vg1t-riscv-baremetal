#[doc = "Register `OUTENCLR` writer"]
pub type W = crate::W<OutenclrSpec>;
#[doc = "Field `VAL` writer - Output disable"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Output disable"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, OutenclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Output disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outenclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutenclrSpec;
impl crate::RegisterSpec for OutenclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`outenclr::W`](W) writer structure"]
impl crate::Writable for OutenclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTENCLR to value 0"]
impl crate::Resettable for OutenclrSpec {}
