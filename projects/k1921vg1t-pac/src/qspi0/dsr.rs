#[doc = "Register `DSR` reader"]
pub type R = crate::R<DsrSpec>;
#[doc = "Field `BUSY` reader - Flag BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `TIP` reader - Transfer In Progress"]
pub type TipR = crate::BitReader;
#[doc = "Field `TST` reader - Transfer Stall"]
pub type TstR = crate::BitReader;
#[doc = "Field `TFE` reader - Tx FIFO Empty"]
pub type TfeR = crate::BitReader;
#[doc = "Field `TFF` reader - Tx FIFO Full"]
pub type TffR = crate::BitReader;
#[doc = "Field `RFE` reader - Rx FIFO Empty"]
pub type RfeR = crate::BitReader;
#[doc = "Field `RFF` reader - Rx FIFO Full"]
pub type RffR = crate::BitReader;
#[doc = "Field `TXFCNT` reader - Curent Tx FIFO Element Count"]
pub type TxfcntR = crate::FieldReader;
#[doc = "Field `RXFCNT` reader - Curent Rx FIFO Element Count"]
pub type RxfcntR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Flag BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer In Progress"]
    #[inline(always)]
    pub fn tip(&self) -> TipR {
        TipR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Stall"]
    #[inline(always)]
    pub fn tst(&self) -> TstR {
        TstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO Empty"]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx FIFO Full"]
    #[inline(always)]
    pub fn tff(&self) -> TffR {
        TffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO Empty"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO Full"]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Curent Tx FIFO Element Count"]
    #[inline(always)]
    pub fn txfcnt(&self) -> TxfcntR {
        TxfcntR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Curent Rx FIFO Element Count"]
    #[inline(always)]
    pub fn rxfcnt(&self) -> RxfcntR {
        RxfcntR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Device status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsrSpec;
impl crate::RegisterSpec for DsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr::R`](R) reader structure"]
impl crate::Readable for DsrSpec {}
#[doc = "`reset()` method sets DSR to value 0x50"]
impl crate::Resettable for DsrSpec {
    const RESET_VALUE: u32 = 0x50;
}
