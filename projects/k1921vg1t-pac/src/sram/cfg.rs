#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `EN` reader - Enable ECC"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable ECC"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRINJEN` reader - Enable error injection on write data bus"]
pub type ErrinjenR = crate::BitReader;
#[doc = "Field `ERRINJEN` writer - Enable error injection on write data bus"]
pub type ErrinjenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ECC"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable error injection on write data bus"]
    #[inline(always)]
    pub fn errinjen(&self) -> ErrinjenR {
        ErrinjenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, CfgSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable error injection on write data bus"]
    #[inline(always)]
    pub fn errinjen(&mut self) -> ErrinjenW<'_, CfgSpec> {
        ErrinjenW::new(self, 8)
    }
}
#[doc = "SRAM ECC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
