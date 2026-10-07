#[doc = "Register `HCSPARAMS` reader"]
pub type R = crate::R<HcsparamsSpec>;
#[doc = "Field `N_PORTS` reader - Number of physical downstream ports implemented"]
pub type NPortsR = crate::FieldReader;
#[doc = "Field `PPC` reader - Port Power Control"]
pub type PpcR = crate::BitReader;
#[doc = "Field `PRR` reader - Port Routing Rules"]
pub type PrrR = crate::BitReader;
#[doc = "Field `N_PCC` reader - Number of Ports per Companion Controller"]
pub type NPccR = crate::FieldReader;
#[doc = "Field `N_CC` reader - Number of Companion Controller"]
pub type NCcR = crate::FieldReader;
#[doc = "Field `P_INDICATOR` reader - Port Indicator"]
pub type PIndicatorR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Number of physical downstream ports implemented"]
    #[inline(always)]
    pub fn n_ports(&self) -> NPortsR {
        NPortsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Port Power Control"]
    #[inline(always)]
    pub fn ppc(&self) -> PpcR {
        PpcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Routing Rules"]
    #[inline(always)]
    pub fn prr(&self) -> PrrR {
        PrrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Ports per Companion Controller"]
    #[inline(always)]
    pub fn n_pcc(&self) -> NPccR {
        NPccR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of Companion Controller"]
    #[inline(always)]
    pub fn n_cc(&self) -> NCcR {
        NCcR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Port Indicator"]
    #[inline(always)]
    pub fn p_indicator(&self) -> PIndicatorR {
        PIndicatorR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "USBHC Structural Parameters register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsparams::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcsparamsSpec;
impl crate::RegisterSpec for HcsparamsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcsparams::R`](R) reader structure"]
impl crate::Readable for HcsparamsSpec {}
#[doc = "`reset()` method sets HCSPARAMS to value 0x1111"]
impl crate::Resettable for HcsparamsSpec {
    const RESET_VALUE: u32 = 0x1111;
}
