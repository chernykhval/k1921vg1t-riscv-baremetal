#[doc = "Register `PERIODICLISTBASE` reader"]
pub type R = crate::R<PeriodiclistbaseSpec>;
#[doc = "Register `PERIODICLISTBASE` writer"]
pub type W = crate::W<PeriodiclistbaseSpec>;
#[doc = "Field `BASEADDRESS` reader - "]
pub type BaseaddressR = crate::FieldReader<u32>;
#[doc = "Field `BASEADDRESS` writer - "]
pub type BaseaddressW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn baseaddress(&self) -> BaseaddressR {
        BaseaddressR::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn baseaddress(&mut self) -> BaseaddressW<'_, PeriodiclistbaseSpec> {
        BaseaddressW::new(self, 12)
    }
}
#[doc = "USBHC EHC Periodic Frame List Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periodiclistbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periodiclistbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriodiclistbaseSpec;
impl crate::RegisterSpec for PeriodiclistbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periodiclistbase::R`](R) reader structure"]
impl crate::Readable for PeriodiclistbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`periodiclistbase::W`](W) writer structure"]
impl crate::Writable for PeriodiclistbaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIODICLISTBASE to value 0"]
impl crate::Resettable for PeriodiclistbaseSpec {}
