#[doc = "Register `Reserved` reader"]
pub type R = crate::R<ReservedSpec>;
#[doc = "Register `Reserved` writer"]
pub type W = crate::W<ReservedSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReservedSpec;
impl crate::RegisterSpec for ReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved::R`](R) reader structure"]
impl crate::Readable for ReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`reserved::W`](W) writer structure"]
impl crate::Writable for ReservedSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Reserved to value 0"]
impl crate::Resettable for ReservedSpec {}
