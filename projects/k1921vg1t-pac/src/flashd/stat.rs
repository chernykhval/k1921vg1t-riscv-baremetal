#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Field `BUSY` reader - Busy status bit when command is processing"]
pub type BusyR = crate::BitReader;
#[doc = "Field `IRQF` reader - IRQ Flag set when command done. Set by hardware only if IRQEN bit is set."]
pub type IrqfR = crate::BitReader;
#[doc = "Field `IRQF` writer - IRQ Flag set when command done. Set by hardware only if IRQEN bit is set."]
pub type IrqfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPD` reader - Flash IP in powerdown mode"]
pub type DpdR = crate::BitReader;
#[doc = "Field `NVRLOCK` reader - If set NVR_CFG sector of flash is locked"]
pub type NvrlockR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Busy status bit when command is processing"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IRQ Flag set when command done. Set by hardware only if IRQEN bit is set."]
    #[inline(always)]
    pub fn irqf(&self) -> IrqfR {
        IrqfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash IP in powerdown mode"]
    #[inline(always)]
    pub fn dpd(&self) -> DpdR {
        DpdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - If set NVR_CFG sector of flash is locked"]
    #[inline(always)]
    pub fn nvrlock(&self) -> NvrlockR {
        NvrlockR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IRQ Flag set when command done. Set by hardware only if IRQEN bit is set."]
    #[inline(always)]
    pub fn irqf(&mut self) -> IrqfW<'_, StatSpec> {
        IrqfW::new(self, 1)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
