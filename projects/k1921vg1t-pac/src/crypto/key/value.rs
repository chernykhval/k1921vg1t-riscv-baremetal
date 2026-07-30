#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `VAL` reader - Key registers (word i). Bits 32*(8-i)-1 to 32*(7-i) for AES_256, Magma, Kuznechik / Bits 32*(4-i)-1 to 32*(3-i) for AES_128 (i < 4)"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Key registers (word i). Bits 32*(8-i)-1 to 32*(7-i) for AES_256, Magma, Kuznechik / Bits 32*(4-i)-1 to 32*(3-i) for AES_128 (i < 4)"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Key registers (word i). Bits 32*(8-i)-1 to 32*(7-i) for AES_256, Magma, Kuznechik / Bits 32*(4-i)-1 to 32*(3-i) for AES_128 (i < 4)"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key registers (word i). Bits 32*(8-i)-1 to 32*(7-i) for AES_256, Magma, Kuznechik / Bits 32*(4-i)-1 to 32*(3-i) for AES_128 (i < 4)"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, ValueSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Secret key register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {}
