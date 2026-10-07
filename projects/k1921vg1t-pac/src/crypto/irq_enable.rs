#[doc = "Register `IRQ_ENABLE` reader"]
pub type R = crate::R<IrqEnableSpec>;
#[doc = "Register `IRQ_ENABLE` writer"]
pub type W = crate::W<IrqEnableSpec>;
#[doc = "Field `DONE` reader - "]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - "]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_IGNORED` reader - "]
pub type WrIgnoredR = crate::BitReader;
#[doc = "Field `WR_IGNORED` writer - "]
pub type WrIgnoredW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_DONE` reader - "]
pub type DmaDoneR = crate::BitReader;
#[doc = "Field `DMA_FAIL` reader - "]
pub type DmaFailR = crate::BitReader;
#[doc = "Field `DMA_FAIL` writer - "]
pub type DmaFailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wr_ignored(&self) -> WrIgnoredR {
        WrIgnoredR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_done(&self) -> DmaDoneR {
        DmaDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_fail(&self) -> DmaFailR {
        DmaFailR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, IrqEnableSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wr_ignored(&mut self) -> WrIgnoredW<'_, IrqEnableSpec> {
        WrIgnoredW::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_fail(&mut self) -> DmaFailW<'_, IrqEnableSpec> {
        DmaFailW::new(self, 3)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnableSpec;
impl crate::RegisterSpec for IrqEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_enable::R`](R) reader structure"]
impl crate::Readable for IrqEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_enable::W`](W) writer structure"]
impl crate::Writable for IrqEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_ENABLE to value 0"]
impl crate::Resettable for IrqEnableSpec {}
