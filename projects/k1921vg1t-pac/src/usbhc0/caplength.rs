#[doc = "Register `CAPLENGTH` reader"]
pub type R = crate::R<CaplengthSpec>;
#[doc = "Field `CAPLENGTH` reader - Capabilities register length"]
pub type CaplengthR = crate::FieldReader;
#[doc = "Field `HCIVERSION` reader - Host Controller Interface Version Number"]
pub type HciversionR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Capabilities register length"]
    #[inline(always)]
    pub fn caplength(&self) -> CaplengthR {
        CaplengthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Host Controller Interface Version Number"]
    #[inline(always)]
    pub fn hciversion(&self) -> HciversionR {
        HciversionR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "USBHC Capabilities register\n\nYou can [`read`](crate::Reg::read) this register and get [`caplength::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CaplengthSpec;
impl crate::RegisterSpec for CaplengthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`caplength::R`](R) reader structure"]
impl crate::Readable for CaplengthSpec {}
#[doc = "`reset()` method sets CAPLENGTH to value 0x0100_0014"]
impl crate::Resettable for CaplengthSpec {
    const RESET_VALUE: u32 = 0x0100_0014;
}
