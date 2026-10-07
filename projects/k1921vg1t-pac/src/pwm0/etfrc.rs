#[doc = "Register `ETFRC` writer"]
pub type W = crate::W<EtfrcSpec>;
#[doc = "Field `INT` writer - PWM_INT force bit."]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCA` writer - PWM_SOCA force bit"]
pub type SocaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCB` writer - PWM_SOCB force bit"]
pub type SocbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQA` writer - PWM DMA request A force bit"]
pub type DrqaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQB` writer - PWM DMA request B force bit"]
pub type DrqbW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - PWM_INT force bit."]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, EtfrcSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 2 - PWM_SOCA force bit"]
    #[inline(always)]
    pub fn soca(&mut self) -> SocaW<'_, EtfrcSpec> {
        SocaW::new(self, 2)
    }
    #[doc = "Bit 3 - PWM_SOCB force bit"]
    #[inline(always)]
    pub fn socb(&mut self) -> SocbW<'_, EtfrcSpec> {
        SocbW::new(self, 3)
    }
    #[doc = "Bit 4 - PWM DMA request A force bit"]
    #[inline(always)]
    pub fn drqa(&mut self) -> DrqaW<'_, EtfrcSpec> {
        DrqaW::new(self, 4)
    }
    #[doc = "Bit 5 - PWM DMA request B force bit"]
    #[inline(always)]
    pub fn drqb(&mut self) -> DrqbW<'_, EtfrcSpec> {
        DrqbW::new(self, 5)
    }
}
#[doc = "Event-Trigger Force Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etfrc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtfrcSpec;
impl crate::RegisterSpec for EtfrcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`etfrc::W`](W) writer structure"]
impl crate::Writable for EtfrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETFRC to value 0"]
impl crate::Resettable for EtfrcSpec {}
