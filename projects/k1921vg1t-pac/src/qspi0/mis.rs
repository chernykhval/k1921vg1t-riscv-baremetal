#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `TCMIS` reader - Transfer Complete masked interrupt status"]
pub type TcmisR = crate::BitReader;
#[doc = "Field `TSMIS` reader - Transfer Stall masked interrupt status"]
pub type TsmisR = crate::BitReader;
#[doc = "Field `TEMIS` reader - Tx FIFO Empty masked interrupt status"]
pub type TemisR = crate::BitReader;
#[doc = "Field `TFMIS` reader - Tx FIFO Full masked interrupt status"]
pub type TfmisR = crate::BitReader;
#[doc = "Field `REMIS` reader - Rx FIFO Empty masked interrupt status"]
pub type RemisR = crate::BitReader;
#[doc = "Field `RFMIS` reader - Rx FIFO Full masked interrupt status"]
pub type RfmisR = crate::BitReader;
#[doc = "Field `TWMIS` reader - Tx FIFO Watermark masked interrupt status"]
pub type TwmisR = crate::BitReader;
#[doc = "Field `RWMIS` reader - Rx FIFO Watermark masked interrupt status"]
pub type RwmisR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Transfer Complete masked interrupt status"]
    #[inline(always)]
    pub fn tcmis(&self) -> TcmisR {
        TcmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Stall masked interrupt status"]
    #[inline(always)]
    pub fn tsmis(&self) -> TsmisR {
        TsmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO Empty masked interrupt status"]
    #[inline(always)]
    pub fn temis(&self) -> TemisR {
        TemisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx FIFO Full masked interrupt status"]
    #[inline(always)]
    pub fn tfmis(&self) -> TfmisR {
        TfmisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO Empty masked interrupt status"]
    #[inline(always)]
    pub fn remis(&self) -> RemisR {
        RemisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO Full masked interrupt status"]
    #[inline(always)]
    pub fn rfmis(&self) -> RfmisR {
        RfmisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx FIFO Watermark masked interrupt status"]
    #[inline(always)]
    pub fn twmis(&self) -> TwmisR {
        TwmisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rx FIFO Watermark masked interrupt status"]
    #[inline(always)]
    pub fn rwmis(&self) -> RwmisR {
        RwmisR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
