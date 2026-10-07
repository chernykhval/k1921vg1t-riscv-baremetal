#[doc = "Register `ACSTATUS` reader"]
pub type R = crate::R<AcstatusSpec>;
#[doc = "Field `STATE` reader - Comparator output pin status"]
pub type StateR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Comparator output pin status"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new((self.bits & 1) != 0)
    }
}
#[doc = "ACMP status\n\nYou can [`read`](crate::Reg::read) this register and get [`acstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcstatusSpec;
impl crate::RegisterSpec for AcstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acstatus::R`](R) reader structure"]
impl crate::Readable for AcstatusSpec {}
#[doc = "`reset()` method sets ACSTATUS to value 0"]
impl crate::Resettable for AcstatusSpec {}
