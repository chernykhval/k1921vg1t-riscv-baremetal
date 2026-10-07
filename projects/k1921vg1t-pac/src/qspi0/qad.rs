#[doc = "Register `QAD` reader"]
pub type R = crate::R<QadSpec>;
#[doc = "Register `QAD` writer"]
pub type W = crate::W<QadSpec>;
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
    pub fn val(&mut self) -> ValW<'_, QadSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "QSPI Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qad::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qad::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QadSpec;
impl crate::RegisterSpec for QadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qad::R`](R) reader structure"]
impl crate::Readable for QadSpec {}
#[doc = "`write(|w| ..)` method takes [`qad::W`](W) writer structure"]
impl crate::Writable for QadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets QAD to value 0"]
impl crate::Resettable for QadSpec {}
