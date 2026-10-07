#[doc = "Register `SYNCCLR` writer"]
pub type W = crate::W<SyncclrSpec>;
#[doc = "Field `VAL` writer - Additional double flip-flop syncronization disable"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Additional double flip-flop syncronization disable"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, SyncclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Additional double flip-flop syncronization disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncclrSpec;
impl crate::RegisterSpec for SyncclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`syncclr::W`](W) writer structure"]
impl crate::Writable for SyncclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCCLR to value 0"]
impl crate::Resettable for SyncclrSpec {}
