#[doc = "Register `SRTMR` reader"]
pub type R = crate::R<SrtmrSpec>;
#[doc = "Register `SRTMR` writer"]
pub type W = crate::W<SrtmrSpec>;
#[doc = "Field `VAL` reader - Sequencer ADC restart timer value"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Sequencer ADC restart timer value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NOWAIT` reader - Timer update with no waiting the end of current seq task"]
pub type NowaitR = crate::BitReader;
#[doc = "Field `NOWAIT` writer - Timer update with no waiting the end of current seq task"]
pub type NowaitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Sequencer ADC restart timer value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 31 - Timer update with no waiting the end of current seq task"]
    #[inline(always)]
    pub fn nowait(&self) -> NowaitR {
        NowaitR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Sequencer ADC restart timer value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, SrtmrSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bit 31 - Timer update with no waiting the end of current seq task"]
    #[inline(always)]
    pub fn nowait(&mut self) -> NowaitW<'_, SrtmrSpec> {
        NowaitW::new(self, 31)
    }
}
#[doc = "Sequencer ADC restart timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`srtmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srtmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrtmrSpec;
impl crate::RegisterSpec for SrtmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srtmr::R`](R) reader structure"]
impl crate::Readable for SrtmrSpec {}
#[doc = "`write(|w| ..)` method takes [`srtmr::W`](W) writer structure"]
impl crate::Writable for SrtmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRTMR to value 0"]
impl crate::Resettable for SrtmrSpec {}
