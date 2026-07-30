#[doc = "Register `SCCTL` reader"]
pub type R = crate::R<ScctlSpec>;
#[doc = "Register `SCCTL` writer"]
pub type W = crate::W<ScctlSpec>;
#[doc = "Field `RCNT` reader - Current number of ADC restarts by sequencer"]
pub type RcntR = crate::FieldReader;
#[doc = "Field `RCNT` writer - Current number of ADC restarts by sequencer"]
pub type RcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RAVGEN` reader - Average of ADC restarts enable"]
pub type RavgenR = crate::BitReader;
#[doc = "Field `RAVGEN` writer - Average of ADC restarts enable"]
pub type RavgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICNT` reader - Number of ADC restarts for interrupt generation"]
pub type IcntR = crate::FieldReader;
#[doc = "Field `ICNT` writer - Number of ADC restarts for interrupt generation"]
pub type IcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Current number of ADC restarts by sequencer"]
    #[inline(always)]
    pub fn rcnt(&self) -> RcntR {
        RcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Average of ADC restarts enable"]
    #[inline(always)]
    pub fn ravgen(&self) -> RavgenR {
        RavgenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of ADC restarts for interrupt generation"]
    #[inline(always)]
    pub fn icnt(&self) -> IcntR {
        IcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Current number of ADC restarts by sequencer"]
    #[inline(always)]
    pub fn rcnt(&mut self) -> RcntW<'_, ScctlSpec> {
        RcntW::new(self, 0)
    }
    #[doc = "Bit 8 - Average of ADC restarts enable"]
    #[inline(always)]
    pub fn ravgen(&mut self) -> RavgenW<'_, ScctlSpec> {
        RavgenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Number of ADC restarts for interrupt generation"]
    #[inline(always)]
    pub fn icnt(&mut self) -> IcntW<'_, ScctlSpec> {
        IcntW::new(self, 16)
    }
}
#[doc = "Sequencer ADC interrupt and restart counter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`scctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScctlSpec;
impl crate::RegisterSpec for ScctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scctl::R`](R) reader structure"]
impl crate::Readable for ScctlSpec {}
#[doc = "`write(|w| ..)` method takes [`scctl::W`](W) writer structure"]
impl crate::Writable for ScctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCCTL to value 0"]
impl crate::Resettable for ScctlSpec {}
