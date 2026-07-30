#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Field `BEGIN` reader - Number of the first message object"]
pub type BeginR = crate::FieldReader;
#[doc = "Field `END` reader - Number of the last message object"]
pub type EndR = crate::FieldReader;
#[doc = "Field `SIZE` reader - List size"]
pub type SizeR = crate::FieldReader;
#[doc = "Field `EMPTY` reader - Indicate empty list"]
pub type EmptyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Number of the first message object"]
    #[inline(always)]
    pub fn begin(&self) -> BeginR {
        BeginR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of the last message object"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - List size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Indicate empty list"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Register list0\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`reset()` method sets VALUE to value 0x0100_0000"]
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
