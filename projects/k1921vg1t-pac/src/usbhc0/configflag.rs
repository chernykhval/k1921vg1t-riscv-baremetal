#[doc = "Register `CONFIGFLAG` reader"]
pub type R = crate::R<ConfigflagSpec>;
#[doc = "Register `CONFIGFLAG` writer"]
pub type W = crate::W<ConfigflagSpec>;
#[doc = "Field `CF` reader - Configure Flag"]
pub type CfR = crate::BitReader;
#[doc = "Field `CF` writer - Configure Flag"]
pub type CfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure Flag"]
    #[inline(always)]
    pub fn cf(&self) -> CfR {
        CfR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configure Flag"]
    #[inline(always)]
    pub fn cf(&mut self) -> CfW<'_, ConfigflagSpec> {
        CfW::new(self, 0)
    }
}
#[doc = "USBHC EHC Configure Flag Register Bit Definitions register\n\nYou can [`read`](crate::Reg::read) this register and get [`configflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`configflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigflagSpec;
impl crate::RegisterSpec for ConfigflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`configflag::R`](R) reader structure"]
impl crate::Readable for ConfigflagSpec {}
#[doc = "`write(|w| ..)` method takes [`configflag::W`](W) writer structure"]
impl crate::Writable for ConfigflagSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIGFLAG to value 0"]
impl crate::Resettable for ConfigflagSpec {}
