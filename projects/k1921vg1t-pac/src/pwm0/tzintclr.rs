#[doc = "Register `TZINTCLR` writer"]
pub type W = crate::W<TzintclrSpec>;
#[doc = "Field `INT` writer - Clear TZ interrupt pending"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear TZ interrupt pending"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, TzintclrSpec> {
        IntW::new(self, 0)
    }
}
#[doc = "Trip-Zone Interrupt pending Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzintclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TzintclrSpec;
impl crate::RegisterSpec for TzintclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tzintclr::W`](W) writer structure"]
impl crate::Writable for TzintclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TZINTCLR to value 0"]
impl crate::Resettable for TzintclrSpec {}
