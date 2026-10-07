#[doc = "Register `CPESTAT` reader"]
pub type R = crate::R<CpestatSpec>;
#[doc = "Field `CPE` reader - Status of CPE PAD latched"]
pub type CpeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of CPE PAD latched"]
    #[inline(always)]
    pub fn cpe(&self) -> CpeR {
        CpeR::new((self.bits & 1) != 0)
    }
}
#[doc = "CPE status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpestat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpestatSpec;
impl crate::RegisterSpec for CpestatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpestat::R`](R) reader structure"]
impl crate::Readable for CpestatSpec {}
#[doc = "`reset()` method sets CPESTAT to value 0"]
impl crate::Resettable for CpestatSpec {}
