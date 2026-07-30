#[doc = "Register `INT_ENABLE` reader"]
pub type R = crate::R<IntEnableSpec>;
#[doc = "Register `INT_ENABLE` writer"]
pub type W = crate::W<IntEnableSpec>;
#[doc = "Field `CH_END` reader - Channel end interrupt"]
pub type ChEndR = crate::BitReader;
#[doc = "Field `CH_END` writer - Channel end interrupt"]
pub type ChEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_SLVERR` reader - A read issued by this channel caused an AXI read slave decode error"]
pub type RdSlverrR = crate::BitReader;
#[doc = "Field `RD_SLVERR` writer - A read issued by this channel caused an AXI read slave decode error"]
pub type RdSlverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_SLVERR` reader - A write issued by this channel caused an AXI write slave decode error"]
pub type WrSlverrR = crate::BitReader;
#[doc = "Field `WR_SLVERR` writer - A write issued by this channel caused an AXI write slave decode error"]
pub type WrSlverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DECERR` reader - A read issued by this channel caused an AXI read decode error"]
pub type RdDecerrR = crate::BitReader;
#[doc = "Field `RD_DECERR` writer - A read issued by this channel caused an AXI read decode error"]
pub type RdDecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_DECERR` reader - A write issued by this channel caused an AXI write decode error"]
pub type WrDecerrR = crate::BitReader;
#[doc = "Field `WR_DECERR` writer - A write issued by this channel caused an AXI write decode error"]
pub type WrDecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` reader - Data FIFO has been overflowed"]
pub type OverflowR = crate::BitReader;
#[doc = "Field `OVERFLOW` writer - Data FIFO has been overflowed"]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` reader - Data FIFO has been underflowed"]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` writer - Data FIFO has been underflowed"]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_R` reader - A read issued by this channel caused a timeout on AXI read data bus"]
pub type TimeoutRR = crate::BitReader;
#[doc = "Field `TIMEOUT_R` writer - A read issued by this channel caused a timeout on AXI read data bus"]
pub type TimeoutRW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_AR` reader - A read issued by this channel caused a timeout on AXI read command bus"]
pub type TimeoutArR = crate::BitReader;
#[doc = "Field `TIMEOUT_AR` writer - A read issued by this channel caused a timeout on AXI read command bus"]
pub type TimeoutArW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_B` reader - A write issued by this channel caused a timeout on AXI write response bus"]
pub type TimeoutBR = crate::BitReader;
#[doc = "Field `TIMEOUT_B` writer - A write issued by this channel caused a timeout on AXI write response bus"]
pub type TimeoutBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_W` reader - A write issued by this channel caused a timeout on AXI write data bus"]
pub type TimeoutWR = crate::BitReader;
#[doc = "Field `TIMEOUT_W` writer - A write issued by this channel caused a timeout on AXI write data bus"]
pub type TimeoutWW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_AW` reader - A write issued by this channel caused a timeout on AXI write command bus"]
pub type TimeoutAwR = crate::BitReader;
#[doc = "Field `TIMEOUT_AW` writer - A write issued by this channel caused a timeout on AXI write command bus"]
pub type TimeoutAwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_TIMEOUT` reader - Channel is active but did not start a burst for 2048 cycles"]
pub type WdtTimeoutR = crate::BitReader;
#[doc = "Field `WDT_TIMEOUT` writer - Channel is active but did not start a burst for 2048 cycles"]
pub type WdtTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel end interrupt"]
    #[inline(always)]
    pub fn ch_end(&self) -> ChEndR {
        ChEndR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A read issued by this channel caused an AXI read slave decode error"]
    #[inline(always)]
    pub fn rd_slverr(&self) -> RdSlverrR {
        RdSlverrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A write issued by this channel caused an AXI write slave decode error"]
    #[inline(always)]
    pub fn wr_slverr(&self) -> WrSlverrR {
        WrSlverrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A read issued by this channel caused an AXI read decode error"]
    #[inline(always)]
    pub fn rd_decerr(&self) -> RdDecerrR {
        RdDecerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A write issued by this channel caused an AXI write decode error"]
    #[inline(always)]
    pub fn wr_decerr(&self) -> WrDecerrR {
        WrDecerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data FIFO has been overflowed"]
    #[inline(always)]
    pub fn overflow(&self) -> OverflowR {
        OverflowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Data FIFO has been underflowed"]
    #[inline(always)]
    pub fn underflow(&self) -> UnderflowR {
        UnderflowR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A read issued by this channel caused a timeout on AXI read data bus"]
    #[inline(always)]
    pub fn timeout_r(&self) -> TimeoutRR {
        TimeoutRR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A read issued by this channel caused a timeout on AXI read command bus"]
    #[inline(always)]
    pub fn timeout_ar(&self) -> TimeoutArR {
        TimeoutArR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A write issued by this channel caused a timeout on AXI write response bus"]
    #[inline(always)]
    pub fn timeout_b(&self) -> TimeoutBR {
        TimeoutBR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A write issued by this channel caused a timeout on AXI write data bus"]
    #[inline(always)]
    pub fn timeout_w(&self) -> TimeoutWR {
        TimeoutWR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - A write issued by this channel caused a timeout on AXI write command bus"]
    #[inline(always)]
    pub fn timeout_aw(&self) -> TimeoutAwR {
        TimeoutAwR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel is active but did not start a burst for 2048 cycles"]
    #[inline(always)]
    pub fn wdt_timeout(&self) -> WdtTimeoutR {
        WdtTimeoutR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel end interrupt"]
    #[inline(always)]
    pub fn ch_end(&mut self) -> ChEndW<'_, IntEnableSpec> {
        ChEndW::new(self, 0)
    }
    #[doc = "Bit 1 - A read issued by this channel caused an AXI read slave decode error"]
    #[inline(always)]
    pub fn rd_slverr(&mut self) -> RdSlverrW<'_, IntEnableSpec> {
        RdSlverrW::new(self, 1)
    }
    #[doc = "Bit 2 - A write issued by this channel caused an AXI write slave decode error"]
    #[inline(always)]
    pub fn wr_slverr(&mut self) -> WrSlverrW<'_, IntEnableSpec> {
        WrSlverrW::new(self, 2)
    }
    #[doc = "Bit 3 - A read issued by this channel caused an AXI read decode error"]
    #[inline(always)]
    pub fn rd_decerr(&mut self) -> RdDecerrW<'_, IntEnableSpec> {
        RdDecerrW::new(self, 3)
    }
    #[doc = "Bit 4 - A write issued by this channel caused an AXI write decode error"]
    #[inline(always)]
    pub fn wr_decerr(&mut self) -> WrDecerrW<'_, IntEnableSpec> {
        WrDecerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Data FIFO has been overflowed"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<'_, IntEnableSpec> {
        OverflowW::new(self, 5)
    }
    #[doc = "Bit 6 - Data FIFO has been underflowed"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UnderflowW<'_, IntEnableSpec> {
        UnderflowW::new(self, 6)
    }
    #[doc = "Bit 7 - A read issued by this channel caused a timeout on AXI read data bus"]
    #[inline(always)]
    pub fn timeout_r(&mut self) -> TimeoutRW<'_, IntEnableSpec> {
        TimeoutRW::new(self, 7)
    }
    #[doc = "Bit 8 - A read issued by this channel caused a timeout on AXI read command bus"]
    #[inline(always)]
    pub fn timeout_ar(&mut self) -> TimeoutArW<'_, IntEnableSpec> {
        TimeoutArW::new(self, 8)
    }
    #[doc = "Bit 9 - A write issued by this channel caused a timeout on AXI write response bus"]
    #[inline(always)]
    pub fn timeout_b(&mut self) -> TimeoutBW<'_, IntEnableSpec> {
        TimeoutBW::new(self, 9)
    }
    #[doc = "Bit 10 - A write issued by this channel caused a timeout on AXI write data bus"]
    #[inline(always)]
    pub fn timeout_w(&mut self) -> TimeoutWW<'_, IntEnableSpec> {
        TimeoutWW::new(self, 10)
    }
    #[doc = "Bit 11 - A write issued by this channel caused a timeout on AXI write command bus"]
    #[inline(always)]
    pub fn timeout_aw(&mut self) -> TimeoutAwW<'_, IntEnableSpec> {
        TimeoutAwW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel is active but did not start a burst for 2048 cycles"]
    #[inline(always)]
    pub fn wdt_timeout(&mut self) -> WdtTimeoutW<'_, IntEnableSpec> {
        WdtTimeoutW::new(self, 12)
    }
}
#[doc = "Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnableSpec;
impl crate::RegisterSpec for IntEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_enable::R`](R) reader structure"]
impl crate::Readable for IntEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`int_enable::W`](W) writer structure"]
impl crate::Writable for IntEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENABLE to value 0x1fff"]
impl crate::Resettable for IntEnableSpec {
    const RESET_VALUE: u32 = 0x1fff;
}
