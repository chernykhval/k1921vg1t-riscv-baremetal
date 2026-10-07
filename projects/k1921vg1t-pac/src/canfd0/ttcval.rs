#[doc = "Register `TTCVAL` reader"]
pub type R = crate::R<TtcvalSpec>;
#[doc = "Field `TIMTTC` reader - CAN TTC Timer Value"]
pub type TimttcR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CAN TTC Timer Value"]
    #[inline(always)]
    pub fn timttc(&self) -> TimttcR {
        TimttcR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CAN TTC Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ttcval::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TtcvalSpec;
impl crate::RegisterSpec for TtcvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttcval::R`](R) reader structure"]
impl crate::Readable for TtcvalSpec {}
#[doc = "`reset()` method sets TTCVAL to value 0"]
impl crate::Resettable for TtcvalSpec {}
