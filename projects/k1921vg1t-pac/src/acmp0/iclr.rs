#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Field `CMP` writer - Clear interrupt status"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear interrupt status"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<'_, IclrSpec> {
        CmpW::new(self, 0)
    }
}
#[doc = "Clear interrupt status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {}
