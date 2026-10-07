#[doc = "Register `CMPA` reader"]
pub type R = crate::R<CmpaSpec>;
#[doc = "Register `CMPA` writer"]
pub type W = crate::W<CmpaSpec>;
#[doc = "Field `CMPAHR` reader - The value compared to the time-base counter (TBCTR) in High-Resolution mode"]
pub type CmpahrR = crate::FieldReader;
#[doc = "Field `CMPAHR` writer - The value compared to the time-base counter (TBCTR) in High-Resolution mode"]
pub type CmpahrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMPA` reader - The value compared to the time-base counter (TBCTR)"]
pub type CmpaR = crate::FieldReader<u16>;
#[doc = "Field `CMPA` writer - The value compared to the time-base counter (TBCTR)"]
pub type CmpaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 8:15 - The value compared to the time-base counter (TBCTR) in High-Resolution mode"]
    #[inline(always)]
    pub fn cmpahr(&self) -> CmpahrR {
        CmpahrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - The value compared to the time-base counter (TBCTR)"]
    #[inline(always)]
    pub fn cmpa(&self) -> CmpaR {
        CmpaR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:15 - The value compared to the time-base counter (TBCTR) in High-Resolution mode"]
    #[inline(always)]
    pub fn cmpahr(&mut self) -> CmpahrW<'_, CmpaSpec> {
        CmpahrW::new(self, 8)
    }
    #[doc = "Bits 16:31 - The value compared to the time-base counter (TBCTR)"]
    #[inline(always)]
    pub fn cmpa(&mut self) -> CmpaW<'_, CmpaSpec> {
        CmpaW::new(self, 16)
    }
}
#[doc = "Counter-Compare A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpaSpec;
impl crate::RegisterSpec for CmpaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpa::R`](R) reader structure"]
impl crate::Readable for CmpaSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpa::W`](W) writer structure"]
impl crate::Writable for CmpaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPA to value 0"]
impl crate::Resettable for CmpaSpec {}
