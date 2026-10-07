#[doc = "Register `FRAC` reader"]
pub type R = crate::R<FracSpec>;
#[doc = "Register `FRAC` writer"]
pub type W = crate::W<FracSpec>;
#[doc = "Field `FRAC` reader - Numerator of fractional part"]
pub type FracR = crate::FieldReader<u32>;
#[doc = "Field `FRAC` writer - Numerator of fractional part"]
pub type FracW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Numerator of fractional part"]
    #[inline(always)]
    pub fn frac(&self) -> FracR {
        FracR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Numerator of fractional part"]
    #[inline(always)]
    pub fn frac(&mut self) -> FracW<'_, FracSpec> {
        FracW::new(self, 0)
    }
}
#[doc = "PLL FRAC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FracSpec;
impl crate::RegisterSpec for FracSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frac::R`](R) reader structure"]
impl crate::Readable for FracSpec {}
#[doc = "`write(|w| ..)` method takes [`frac::W`](W) writer structure"]
impl crate::Writable for FracSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAC to value 0"]
impl crate::Resettable for FracSpec {}
