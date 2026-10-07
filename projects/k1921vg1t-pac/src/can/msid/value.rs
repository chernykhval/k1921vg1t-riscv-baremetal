#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Field `INDEX` reader - Number field waiting bit"]
pub type IndexR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Number field waiting bit"]
    #[inline(always)]
    pub fn index(&self) -> IndexR {
        IndexR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "Register messages index0\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`reset()` method sets VALUE to value 0x20"]
impl crate::Resettable for ValueSpec {
    const RESET_VALUE: u32 = 0x20;
}
