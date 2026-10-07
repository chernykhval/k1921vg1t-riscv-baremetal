#[doc = "Register `WROFFSET` reader"]
pub type R = crate::R<WroffsetSpec>;
#[doc = "Field `WR_OFFSET` reader - Offset from the beginning of the buffer"]
pub type WrOffsetR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Offset from the beginning of the buffer"]
    #[inline(always)]
    pub fn wr_offset(&self) -> WrOffsetR {
        WrOffsetR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Channels write offset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`wroffset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WroffsetSpec;
impl crate::RegisterSpec for WroffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wroffset::R`](R) reader structure"]
impl crate::Readable for WroffsetSpec {}
#[doc = "`reset()` method sets WROFFSET to value 0"]
impl crate::Resettable for WroffsetSpec {}
