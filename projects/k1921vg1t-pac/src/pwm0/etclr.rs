#[doc = "Register `ETCLR` writer"]
pub type W = crate::W<EtclrSpec>;
#[doc = "Field `INT` writer - Latched PWM Interrupt (PWM_INT) flag clear bit"]
pub type IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCA` writer - Latched PWM ADC Start-of-Conversion A (PWM_SOCA) flag clear bit"]
pub type SocaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCB` writer - Latched PWM ADC Start-of-Conversion B (PWM_SOCB) flag clear bit"]
pub type SocbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQA` writer - Latched PWM DMA request A flag clear bit"]
pub type DrqaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQB` writer - Latched PWM DMA request B flag clear bit"]
pub type DrqbW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Latched PWM Interrupt (PWM_INT) flag clear bit"]
    #[inline(always)]
    pub fn int(&mut self) -> IntW<'_, EtclrSpec> {
        IntW::new(self, 0)
    }
    #[doc = "Bit 2 - Latched PWM ADC Start-of-Conversion A (PWM_SOCA) flag clear bit"]
    #[inline(always)]
    pub fn soca(&mut self) -> SocaW<'_, EtclrSpec> {
        SocaW::new(self, 2)
    }
    #[doc = "Bit 3 - Latched PWM ADC Start-of-Conversion B (PWM_SOCB) flag clear bit"]
    #[inline(always)]
    pub fn socb(&mut self) -> SocbW<'_, EtclrSpec> {
        SocbW::new(self, 3)
    }
    #[doc = "Bit 4 - Latched PWM DMA request A flag clear bit"]
    #[inline(always)]
    pub fn drqa(&mut self) -> DrqaW<'_, EtclrSpec> {
        DrqaW::new(self, 4)
    }
    #[doc = "Bit 5 - Latched PWM DMA request B flag clear bit"]
    #[inline(always)]
    pub fn drqb(&mut self) -> DrqbW<'_, EtclrSpec> {
        DrqbW::new(self, 5)
    }
}
#[doc = "Event-Trigger Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtclrSpec;
impl crate::RegisterSpec for EtclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`etclr::W`](W) writer structure"]
impl crate::Writable for EtclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETCLR to value 0"]
impl crate::Resettable for EtclrSpec {}
