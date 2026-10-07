#[doc = "Register `D_STAT` reader"]
pub type R = crate::R<DStatSpec>;
#[doc = "Field `DD` reader - Downstream Disabled flag"]
pub type DdR = crate::BitReader;
#[doc = "Field `CP` reader - Command Pending flag"]
pub type CpR = crate::BitReader;
#[doc = "Field `DP` reader - Data Pending flag"]
pub type DpR = crate::BitReader;
#[doc = "Field `CFA` reader - Command Frame Active flag"]
pub type CfaR = crate::BitReader;
#[doc = "Field `DFA` reader - Data Frame Active flag"]
pub type DfaR = crate::BitReader;
#[doc = "Field `BC` reader - Downstream bits counter"]
pub type BcR = crate::FieldReader;
#[doc = "Field `TFC` reader - passive Time Frames counter"]
pub type TfcR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Downstream Disabled flag"]
    #[inline(always)]
    pub fn dd(&self) -> DdR {
        DdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Pending flag"]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Pending flag"]
    #[inline(always)]
    pub fn dp(&self) -> DpR {
        DpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command Frame Active flag"]
    #[inline(always)]
    pub fn cfa(&self) -> CfaR {
        CfaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Frame Active flag"]
    #[inline(always)]
    pub fn dfa(&self) -> DfaR {
        DfaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Downstream bits counter"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - passive Time Frames counter"]
    #[inline(always)]
    pub fn tfc(&self) -> TfcR {
        TfcR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Downstream status register\n\nYou can [`read`](crate::Reg::read) this register and get [`d_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DStatSpec;
impl crate::RegisterSpec for DStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d_stat::R`](R) reader structure"]
impl crate::Readable for DStatSpec {}
#[doc = "`reset()` method sets D_STAT to value 0"]
impl crate::Resettable for DStatSpec {}
