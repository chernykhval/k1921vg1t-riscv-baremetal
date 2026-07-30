#[doc = "Register `ERR_STOP` reader"]
pub type R = crate::R<ErrStopSpec>;
#[doc = "Register `ERR_STOP` writer"]
pub type W = crate::W<ErrStopSpec>;
#[doc = "Field `RD_SLVERR` reader - If set, stop read and write transactions when this condition occurs"]
pub type RdSlverrR = crate::BitReader;
#[doc = "Field `RD_SLVERR` writer - If set, stop read and write transactions when this condition occurs"]
pub type RdSlverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_SLVERR` reader - If set, stop read and write transactions when this condition occurs"]
pub type WrSlverrR = crate::BitReader;
#[doc = "Field `WR_SLVERR` writer - If set, stop read and write transactions when this condition occurs"]
pub type WrSlverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DECERR` reader - If set, stop read and write transactions when this condition occurs"]
pub type RdDecerrR = crate::BitReader;
#[doc = "Field `RD_DECERR` writer - If set, stop read and write transactions when this condition occurs"]
pub type RdDecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_DECERR` reader - If set, stop read and write transactions when this condition occurs"]
pub type WrDecerrR = crate::BitReader;
#[doc = "Field `WR_DECERR` writer - If set, stop read and write transactions when this condition occurs"]
pub type WrDecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - If set, stop read and write transactions when this condition occurs"]
pub type OverflowR = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - If set, stop read and write transactions when this condition occurs"]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - If set, stop read and write transactions when this condition occurs"]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - If set, stop read and write transactions when this condition occurs"]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_R` reader - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutRR = crate::BitReader;
#[doc = "Field `TIMEOUT_R` writer - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutRW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_AR` reader - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutArR = crate::BitReader;
#[doc = "Field `TIMEOUT_AR` writer - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutArW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_B` reader - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutBR = crate::BitReader;
#[doc = "Field `TIMEOUT_B` writer - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_W` reader - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutWR = crate::BitReader;
#[doc = "Field `TIMEOUT_W` writer - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutWW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_AW` reader - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutAwR = crate::BitReader;
#[doc = "Field `TIMEOUT_AW` writer - If set, stop read and write transactions when this condition occurs"]
pub type TimeoutAwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_TIMEOUT` reader - If set, stop read and write transactions when this condition occurs"]
pub type WdtTimeoutR = crate::BitReader;
#[doc = "Field `WDT_TIMEOUT` writer - If set, stop read and write transactions when this condition occurs"]
pub type WdtTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn rd_slverr(&self) -> RdSlverrR {
        RdSlverrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn wr_slverr(&self) -> WrSlverrR {
        WrSlverrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn rd_decerr(&self) -> RdDecerrR {
        RdDecerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn wr_decerr(&self) -> WrDecerrR {
        WrDecerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_r(&self) -> TimeoutRR {
        TimeoutRR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_ar(&self) -> TimeoutArR {
        TimeoutArR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_b(&self) -> TimeoutBR {
        TimeoutBR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_w(&self) -> TimeoutWR {
        TimeoutWR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_aw(&self) -> TimeoutAwR {
        TimeoutAwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn wdt_timeout(&self) -> WdtTimeoutR {
        WdtTimeoutR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn rd_slverr(&mut self) -> RdSlverrW<'_, ErrStopSpec> {
        RdSlverrW::new(self, 1)
    }
    #[doc = "Bit 2 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn wr_slverr(&mut self) -> WrSlverrW<'_, ErrStopSpec> {
        WrSlverrW::new(self, 2)
    }
    #[doc = "Bit 3 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn rd_decerr(&mut self) -> RdDecerrW<'_, ErrStopSpec> {
        RdDecerrW::new(self, 3)
    }
    #[doc = "Bit 4 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn wr_decerr(&mut self) -> WrDecerrW<'_, ErrStopSpec> {
        WrDecerrW::new(self, 4)
    }
    #[doc = "Bit 5 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<'_, ErrStopSpec> {
        OverflowW::new(self, 5)
    }
    #[doc = "Bit 6 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UnderflowW<'_, ErrStopSpec> {
        UnderflowW::new(self, 6)
    }
    #[doc = "Bit 7 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_r(&mut self) -> TimeoutRW<'_, ErrStopSpec> {
        TimeoutRW::new(self, 7)
    }
    #[doc = "Bit 8 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_ar(&mut self) -> TimeoutArW<'_, ErrStopSpec> {
        TimeoutArW::new(self, 8)
    }
    #[doc = "Bit 9 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_b(&mut self) -> TimeoutBW<'_, ErrStopSpec> {
        TimeoutBW::new(self, 9)
    }
    #[doc = "Bit 10 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_w(&mut self) -> TimeoutWW<'_, ErrStopSpec> {
        TimeoutWW::new(self, 10)
    }
    #[doc = "Bit 11 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn timeout_aw(&mut self) -> TimeoutAwW<'_, ErrStopSpec> {
        TimeoutAwW::new(self, 11)
    }
    #[doc = "Bit 12 - If set, stop read and write transactions when this condition occurs"]
    #[inline(always)]
    pub fn wdt_timeout(&mut self) -> WdtTimeoutW<'_, ErrStopSpec> {
        WdtTimeoutW::new(self, 12)
    }
}
#[doc = "Channel error stop condition register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrStopSpec;
impl crate::RegisterSpec for ErrStopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_stop::R`](R) reader structure"]
impl crate::Readable for ErrStopSpec {}
#[doc = "`write(|w| ..)` method takes [`err_stop::W`](W) writer structure"]
impl crate::Writable for ErrStopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERR_STOP to value 0"]
impl crate::Resettable for ErrStopSpec {}
