#[doc = "Register `DATA12` reader"]
pub type R = crate::R<Data12Spec>;
#[doc = "Register `DATA12` writer"]
pub type W = crate::W<Data12Spec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - "]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, Data12Spec> {
        ValW::new(self, 0)
    }
}
#[doc = "Output data register (12-bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`data12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Data12Spec;
impl crate::RegisterSpec for Data12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data12::R`](R) reader structure"]
impl crate::Readable for Data12Spec {}
#[doc = "`write(|w| ..)` method takes [`data12::W`](W) writer structure"]
impl crate::Writable for Data12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA12 to value 0"]
impl crate::Resettable for Data12Spec {}
