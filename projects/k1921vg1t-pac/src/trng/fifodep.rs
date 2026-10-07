#[doc = "Register `FIFODEP` reader"]
pub type R = crate::R<FifodepSpec>;
#[doc = "Field `VAL` reader - Maximum number of 32 bits words that can be stored in the FIFO"]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Maximum number of 32 bits words that can be stored in the FIFO"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "FIFO Depth register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodep::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifodepSpec;
impl crate::RegisterSpec for FifodepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifodep::R`](R) reader structure"]
impl crate::Readable for FifodepSpec {}
#[doc = "`reset()` method sets FIFODEP to value 0x20"]
impl crate::Resettable for FifodepSpec {
    const RESET_VALUE: u32 = 0x20;
}
