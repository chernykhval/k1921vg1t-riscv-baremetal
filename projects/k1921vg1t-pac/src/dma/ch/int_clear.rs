#[doc = "Register `INT_CLEAR` writer"]
pub type W = crate::W<IntClearSpec>;
#[doc = "Field `CH_END` writer - Channel end interrupt"]
pub type ChEndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_SLVERR` writer - A read issued by this channel caused an AXI read slave decode error"]
pub type RdSlverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_SLVERR` writer - A write issued by this channel caused an AXI write slave decode error"]
pub type WrSlverrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DECERR` writer - A read issued by this channel caused an AXI read decode error"]
pub type RdDecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_DECERR` writer - A write issued by this channel caused an AXI write decode error"]
pub type WrDecerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVERFLOW` writer - Data FIFO has been overflowed"]
pub type OverflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UNDERFLOW` writer - Data FIFO has been underflowed"]
pub type UnderflowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_R` writer - A read issued by this channel caused a timeout on AXI read data bus"]
pub type TimeoutRW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_AR` writer - A read issued by this channel caused a timeout on AXI read command bus"]
pub type TimeoutArW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_B` writer - A write issued by this channel caused a timeout on AXI write response bus"]
pub type TimeoutBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_W` writer - A write issued by this channel caused a timeout on AXI write data bus"]
pub type TimeoutWW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT_AW` writer - A write issued by this channel caused a timeout on AXI write command bus"]
pub type TimeoutAwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT_TIMEOUT` writer - Channel is active but did not start a burst for 2048 cycles"]
pub type WdtTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel end interrupt"]
    #[inline(always)]
    pub fn ch_end(&mut self) -> ChEndW<'_, IntClearSpec> {
        ChEndW::new(self, 0)
    }
    #[doc = "Bit 1 - A read issued by this channel caused an AXI read slave decode error"]
    #[inline(always)]
    pub fn rd_slverr(&mut self) -> RdSlverrW<'_, IntClearSpec> {
        RdSlverrW::new(self, 1)
    }
    #[doc = "Bit 2 - A write issued by this channel caused an AXI write slave decode error"]
    #[inline(always)]
    pub fn wr_slverr(&mut self) -> WrSlverrW<'_, IntClearSpec> {
        WrSlverrW::new(self, 2)
    }
    #[doc = "Bit 3 - A read issued by this channel caused an AXI read decode error"]
    #[inline(always)]
    pub fn rd_decerr(&mut self) -> RdDecerrW<'_, IntClearSpec> {
        RdDecerrW::new(self, 3)
    }
    #[doc = "Bit 4 - A write issued by this channel caused an AXI write decode error"]
    #[inline(always)]
    pub fn wr_decerr(&mut self) -> WrDecerrW<'_, IntClearSpec> {
        WrDecerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Data FIFO has been overflowed"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OverflowW<'_, IntClearSpec> {
        OverflowW::new(self, 5)
    }
    #[doc = "Bit 6 - Data FIFO has been underflowed"]
    #[inline(always)]
    pub fn underflow(&mut self) -> UnderflowW<'_, IntClearSpec> {
        UnderflowW::new(self, 6)
    }
    #[doc = "Bit 7 - A read issued by this channel caused a timeout on AXI read data bus"]
    #[inline(always)]
    pub fn timeout_r(&mut self) -> TimeoutRW<'_, IntClearSpec> {
        TimeoutRW::new(self, 7)
    }
    #[doc = "Bit 8 - A read issued by this channel caused a timeout on AXI read command bus"]
    #[inline(always)]
    pub fn timeout_ar(&mut self) -> TimeoutArW<'_, IntClearSpec> {
        TimeoutArW::new(self, 8)
    }
    #[doc = "Bit 9 - A write issued by this channel caused a timeout on AXI write response bus"]
    #[inline(always)]
    pub fn timeout_b(&mut self) -> TimeoutBW<'_, IntClearSpec> {
        TimeoutBW::new(self, 9)
    }
    #[doc = "Bit 10 - A write issued by this channel caused a timeout on AXI write data bus"]
    #[inline(always)]
    pub fn timeout_w(&mut self) -> TimeoutWW<'_, IntClearSpec> {
        TimeoutWW::new(self, 10)
    }
    #[doc = "Bit 11 - A write issued by this channel caused a timeout on AXI write command bus"]
    #[inline(always)]
    pub fn timeout_aw(&mut self) -> TimeoutAwW<'_, IntClearSpec> {
        TimeoutAwW::new(self, 11)
    }
    #[doc = "Bit 12 - Channel is active but did not start a burst for 2048 cycles"]
    #[inline(always)]
    pub fn wdt_timeout(&mut self) -> WdtTimeoutW<'_, IntClearSpec> {
        WdtTimeoutW::new(self, 12)
    }
}
#[doc = "Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntClearSpec;
impl crate::RegisterSpec for IntClearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clear::W`](W) writer structure"]
impl crate::Writable for IntClearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0"]
impl crate::Resettable for IntClearSpec {}
