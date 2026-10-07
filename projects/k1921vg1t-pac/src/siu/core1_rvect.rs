#[doc = "Register `CORE1_RVECT` reader"]
pub type R = crate::R<Core1RvectSpec>;
#[doc = "Register `CORE1_RVECT` writer"]
pub type W = crate::W<Core1RvectSpec>;
#[doc = "Field `RVECT` reader - Reset vector (affected only in DUALCORE mode)"]
pub type RvectR = crate::FieldReader<u32>;
#[doc = "Field `RVECT` writer - Reset vector (affected only in DUALCORE mode)"]
pub type RvectW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reset vector (affected only in DUALCORE mode)"]
    #[inline(always)]
    pub fn rvect(&self) -> RvectR {
        RvectR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reset vector (affected only in DUALCORE mode)"]
    #[inline(always)]
    pub fn rvect(&mut self) -> RvectW<'_, Core1RvectSpec> {
        RvectW::new(self, 0)
    }
}
#[doc = "Core 1 reset vector register\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_rvect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_rvect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Core1RvectSpec;
impl crate::RegisterSpec for Core1RvectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core1_rvect::R`](R) reader structure"]
impl crate::Readable for Core1RvectSpec {}
#[doc = "`write(|w| ..)` method takes [`core1_rvect::W`](W) writer structure"]
impl crate::Writable for Core1RvectSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORE1_RVECT to value 0"]
impl crate::Resettable for Core1RvectSpec {}
