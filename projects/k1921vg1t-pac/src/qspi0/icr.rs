#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `TCIC` reader - Transfer Complete Interrupt Clear"]
pub type TcicR = crate::BitReader;
#[doc = "Field `TCIC` writer - Transfer Complete Interrupt Clear"]
pub type TcicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIC` reader - Transfer Stall Interrupt Clear"]
pub type TsicR = crate::BitReader;
#[doc = "Field `TSIC` writer - Transfer Stall Interrupt Clear"]
pub type TsicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIC` reader - Tx FIFO Empty Interrupt Clear"]
pub type TeicR = crate::BitReader;
#[doc = "Field `TEIC` writer - Tx FIFO Empty Interrupt Clear"]
pub type TeicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIC` reader - Tx FIFO Full Interrupt Clear"]
pub type TficR = crate::BitReader;
#[doc = "Field `TFIC` writer - Tx FIFO Full Interrupt Clear"]
pub type TficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REIC` reader - Rx FIFO Empty Interrupt Clear"]
pub type ReicR = crate::BitReader;
#[doc = "Field `REIC` writer - Rx FIFO Empty Interrupt Clear"]
pub type ReicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIC` reader - Rx FIFO Full Interrupt Clear"]
pub type RficR = crate::BitReader;
#[doc = "Field `RFIC` writer - Rx FIFO Full Interrupt Clear"]
pub type RficW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWMIC` reader - Tx FIFO Watermark Interrupt Clear"]
pub type TwmicR = crate::BitReader;
#[doc = "Field `TWMIC` writer - Tx FIFO Watermark Interrupt Clear"]
pub type TwmicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWMIC` reader - Rx FIFO Watermark Interrupt Clear"]
pub type RwmicR = crate::BitReader;
#[doc = "Field `RWMIC` writer - Rx FIFO Watermark Interrupt Clear"]
pub type RwmicW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Transfer Complete Interrupt Clear"]
    #[inline(always)]
    pub fn tcic(&self) -> TcicR {
        TcicR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Stall Interrupt Clear"]
    #[inline(always)]
    pub fn tsic(&self) -> TsicR {
        TsicR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO Empty Interrupt Clear"]
    #[inline(always)]
    pub fn teic(&self) -> TeicR {
        TeicR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx FIFO Full Interrupt Clear"]
    #[inline(always)]
    pub fn tfic(&self) -> TficR {
        TficR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO Empty Interrupt Clear"]
    #[inline(always)]
    pub fn reic(&self) -> ReicR {
        ReicR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO Full Interrupt Clear"]
    #[inline(always)]
    pub fn rfic(&self) -> RficR {
        RficR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx FIFO Watermark Interrupt Clear"]
    #[inline(always)]
    pub fn twmic(&self) -> TwmicR {
        TwmicR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx FIFO Watermark Interrupt Clear"]
    #[inline(always)]
    pub fn rwmic(&self) -> RwmicR {
        RwmicR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transfer Complete Interrupt Clear"]
    #[inline(always)]
    pub fn tcic(&mut self) -> TcicW<'_, IcrSpec> {
        TcicW::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Stall Interrupt Clear"]
    #[inline(always)]
    pub fn tsic(&mut self) -> TsicW<'_, IcrSpec> {
        TsicW::new(self, 2)
    }
    #[doc = "Bit 4 - Tx FIFO Empty Interrupt Clear"]
    #[inline(always)]
    pub fn teic(&mut self) -> TeicW<'_, IcrSpec> {
        TeicW::new(self, 4)
    }
    #[doc = "Bit 5 - Tx FIFO Full Interrupt Clear"]
    #[inline(always)]
    pub fn tfic(&mut self) -> TficW<'_, IcrSpec> {
        TficW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO Empty Interrupt Clear"]
    #[inline(always)]
    pub fn reic(&mut self) -> ReicW<'_, IcrSpec> {
        ReicW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO Full Interrupt Clear"]
    #[inline(always)]
    pub fn rfic(&mut self) -> RficW<'_, IcrSpec> {
        RficW::new(self, 7)
    }
    #[doc = "Bit 8 - Tx FIFO Watermark Interrupt Clear"]
    #[inline(always)]
    pub fn twmic(&mut self) -> TwmicW<'_, IcrSpec> {
        TwmicW::new(self, 8)
    }
    #[doc = "Bit 9 - Rx FIFO Watermark Interrupt Clear"]
    #[inline(always)]
    pub fn rwmic(&mut self) -> RwmicW<'_, IcrSpec> {
        RwmicW::new(self, 9)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
