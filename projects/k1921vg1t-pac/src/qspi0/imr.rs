#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `TCIM` reader - Transfer Complete Interrupt Mask"]
pub type TcimR = crate::BitReader;
#[doc = "Field `TCIM` writer - Transfer Complete Interrupt Mask"]
pub type TcimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIM` reader - Transfer Stall Interrupt Mask"]
pub type TsimR = crate::BitReader;
#[doc = "Field `TSIM` writer - Transfer Stall Interrupt Mask"]
pub type TsimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIM` reader - Tx FIFO Empty Interrupt Mask"]
pub type TeimR = crate::BitReader;
#[doc = "Field `TEIM` writer - Tx FIFO Empty Interrupt Mask"]
pub type TeimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TFIM` reader - Tx FIFO Full Interrupt Mask"]
pub type TfimR = crate::BitReader;
#[doc = "Field `TFIM` writer - Tx FIFO Full Interrupt Mask"]
pub type TfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REIM` reader - Rx FIFO Empty Interrupt Mask"]
pub type ReimR = crate::BitReader;
#[doc = "Field `REIM` writer - Rx FIFO Empty Interrupt Mask"]
pub type ReimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIM` reader - Rx FIFO Full Interrupt Mask"]
pub type RfimR = crate::BitReader;
#[doc = "Field `RFIM` writer - Rx FIFO Full Interrupt Mask"]
pub type RfimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TWMIM` reader - Tx FIFO Watermark Interrupt Mask"]
pub type TwmimR = crate::BitReader;
#[doc = "Field `TWMIM` writer - Tx FIFO Watermark Interrupt Mask"]
pub type TwmimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWMIM` reader - Rx FIFO Watermark Interrupt Mask"]
pub type RwmimR = crate::BitReader;
#[doc = "Field `RWMIM` writer - Rx FIFO Watermark Interrupt Mask"]
pub type RwmimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Transfer Complete Interrupt Mask"]
    #[inline(always)]
    pub fn tcim(&self) -> TcimR {
        TcimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Stall Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&self) -> TsimR {
        TsimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn teim(&self) -> TeimR {
        TeimR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn tfim(&self) -> TfimR {
        TfimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn reim(&self) -> ReimR {
        ReimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rfim(&self) -> RfimR {
        RfimR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx FIFO Watermark Interrupt Mask"]
    #[inline(always)]
    pub fn twmim(&self) -> TwmimR {
        TwmimR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx FIFO Watermark Interrupt Mask"]
    #[inline(always)]
    pub fn rwmim(&self) -> RwmimR {
        RwmimR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Transfer Complete Interrupt Mask"]
    #[inline(always)]
    pub fn tcim(&mut self) -> TcimW<'_, ImrSpec> {
        TcimW::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Stall Interrupt Mask"]
    #[inline(always)]
    pub fn tsim(&mut self) -> TsimW<'_, ImrSpec> {
        TsimW::new(self, 2)
    }
    #[doc = "Bit 4 - Tx FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn teim(&mut self) -> TeimW<'_, ImrSpec> {
        TeimW::new(self, 4)
    }
    #[doc = "Bit 5 - Tx FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn tfim(&mut self) -> TfimW<'_, ImrSpec> {
        TfimW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn reim(&mut self) -> ReimW<'_, ImrSpec> {
        ReimW::new(self, 6)
    }
    #[doc = "Bit 7 - Rx FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn rfim(&mut self) -> RfimW<'_, ImrSpec> {
        RfimW::new(self, 7)
    }
    #[doc = "Bit 8 - Tx FIFO Watermark Interrupt Mask"]
    #[inline(always)]
    pub fn twmim(&mut self) -> TwmimW<'_, ImrSpec> {
        TwmimW::new(self, 8)
    }
    #[doc = "Bit 9 - Rx FIFO Watermark Interrupt Mask"]
    #[inline(always)]
    pub fn rwmim(&mut self) -> RwmimW<'_, ImrSpec> {
        RwmimW::new(self, 9)
    }
}
#[doc = "Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
