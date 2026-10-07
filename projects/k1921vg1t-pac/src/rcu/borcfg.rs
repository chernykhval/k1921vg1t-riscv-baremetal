#[doc = "Register `BORCFG` reader"]
pub type R = crate::R<BorcfgSpec>;
#[doc = "Register `BORCFG` writer"]
pub type W = crate::W<BorcfgSpec>;
#[doc = "Field `BOR_EN` reader - Enable Brown-Out reset generation"]
pub type BorEnR = crate::BitReader;
#[doc = "Field `BOR_EN` writer - Enable Brown-Out reset generation"]
pub type BorEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOR_TUNE` reader - Tune BOR level"]
pub type BorTuneR = crate::FieldReader;
#[doc = "Field `BOR_TUNE` writer - Tune BOR level"]
pub type BorTuneW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable Brown-Out reset generation"]
    #[inline(always)]
    pub fn bor_en(&self) -> BorEnR {
        BorEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Tune BOR level"]
    #[inline(always)]
    pub fn bor_tune(&self) -> BorTuneR {
        BorTuneR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Brown-Out reset generation"]
    #[inline(always)]
    pub fn bor_en(&mut self) -> BorEnW<'_, BorcfgSpec> {
        BorEnW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Tune BOR level"]
    #[inline(always)]
    pub fn bor_tune(&mut self) -> BorTuneW<'_, BorcfgSpec> {
        BorTuneW::new(self, 4)
    }
}
#[doc = "Broun-Out Reset Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`borcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`borcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BorcfgSpec;
impl crate::RegisterSpec for BorcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`borcfg::R`](R) reader structure"]
impl crate::Readable for BorcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`borcfg::W`](W) writer structure"]
impl crate::Writable for BorcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BORCFG to value 0"]
impl crate::Resettable for BorcfgSpec {}
