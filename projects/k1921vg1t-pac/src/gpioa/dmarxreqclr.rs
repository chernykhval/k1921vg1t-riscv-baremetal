#[doc = "Register `DMARXREQCLR` writer"]
pub type W = crate::W<DmarxreqclrSpec>;
#[doc = "Field `VAL` writer - DMA request disable"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - DMA request disable"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, DmarxreqclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "DMA RX request generator disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmarxreqclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmarxreqclrSpec;
impl crate::RegisterSpec for DmarxreqclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmarxreqclr::W`](W) writer structure"]
impl crate::Writable for DmarxreqclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMARXREQCLR to value 0"]
impl crate::Resettable for DmarxreqclrSpec {}
