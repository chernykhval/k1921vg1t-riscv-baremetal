#[doc = "Register `CH_STATUS` reader"]
pub type R = crate::R<ChStatusSpec>;
#[doc = "Field `CH_RD_ACTIVE` reader - Read on AXI bus is performed"]
pub type ChRdActiveR = crate::BitReader;
#[doc = "Field `CH_WR_ACTIVE` reader - Write on AXI bus is performed"]
pub type ChWrActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Read on AXI bus is performed"]
    #[inline(always)]
    pub fn ch_rd_active(&self) -> ChRdActiveR {
        ChRdActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write on AXI bus is performed"]
    #[inline(always)]
    pub fn ch_wr_active(&self) -> ChWrActiveR {
        ChWrActiveR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Channel status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChStatusSpec;
impl crate::RegisterSpec for ChStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_status::R`](R) reader structure"]
impl crate::Readable for ChStatusSpec {}
#[doc = "`reset()` method sets CH_STATUS to value 0"]
impl crate::Resettable for ChStatusSpec {}
