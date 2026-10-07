#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `INIT_DONE` reader - Indicate that SDRAM Initialization has done"]
pub type InitDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicate that SDRAM Initialization has done"]
    #[inline(always)]
    pub fn init_done(&self) -> InitDoneR {
        InitDoneR::new((self.bits & 1) != 0)
    }
}
#[doc = "SDRAM Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
