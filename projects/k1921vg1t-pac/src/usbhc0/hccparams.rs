#[doc = "Register `HCCPARAMS` reader"]
pub type R = crate::R<HccparamsSpec>;
#[doc = "Field `ADDR_CAP_64` reader - 64-bit Addressing Capability"]
pub type AddrCap64R = crate::BitReader;
#[doc = "Field `PFRAMELSTFLG` reader - Programmable Frame List Flag"]
pub type PframelstflgR = crate::BitReader;
#[doc = "Field `ASYNCSCHEDPK` reader - Asynchronous Schedule Park Capability"]
pub type AsyncschedpkR = crate::BitReader;
#[doc = "Field `ISSCHEDTHRES` reader - Isochronous Scheduling Threshold"]
pub type IsschedthresR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - 64-bit Addressing Capability"]
    #[inline(always)]
    pub fn addr_cap_64(&self) -> AddrCap64R {
        AddrCap64R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Programmable Frame List Flag"]
    #[inline(always)]
    pub fn pframelstflg(&self) -> PframelstflgR {
        PframelstflgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Asynchronous Schedule Park Capability"]
    #[inline(always)]
    pub fn asyncschedpk(&self) -> AsyncschedpkR {
        AsyncschedpkR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Isochronous Scheduling Threshold"]
    #[inline(always)]
    pub fn isschedthres(&self) -> IsschedthresR {
        IsschedthresR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "USBHC Capability Parameters register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccparams::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccparamsSpec;
impl crate::RegisterSpec for HccparamsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hccparams::R`](R) reader structure"]
impl crate::Readable for HccparamsSpec {}
#[doc = "`reset()` method sets HCCPARAMS to value 0x86"]
impl crate::Resettable for HccparamsSpec {
    const RESET_VALUE: u32 = 0x86;
}
