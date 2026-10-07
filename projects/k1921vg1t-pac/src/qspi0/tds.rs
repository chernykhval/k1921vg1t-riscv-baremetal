#[doc = "Register `TDS` reader"]
pub type R = crate::R<TdsSpec>;
#[doc = "Register `TDS` writer"]
pub type W = crate::W<TdsSpec>;
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
    pub fn val(&mut self) -> ValW<'_, TdsSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Transfer Data size register\n\nYou can [`read`](crate::Reg::read) this register and get [`tds::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tds::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdsSpec;
impl crate::RegisterSpec for TdsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tds::R`](R) reader structure"]
impl crate::Readable for TdsSpec {}
#[doc = "`write(|w| ..)` method takes [`tds::W`](W) writer structure"]
impl crate::Writable for TdsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDS to value 0"]
impl crate::Resettable for TdsSpec {}
