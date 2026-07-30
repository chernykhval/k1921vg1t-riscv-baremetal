#[doc = "Register `IRQ` reader"]
pub type R = crate::R<IrqSpec>;
#[doc = "Register `IRQ` writer"]
pub type W = crate::W<IrqSpec>;
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
#[doc = "Field `DMA_DONE` writer - "]
pub type DmaDoneW<'a, REG> = crate::BitWriter<'a, REG>;
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
    pub fn done(&mut self) -> DoneW<'_, IrqSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wr_ignored(&mut self) -> WrIgnoredW<'_, IrqSpec> {
        WrIgnoredW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dma_done(&mut self) -> DmaDoneW<'_, IrqSpec> {
        DmaDoneW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dma_fail(&mut self) -> DmaFailW<'_, IrqSpec> {
        DmaFailW::new(self, 3)
    }
}
#[doc = "Interrupts pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqSpec;
impl crate::RegisterSpec for IrqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq::R`](R) reader structure"]
impl crate::Readable for IrqSpec {}
#[doc = "`write(|w| ..)` method takes [`irq::W`](W) writer structure"]
impl crate::Writable for IrqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IrqSpec {}
