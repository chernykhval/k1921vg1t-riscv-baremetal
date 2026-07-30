#[doc = "Register `TZFRC` writer"]
pub type W = crate::W<TzfrcSpec>;
#[doc = "Field `CBC` writer - Force a Cycle-by-Cycle trip event via software"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` writer - Force a One-Shot trip event via software"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Force a Cycle-by-Cycle trip event via software"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<'_, TzfrcSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Force a One-Shot trip event via software"]
    #[inline(always)]
    pub fn ost(&mut self) -> OstW<'_, TzfrcSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Trip-Zone Force Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzfrc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzfrcSpec;
impl crate::RegisterSpec for TzfrcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tzfrc::W`](W) writer structure"]
impl crate::Writable for TzfrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZFRC to value 0"]
impl crate::Resettable for TzfrcSpec {}
