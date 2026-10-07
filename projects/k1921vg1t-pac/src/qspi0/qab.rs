#[doc = "Register `QAB` reader"]
pub type R = crate::R<QabSpec>;
#[doc = "Register `QAB` writer"]
pub type W = crate::W<QabSpec>;
#[doc = "Field `VAL` reader - "]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, QabSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "QSPI Alternate Bytes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qab::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qab::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QabSpec;
impl crate::RegisterSpec for QabSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qab::R`](R) reader structure"]
impl crate::Readable for QabSpec {}
#[doc = "`write(|w| ..)` method takes [`qab::W`](W) writer structure"]
impl crate::Writable for QabSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QAB to value 0"]
impl crate::Resettable for QabSpec {}
