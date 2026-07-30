#[doc = "Register `HDFRC` writer"]
pub type W = crate::W<HdfrcSpec>;
#[doc = "Field `CBC` writer - Force a Cycle-by-Cycle hold detector event via software"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` writer - Force a One-Shot hold detector event via software"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - Force a Cycle-by-Cycle hold detector event via software"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<'_, HdfrcSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Force a One-Shot hold detector event via software"]
    #[inline(always)]
    pub fn ost(&mut self) -> OstW<'_, HdfrcSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Hold Detector Force Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdfrc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdfrcSpec;
impl crate::RegisterSpec for HdfrcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdfrc::W`](W) writer structure"]
impl crate::Writable for HdfrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HDFRC to value 0"]
impl crate::Resettable for HdfrcSpec {}
