#[doc = "Register `FECCDEC` reader"]
pub type R = crate::R<FeccdecSpec>;
#[doc = "Register `FECCDEC` writer"]
pub type W = crate::W<FeccdecSpec>;
#[doc = "Field `VAL` reader - Double error times occurred"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Double error times occurred"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double error times occurred"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double error times occurred"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FeccdecSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Flash ECC Double Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccdec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccdec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeccdecSpec;
impl crate::RegisterSpec for FeccdecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feccdec::R`](R) reader structure"]
impl crate::Readable for FeccdecSpec {}
#[doc = "`write(|w| ..)` method takes [`feccdec::W`](W) writer structure"]
impl crate::Writable for FeccdecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FECCDEC to value 0"]
impl crate::Resettable for FeccdecSpec {}
