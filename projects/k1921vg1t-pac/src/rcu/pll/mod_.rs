#[doc = "Register `MOD` reader"]
pub type R = crate::R<ModSpec>;
#[doc = "Register `MOD` writer"]
pub type W = crate::W<ModSpec>;
#[doc = "Field `MOD` reader - Denominator of fractional part"]
pub type ModR = crate::FieldReader<u32>;
#[doc = "Field `MOD` writer - Denominator of fractional part"]
pub type ModW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Denominator of fractional part"]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Denominator of fractional part"]
    #[inline(always)]
    pub fn mod_(&mut self) -> ModW<'_, ModSpec> {
        ModW::new(self, 0)
    }
}
#[doc = "PLL MOD Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModSpec;
impl crate::RegisterSpec for ModSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod_::R`](R) reader structure"]
impl crate::Readable for ModSpec {}
#[doc = "`write(|w| ..)` method takes [`mod_::W`](W) writer structure"]
impl crate::Writable for ModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for ModSpec {}
