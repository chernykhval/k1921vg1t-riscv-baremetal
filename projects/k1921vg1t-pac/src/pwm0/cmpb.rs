#[doc = "Register `CMPB` reader"]
pub type R = crate::R<CmpbSpec>;
#[doc = "Register `CMPB` writer"]
pub type W = crate::W<CmpbSpec>;
#[doc = "Field `CMPB` reader - The value compared to the time-base counter (TBCTR)"]
pub type CmpbR = crate::FieldReader<u16>;
#[doc = "Field `CMPB` writer - The value compared to the time-base counter (TBCTR)"]
pub type CmpbW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - The value compared to the time-base counter (TBCTR)"]
    #[inline(always)]
    pub fn cmpb(&self) -> CmpbR {
        CmpbR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - The value compared to the time-base counter (TBCTR)"]
    #[inline(always)]
    pub fn cmpb(&mut self) -> CmpbW<'_, CmpbSpec> {
        CmpbW::new(self, 16)
    }
}
#[doc = "Counter-Compare B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpbSpec;
impl crate::RegisterSpec for CmpbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpb::R`](R) reader structure"]
impl crate::Readable for CmpbSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpb::W`](W) writer structure"]
impl crate::Writable for CmpbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPB to value 0"]
impl crate::Resettable for CmpbSpec {}
