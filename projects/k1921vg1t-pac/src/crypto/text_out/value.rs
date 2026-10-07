#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Field `VAL` reader - Text out registers (word i). Bits 32*(4-i)-1 to 32*(3-i) for AES_128, AES_256, Kuznechik / Bits 32*(2-i)-1 to 32*(1-i) for Magma (i < 2)"]
pub type ValR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Text out registers (word i). Bits 32*(4-i)-1 to 32*(3-i) for AES_128, AES_256, Kuznechik / Bits 32*(2-i)-1 to 32*(1-i) for Magma (i < 2)"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
#[doc = "Output text regiter 0 (cipher text / plain text)\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {}
