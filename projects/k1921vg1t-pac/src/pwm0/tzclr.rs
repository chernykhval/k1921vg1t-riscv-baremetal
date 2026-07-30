#[doc = "Register `TZCLR` writer"]
pub type W = crate::W<TzclrSpec>;
#[doc = "Field `INT` writer - Clear trip-zone interrupt flag"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC` writer - Clear flag for Cycle-By-Cycle trip latch"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` writer - Clear flag for One-Shot trip latch"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear trip-zone interrupt flag"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, TzclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear flag for Cycle-By-Cycle trip latch"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<'_, TzclrSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear flag for One-Shot trip latch"]
    #[inline(always)]
    pub fn ost(&mut self) -> OstW<'_, TzclrSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Trip-Zone Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzclrSpec;
impl crate::RegisterSpec for TzclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tzclr::W`](W) writer structure"]
impl crate::Writable for TzclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZCLR to value 0"]
impl crate::Resettable for TzclrSpec {}
