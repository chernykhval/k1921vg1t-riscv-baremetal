#[doc = "Register `HOSTEN` reader"]
pub type R = crate::R<HostenSpec>;
#[doc = "Register `HOSTEN` writer"]
pub type W = crate::W<HostenSpec>;
#[doc = "Field `HOSTEN` reader - Congifure USB output as a host"]
pub type HostenR = crate::BitReader;
#[doc = "Field `HOSTEN` writer - Congifure USB output as a host"]
pub type HostenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Congifure USB output as a host"]
    #[inline(always)]
    pub fn hosten(&self) -> HostenR {
        HostenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Congifure USB output as a host"]
    #[inline(always)]
    pub fn hosten(&mut self) -> HostenW<'_, HostenSpec> {
        HostenW::new(self, 0)
    }
}
#[doc = "USB Host Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`hosten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hosten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostenSpec;
impl crate::RegisterSpec for HostenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hosten::R`](R) reader structure"]
impl crate::Readable for HostenSpec {}
#[doc = "`write(|w| ..)` method takes [`hosten::W`](W) writer structure"]
impl crate::Writable for HostenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOSTEN to value 0"]
impl crate::Resettable for HostenSpec {}
