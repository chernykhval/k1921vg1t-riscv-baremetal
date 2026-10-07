#[doc = "Register `TREC` reader"]
pub type R = crate::R<TrecSpec>;
#[doc = "Field `TEC` reader - Transmit Error Counter Value"]
pub type TecR = crate::FieldReader;
#[doc = "Field `REC` reader - Receive Error Counter Value"]
pub type RecR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter Value"]
    #[inline(always)]
    pub fn tec(&self) -> TecR {
        TecR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive Error Counter Value"]
    #[inline(always)]
    pub fn rec(&self) -> RecR {
        RecR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "CAN Transmit Error Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrecSpec;
impl crate::RegisterSpec for TrecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trec::R`](R) reader structure"]
impl crate::Readable for TrecSpec {}
#[doc = "`reset()` method sets TREC to value 0"]
impl crate::Resettable for TrecSpec {}
