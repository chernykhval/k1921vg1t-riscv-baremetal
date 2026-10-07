#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Field `CH_END` reader - Channel end interrupt"]
pub type ChEndR = crate::BitReader;
#[doc = "Field `RD_SLVERR` reader - A read issued by this channel caused an AXI read slave decode error"]
pub type RdSlverrR = crate::BitReader;
#[doc = "Field `WR_SLVERR` reader - A write issued by this channel caused an AXI write slave decode error"]
pub type WrSlverrR = crate::BitReader;
#[doc = "Field `RD_DECERR` reader - A read issued by this channel caused an AXI read decode error"]
pub type RdDecerrR = crate::BitReader;
#[doc = "Field `WR_DECERR` reader - A write issued by this channel caused an AXI write decode error"]
pub type WrDecerrR = crate::BitReader;
#[doc = "Field `OVERFLOW` reader - Data FIFO has been overflowed"]
pub type OverflowR = crate::BitReader;
#[doc = "Field `UNDERFLOW` reader - Data FIFO has been underflowed"]
pub type UnderflowR = crate::BitReader;
#[doc = "Field `TIMEOUT_R` reader - A read issued by this channel caused a timeout on AXI read data bus"]
pub type TimeoutRR = crate::BitReader;
#[doc = "Field `TIMEOUT_AR` reader - A read issued by this channel caused a timeout on AXI read command bus"]
pub type TimeoutArR = crate::BitReader;
#[doc = "Field `TIMEOUT_B` reader - A write issued by this channel caused a timeout on AXI write response bus"]
pub type TimeoutBR = crate::BitReader;
#[doc = "Field `TIMEOUT_W` reader - A write issued by this channel caused a timeout on AXI write data bus"]
pub type TimeoutWR = crate::BitReader;
#[doc = "Field `TIMEOUT_AW` reader - A write issued by this channel caused a timeout on AXI write command bus"]
pub type TimeoutAwR = crate::BitReader;
#[doc = "Field `WDT_TIMEOUT` reader - Channel is active but did not start a burst for 2048 cycles"]
pub type WdtTimeoutR = crate::BitReader;
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
#[doc = "Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {}
