#[doc = "Register `RDOFFSET` reader"]
pub type R = crate::R<RdoffsetSpec>;
#[doc = "Field `RD_OFFSET` reader - Offset from the beginning of the buffer"]
pub type RdOffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Offset from the beginning of the buffer"]
    #[inline(always)]
    pub fn rd_offset(&self) -> RdOffsetR {
        RdOffsetR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channels read offset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdoffset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdoffsetSpec;
impl crate::RegisterSpec for RdoffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdoffset::R`](R) reader structure"]
impl crate::Readable for RdoffsetSpec {}
#[doc = "`reset()` method sets RDOFFSET to value 0"]
impl crate::Resettable for RdoffsetSpec {}
