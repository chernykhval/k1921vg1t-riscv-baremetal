#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `TCRIS` reader - Transfer Complete raw interrupt status"]
pub type TcrisR = crate::BitReader;
#[doc = "Field `TSRIS` reader - Transfer Stall raw interrupt status"]
pub type TsrisR = crate::BitReader;
#[doc = "Field `TERIS` reader - Tx FIFO Empty raw interrupt status"]
pub type TerisR = crate::BitReader;
#[doc = "Field `TFRIS` reader - Tx FIFO Full raw interrupt status"]
pub type TfrisR = crate::BitReader;
#[doc = "Field `RERIS` reader - Rx FIFO Empty raw interrupt status"]
pub type RerisR = crate::BitReader;
#[doc = "Field `RFRIS` reader - Rx FIFO Full raw interrupt status"]
pub type RfrisR = crate::BitReader;
#[doc = "Field `TWMRIS` reader - Tx FIFO Watermark raw interrupt status"]
pub type TwmrisR = crate::BitReader;
#[doc = "Field `RWMRIS` reader - Rx FIFO Watermark raw interrupt status"]
pub type RwmrisR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Transfer Complete raw interrupt status"]
    #[inline(always)]
    pub fn tcris(&self) -> TcrisR {
        TcrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Stall raw interrupt status"]
    #[inline(always)]
    pub fn tsris(&self) -> TsrisR {
        TsrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO Empty raw interrupt status"]
    #[inline(always)]
    pub fn teris(&self) -> TerisR {
        TerisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx FIFO Full raw interrupt status"]
    #[inline(always)]
    pub fn tfris(&self) -> TfrisR {
        TfrisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO Empty raw interrupt status"]
    #[inline(always)]
    pub fn reris(&self) -> RerisR {
        RerisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO Full raw interrupt status"]
    #[inline(always)]
    pub fn rfris(&self) -> RfrisR {
        RfrisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx FIFO Watermark raw interrupt status"]
    #[inline(always)]
    pub fn twmris(&self) -> TwmrisR {
        TwmrisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx FIFO Watermark raw interrupt status"]
    #[inline(always)]
    pub fn rwmris(&self) -> RwmrisR {
        RwmrisR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {}
