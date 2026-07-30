#[doc = "Register `HWVERSION` reader"]
pub type R = crate::R<HwversionSpec>;
#[doc = "Field `HWVER` reader - "]
pub type HwverR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hwver(&self) -> HwverR {
        HwverR::new((self.bits & 1) != 0)
    }
}
#[doc = "Hardware Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwversion::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwversionSpec;
impl crate::RegisterSpec for HwversionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwversion::R`](R) reader structure"]
impl crate::Readable for HwversionSpec {}
#[doc = "`reset()` method sets HWVERSION to value 0x01"]
impl crate::Resettable for HwversionSpec {
    const RESET_VALUE: u32 = 0x01;
}
