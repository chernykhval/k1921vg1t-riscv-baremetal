#[doc = "Register `SWTRIG` writer"]
pub type W = crate::W<SwtrigSpec>;
#[doc = "Field `SWTRIG` writer - Software trigger"]
pub type SwtrigW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Software trigger"]
    #[inline(always)]
    pub fn swtrig(&mut self) -> SwtrigW<'_, SwtrigSpec> {
        SwtrigW::new(self, 0)
    }
}
#[doc = "Software trigger control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrig::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrigSpec;
impl crate::RegisterSpec for SwtrigSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrig::W`](W) writer structure"]
impl crate::Writable for SwtrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SwtrigSpec {}
