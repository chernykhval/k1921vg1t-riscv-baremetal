#[doc = "Register `NPCR` reader"]
pub type R = crate::R<NpcrSpec>;
#[doc = "Register `NPCR` writer"]
pub type W = crate::W<NpcrSpec>;
#[doc = "Field `LBM` reader - Enable mode Loop-Back"]
pub type LbmR = crate::BitReader;
#[doc = "Field `LBM` writer - Enable mode Loop-Back"]
pub type LbmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - Enable mode Loop-Back"]
    #[inline(always)]
    pub fn lbm(&self) -> LbmR {
        LbmR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Enable mode Loop-Back"]
    #[inline(always)]
    pub fn lbm(&mut self) -> LbmW<'_, NpcrSpec> {
        LbmW::new(self, 8)
    }
}
#[doc = "Port control register node0\n\nYou can [`read`](crate::Reg::read) this register and get [`npcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`npcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NpcrSpec;
impl crate::RegisterSpec for NpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`npcr::R`](R) reader structure"]
impl crate::Readable for NpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`npcr::W`](W) writer structure"]
impl crate::Writable for NpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NPCR to value 0"]
impl crate::Resettable for NpcrSpec {}
