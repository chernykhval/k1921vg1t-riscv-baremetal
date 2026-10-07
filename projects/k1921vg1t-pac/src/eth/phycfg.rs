#[doc = "Register `PHYCFG` reader"]
pub type R = crate::R<PhycfgSpec>;
#[doc = "Register `PHYCFG` writer"]
pub type W = crate::W<PhycfgSpec>;
#[doc = "Field `EXTPHY` reader - Use external PHY Layer"]
pub type ExtphyR = crate::BitReader;
#[doc = "Field `EXTPHY` writer - Use external PHY Layer"]
pub type ExtphyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Use external PHY Layer"]
    #[inline(always)]
    pub fn extphy(&self) -> ExtphyR {
        ExtphyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Use external PHY Layer"]
    #[inline(always)]
    pub fn extphy(&mut self) -> ExtphyW<'_, PhycfgSpec> {
        ExtphyW::new(self, 0)
    }
}
#[doc = "ETH PHY control register\n\nYou can [`read`](crate::Reg::read) this register and get [`phycfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phycfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhycfgSpec;
impl crate::RegisterSpec for PhycfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phycfg::R`](R) reader structure"]
impl crate::Readable for PhycfgSpec {}
#[doc = "`write(|w| ..)` method takes [`phycfg::W`](W) writer structure"]
impl crate::Writable for PhycfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHYCFG to value 0x01"]
impl crate::Resettable for PhycfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
