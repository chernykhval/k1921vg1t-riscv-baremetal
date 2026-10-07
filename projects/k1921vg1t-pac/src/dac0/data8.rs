#[doc = "Register `DATA8` reader"]
pub type R = crate::R<Data8Spec>;
#[doc = "Register `DATA8` writer"]
pub type W = crate::W<Data8Spec>;
#[doc = "Field `VAL` reader - Write 8 most significant bits (4 less significant bits will be set to 0)"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Write 8 most significant bits (4 less significant bits will be set to 0)"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Write 8 most significant bits (4 less significant bits will be set to 0)"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Write 8 most significant bits (4 less significant bits will be set to 0)"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, Data8Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Output data register (8-bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`data8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data8Spec;
impl crate::RegisterSpec for Data8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data8::R`](R) reader structure"]
impl crate::Readable for Data8Spec {}
#[doc = "`write(|w| ..)` method takes [`data8::W`](W) writer structure"]
impl crate::Writable for Data8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA8 to value 0"]
impl crate::Resettable for Data8Spec {}
