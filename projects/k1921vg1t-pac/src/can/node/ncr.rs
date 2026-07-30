#[doc = "Register `NCR` reader"]
pub type R = crate::R<NcrSpec>;
#[doc = "Register `NCR` writer"]
pub type W = crate::W<NcrSpec>;
#[doc = "Field `INIT` reader - Node initialization"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Node initialization"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIE` reader - Interrupt enable bit of the assembly at the end of the transmission / reception"]
pub type TrieR = crate::BitReader;
#[doc = "Field `TRIE` writer - Interrupt enable bit of the assembly at the end of the transmission / reception"]
pub type TrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LECIE` reader - Interrupt enable bit of the assembly when it detects the last error code"]
pub type LecieR = crate::BitReader;
#[doc = "Field `LECIE` writer - Interrupt enable bit of the assembly when it detects the last error code"]
pub type LecieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIE` reader - Enable interrupt ALERT from node"]
pub type AlieR = crate::BitReader;
#[doc = "Field `ALIE` writer - Enable interrupt ALERT from node"]
pub type AlieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANDIS` reader - Off node"]
pub type CandisR = crate::BitReader;
#[doc = "Field `CANDIS` writer - Off node"]
pub type CandisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - Permission node configuration changes"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - Permission node configuration changes"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALM` reader - Activate the node analysis"]
pub type CalmR = crate::BitReader;
#[doc = "Field `CALM` writer - Activate the node analysis"]
pub type CalmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Node initialization"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable bit of the assembly at the end of the transmission / reception"]
    #[inline(always)]
    pub fn trie(&self) -> TrieR {
        TrieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable bit of the assembly when it detects the last error code"]
    #[inline(always)]
    pub fn lecie(&self) -> LecieR {
        LecieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable interrupt ALERT from node"]
    #[inline(always)]
    pub fn alie(&self) -> AlieR {
        AlieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Off node"]
    #[inline(always)]
    pub fn candis(&self) -> CandisR {
        CandisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Permission node configuration changes"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activate the node analysis"]
    #[inline(always)]
    pub fn calm(&self) -> CalmR {
        CalmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Node initialization"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<'_, NcrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable bit of the assembly at the end of the transmission / reception"]
    #[inline(always)]
    pub fn trie(&mut self) -> TrieW<'_, NcrSpec> {
        TrieW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable bit of the assembly when it detects the last error code"]
    #[inline(always)]
    pub fn lecie(&mut self) -> LecieW<'_, NcrSpec> {
        LecieW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable interrupt ALERT from node"]
    #[inline(always)]
    pub fn alie(&mut self) -> AlieW<'_, NcrSpec> {
        AlieW::new(self, 3)
    }
    #[doc = "Bit 4 - Off node"]
    #[inline(always)]
    pub fn candis(&mut self) -> CandisW<'_, NcrSpec> {
        CandisW::new(self, 4)
    }
    #[doc = "Bit 6 - Permission node configuration changes"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<'_, NcrSpec> {
        CceW::new(self, 6)
    }
    #[doc = "Bit 7 - Activate the node analysis"]
    #[inline(always)]
    pub fn calm(&mut self) -> CalmW<'_, NcrSpec> {
        CalmW::new(self, 7)
    }
}
#[doc = "Register control node0\n\nYou can [`read`](crate::Reg::read) this register and get [`ncr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ncr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NcrSpec;
impl crate::RegisterSpec for NcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ncr::R`](R) reader structure"]
impl crate::Readable for NcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ncr::W`](W) writer structure"]
impl crate::Writable for NcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NCR to value 0x01"]
impl crate::Resettable for NcrSpec {
    const RESET_VALUE: u32 = 0x01;
}
