#[doc = "Register `HWCFG` reader"]
pub type R = crate::R<HwcfgSpec>;
#[doc = "Field `INVCOUNT` reader - Generic g_NbOfInverters value"]
pub type InvcountR = crate::FieldReader;
#[doc = "Field `AUTOCORRTEST` reader - Generic g_Log2NbOfAutoCorrTestsPerShare value"]
pub type AutocorrtestR = crate::FieldReader;
#[doc = "Field `FIFODEP` reader - Generic g_Log2FifoDepth value"]
pub type FifodepR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Generic g_NbOfInverters value"]
    #[inline(always)]
    pub fn invcount(&self) -> InvcountR {
        InvcountR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Generic g_Log2NbOfAutoCorrTestsPerShare value"]
    #[inline(always)]
    pub fn autocorrtest(&self) -> AutocorrtestR {
        AutocorrtestR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Generic g_Log2FifoDepth value"]
    #[inline(always)]
    pub fn fifodep(&self) -> FifodepR {
        FifodepR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Hardware Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwcfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwcfgSpec;
impl crate::RegisterSpec for HwcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwcfg::R`](R) reader structure"]
impl crate::Readable for HwcfgSpec {}
#[doc = "`reset()` method sets HWCFG to value 0x511f"]
impl crate::Resettable for HwcfgSpec {
    const RESET_VALUE: u32 = 0x511f;
}
