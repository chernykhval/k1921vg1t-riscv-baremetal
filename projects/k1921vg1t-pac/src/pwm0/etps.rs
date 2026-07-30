#[doc = "Register `ETPS` reader"]
pub type R = crate::R<EtpsSpec>;
#[doc = "Register `ETPS` writer"]
pub type W = crate::W<EtpsSpec>;
#[doc = "Field `INTPRD` reader - PWM interrupt (PWM_INT) period select"]
pub type IntprdR = crate::FieldReader;
#[doc = "Field `INTPRD` writer - PWM interrupt (PWM_INT) period select"]
pub type IntprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INTCNT` reader - PWM interrupt event (PWM_INT) counter register"]
pub type IntcntR = crate::FieldReader;
#[doc = "Field `SOCAPRD` reader - PWM ADC Start-of-Conversion A event (PWM_SOCA) period select"]
pub type SocaprdR = crate::FieldReader;
#[doc = "Field `SOCAPRD` writer - PWM ADC Start-of-Conversion A event (PWM_SOCA) period select"]
pub type SocaprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOCACNT` reader - PWM ADC Start-of-Conversion A event (PWM_SOCA) counter register"]
pub type SocacntR = crate::FieldReader;
#[doc = "Field `SOCBPRD` reader - PWM ADC Start-of-Conversion B event (PWM_SOCB) period select"]
pub type SocbprdR = crate::FieldReader;
#[doc = "Field `SOCBPRD` writer - PWM ADC Start-of-Conversion B event (PWM_SOCB) period select"]
pub type SocbprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SOCBCNT` reader - PWM ADC Start-of-Conversion B event (PWM_SOCB) counter register"]
pub type SocbcntR = crate::FieldReader;
#[doc = "Field `DRQAPRD` reader - PWM DMA request A period select"]
pub type DrqaprdR = crate::FieldReader;
#[doc = "Field `DRQAPRD` writer - PWM DMA request A period select"]
pub type DrqaprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRQACNT` reader - PWM DMA request event A counter"]
pub type DrqacntR = crate::FieldReader;
#[doc = "Field `DRQBPRD` reader - PWM DMA request B period select"]
pub type DrqbprdR = crate::FieldReader;
#[doc = "Field `DRQBPRD` writer - PWM DMA request B period select"]
pub type DrqbprdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DRQBCNT` reader - PWM DMA request event B counter"]
pub type DrqbcntR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - PWM interrupt (PWM_INT) period select"]
    #[inline(always)]
    pub fn intprd(&self) -> IntprdR {
        IntprdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PWM interrupt event (PWM_INT) counter register"]
    #[inline(always)]
    pub fn intcnt(&self) -> IntcntR {
        IntcntR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PWM ADC Start-of-Conversion A event (PWM_SOCA) period select"]
    #[inline(always)]
    pub fn socaprd(&self) -> SocaprdR {
        SocaprdR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PWM ADC Start-of-Conversion A event (PWM_SOCA) counter register"]
    #[inline(always)]
    pub fn socacnt(&self) -> SocacntR {
        SocacntR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PWM ADC Start-of-Conversion B event (PWM_SOCB) period select"]
    #[inline(always)]
    pub fn socbprd(&self) -> SocbprdR {
        SocbprdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PWM ADC Start-of-Conversion B event (PWM_SOCB) counter register"]
    #[inline(always)]
    pub fn socbcnt(&self) -> SocbcntR {
        SocbcntR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - PWM DMA request A period select"]
    #[inline(always)]
    pub fn drqaprd(&self) -> DrqaprdR {
        DrqaprdR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PWM DMA request event A counter"]
    #[inline(always)]
    pub fn drqacnt(&self) -> DrqacntR {
        DrqacntR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PWM DMA request B period select"]
    #[inline(always)]
    pub fn drqbprd(&self) -> DrqbprdR {
        DrqbprdR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PWM DMA request event B counter"]
    #[inline(always)]
    pub fn drqbcnt(&self) -> DrqbcntR {
        DrqbcntR::new(((self.bits >> 22) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PWM interrupt (PWM_INT) period select"]
    #[inline(always)]
    pub fn intprd(&mut self) -> IntprdW<'_, EtpsSpec> {
        IntprdW::new(self, 0)
    }
    #[doc = "Bits 8:9 - PWM ADC Start-of-Conversion A event (PWM_SOCA) period select"]
    #[inline(always)]
    pub fn socaprd(&mut self) -> SocaprdW<'_, EtpsSpec> {
        SocaprdW::new(self, 8)
    }
    #[doc = "Bits 12:13 - PWM ADC Start-of-Conversion B event (PWM_SOCB) period select"]
    #[inline(always)]
    pub fn socbprd(&mut self) -> SocbprdW<'_, EtpsSpec> {
        SocbprdW::new(self, 12)
    }
    #[doc = "Bits 16:17 - PWM DMA request A period select"]
    #[inline(always)]
    pub fn drqaprd(&mut self) -> DrqaprdW<'_, EtpsSpec> {
        DrqaprdW::new(self, 16)
    }
    #[doc = "Bits 20:21 - PWM DMA request B period select"]
    #[inline(always)]
    pub fn drqbprd(&mut self) -> DrqbprdW<'_, EtpsSpec> {
        DrqbprdW::new(self, 20)
    }
}
#[doc = "Event-Trigger Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etps::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etps::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtpsSpec;
impl crate::RegisterSpec for EtpsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etps::R`](R) reader structure"]
impl crate::Readable for EtpsSpec {}
#[doc = "`write(|w| ..)` method takes [`etps::W`](W) writer structure"]
impl crate::Writable for EtpsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETPS to value 0"]
impl crate::Resettable for EtpsSpec {}
