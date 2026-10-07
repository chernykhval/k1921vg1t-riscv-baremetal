#[doc = "Register `TBCTR` reader"]
pub type R = crate::R<TbctrSpec>;
#[doc = "Register `TBCTR` writer"]
pub type W = crate::W<TbctrSpec>;
#[doc = "Field `VAL` reader - Current time-base counter value"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Current time-base counter value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Current time-base counter value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Current time-base counter value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, TbctrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Time-Base Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbctrSpec;
impl crate::RegisterSpec for TbctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbctr::R`](R) reader structure"]
impl crate::Readable for TbctrSpec {}
#[doc = "`write(|w| ..)` method takes [`tbctr::W`](W) writer structure"]
impl crate::Writable for TbctrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBCTR to value 0"]
impl crate::Resettable for TbctrSpec {}
