#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Field `RORMIS` reader - Masked interrupt status SSPRORINTR"]
pub type RormisR = crate::BitReader;
#[doc = "Field `RTMIS` reader - Masked interrupt status SSPRTINTR"]
pub type RtmisR = crate::BitReader;
#[doc = "Field `RXMIS` reader - Masked interrupt status SSPRXINTR"]
pub type RxmisR = crate::BitReader;
#[doc = "Field `TXMIS` reader - Masked interrupt status SSPTXINTR"]
pub type TxmisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Masked interrupt status SSPRORINTR"]
    #[inline(always)]
    pub fn rormis(&self) -> RormisR {
        RormisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt status SSPRTINTR"]
    #[inline(always)]
    pub fn rtmis(&self) -> RtmisR {
        RtmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked interrupt status SSPRXINTR"]
    #[inline(always)]
    pub fn rxmis(&self) -> RxmisR {
        RxmisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked interrupt status SSPTXINTR"]
    #[inline(always)]
    pub fn txmis(&self) -> TxmisR {
        TxmisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status register interrupt masking account\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {}
