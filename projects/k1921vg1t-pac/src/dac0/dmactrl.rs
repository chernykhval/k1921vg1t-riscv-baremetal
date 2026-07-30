#[doc = "Register `DMACTRL` reader"]
pub type R = crate::R<DmactrlSpec>;
#[doc = "Register `DMACTRL` writer"]
pub type W = crate::W<DmactrlSpec>;
#[doc = "Field `ENABLE` reader - DMA request generation enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - DMA request generation enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSTOP` reader - Automatic clear DMA TX request generation when channel ends all transactions"]
pub type TxstopR = crate::BitReader;
#[doc = "Field `TXSTOP` writer - Automatic clear DMA TX request generation when channel ends all transactions"]
pub type TxstopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA request generation enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Automatic clear DMA TX request generation when channel ends all transactions"]
    #[inline(always)]
    pub fn txstop(&self) -> TxstopR {
        TxstopR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA request generation enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, DmactrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Automatic clear DMA TX request generation when channel ends all transactions"]
    #[inline(always)]
    pub fn txstop(&mut self) -> TxstopW<'_, DmactrlSpec> {
        TxstopW::new(self, 1)
    }
}
#[doc = "DAC DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactrlSpec;
impl crate::RegisterSpec for DmactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactrl::R`](R) reader structure"]
impl crate::Readable for DmactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactrl::W`](W) writer structure"]
impl crate::Writable for DmactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACTRL to value 0"]
impl crate::Resettable for DmactrlSpec {}
