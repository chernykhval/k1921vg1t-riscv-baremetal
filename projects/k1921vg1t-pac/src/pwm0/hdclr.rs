#[doc = "Register `HDCLR` writer"]
pub type W = crate::W<HdclrSpec>;
#[doc = "Field `INT` writer - Clear hold detector interrupt flag"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBC` writer - Clear flag for Cycle-By-Cycle hold detector latch"]
pub type CbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OST` writer - Clear flag for One-Shot hold detector latch"]
pub type OstW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear hold detector interrupt flag"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, HdclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear flag for Cycle-By-Cycle hold detector latch"]
    #[inline(always)]
    pub fn cbc(&mut self) -> CbcW<'_, HdclrSpec> {
        CbcW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear flag for One-Shot hold detector latch"]
    #[inline(always)]
    pub fn ost(&mut self) -> OstW<'_, HdclrSpec> {
        OstW::new(self, 2)
    }
}
#[doc = "Register clear HD flag\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HdclrSpec;
impl crate::RegisterSpec for HdclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`hdclr::W`](W) writer structure"]
impl crate::Writable for HdclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HDCLR to value 0"]
impl crate::Resettable for HdclrSpec {}
