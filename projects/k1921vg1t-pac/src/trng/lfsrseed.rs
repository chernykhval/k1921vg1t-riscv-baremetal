#[doc = "Register `LFSRSEED` reader"]
pub type R = crate::R<LfsrseedSpec>;
#[doc = "Register `LFSRSEED` writer"]
pub type W = crate::W<LfsrseedSpec>;
#[doc = "Field `SEED` reader - LFSR initialization value"]
pub type SeedR = crate::FieldReader<u32>;
#[doc = "Field `SEED` writer - LFSR initialization value"]
pub type SeedW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `SEL` writer - Share index for which initialization value should be used"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:23 - LFSR initialization value"]
    #[inline(always)]
    pub fn seed(&self) -> SeedR {
        SeedR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - LFSR initialization value"]
    #[inline(always)]
    pub fn seed(&mut self) -> SeedW<'_, LfsrseedSpec> {
        SeedW::new(self, 0)
    }
    #[doc = "Bits 24:25 - Share index for which initialization value should be used"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, LfsrseedSpec> {
        SelW::new(self, 24)
    }
}
#[doc = "LFSR Seed register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfsrseed::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfsrseed::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LfsrseedSpec;
impl crate::RegisterSpec for LfsrseedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfsrseed::R`](R) reader structure"]
impl crate::Readable for LfsrseedSpec {}
#[doc = "`write(|w| ..)` method takes [`lfsrseed::W`](W) writer structure"]
impl crate::Writable for LfsrseedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFSRSEED to value 0x00ff_ffff"]
impl crate::Resettable for LfsrseedSpec {
    const RESET_VALUE: u32 = 0x00ff_ffff;
}
