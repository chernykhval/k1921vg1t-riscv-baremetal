#[doc = "Register `CTRLDSSEG` reader"]
pub type R = crate::R<CtrldssegSpec>;
#[doc = "Register `CTRLDSSEG` writer"]
pub type W = crate::W<CtrldssegSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "USBHC EHC Control Data Structure Segment register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrldsseg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrldsseg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrldssegSpec;
impl crate::RegisterSpec for CtrldssegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrldsseg::R`](R) reader structure"]
impl crate::Readable for CtrldssegSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrldsseg::W`](W) writer structure"]
impl crate::Writable for CtrldssegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLDSSEG to value 0"]
impl crate::Resettable for CtrldssegSpec {}
