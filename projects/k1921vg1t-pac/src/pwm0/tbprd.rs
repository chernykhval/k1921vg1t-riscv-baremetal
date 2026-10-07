#[doc = "Register `TBPRD` reader"]
pub type R = crate::R<TbprdSpec>;
#[doc = "Register `TBPRD` writer"]
pub type W = crate::W<TbprdSpec>;
#[doc = "Field `VAL` reader - Period of the time-base counter"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Period of the time-base counter"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Period of the time-base counter"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Period of the time-base counter"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, TbprdSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Time-Base Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbprd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbprd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbprdSpec;
impl crate::RegisterSpec for TbprdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbprd::R`](R) reader structure"]
impl crate::Readable for TbprdSpec {}
#[doc = "`write(|w| ..)` method takes [`tbprd::W`](W) writer structure"]
impl crate::Writable for TbprdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBPRD to value 0"]
impl crate::Resettable for TbprdSpec {}
