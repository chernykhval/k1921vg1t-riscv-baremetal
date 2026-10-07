#[doc = "Register `HSECFG` reader"]
pub type R = crate::R<HsecfgSpec>;
#[doc = "Register `HSECFG` writer"]
pub type W = crate::W<HsecfgSpec>;
#[doc = "Field `EN` reader - Enable HSE"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable HSE"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DS` reader - Drive current select pins"]
pub type DsR = crate::FieldReader;
#[doc = "Field `DS` writer - Drive current select pins"]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable HSE"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Drive current select pins"]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable HSE"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, HsecfgSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Drive current select pins"]
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<'_, HsecfgSpec> {
        DsW::new(self, 4)
    }
}
#[doc = "HSE Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsecfgSpec;
impl crate::RegisterSpec for HsecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsecfg::R`](R) reader structure"]
impl crate::Readable for HsecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`hsecfg::W`](W) writer structure"]
impl crate::Writable for HsecfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSECFG to value 0x01"]
impl crate::Resettable for HsecfgSpec {
    const RESET_VALUE: u32 = 0x01;
}
