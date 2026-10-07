#[doc = "Register `SERVCTL` reader"]
pub type R = crate::R<ServctlSpec>;
#[doc = "Register `SERVCTL` writer"]
pub type W = crate::W<ServctlSpec>;
#[doc = "Field `SERVSTAT` reader - Service mode status"]
pub type ServstatR = crate::BitReader;
#[doc = "Field `SERVCMD` reader - Service mode erase comand/status"]
pub type ServcmdR = crate::BitReader;
#[doc = "Field `SERVCMD` writer - Service mode erase comand/status"]
pub type ServcmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Service mode status"]
    #[inline(always)]
    pub fn servstat(&self) -> ServstatR {
        ServstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Service mode erase comand/status"]
    #[inline(always)]
    pub fn servcmd(&self) -> ServcmdR {
        ServcmdR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Service mode erase comand/status"]
    #[inline(always)]
    pub fn servcmd(&mut self) -> ServcmdW<'_, ServctlSpec> {
        ServcmdW::new(self, 8)
    }
}
#[doc = "Service mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`servctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`servctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ServctlSpec;
impl crate::RegisterSpec for ServctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`servctl::R`](R) reader structure"]
impl crate::Readable for ServctlSpec {}
#[doc = "`write(|w| ..)` method takes [`servctl::W`](W) writer structure"]
impl crate::Writable for ServctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SERVCTL to value 0"]
impl crate::Resettable for ServctlSpec {}
