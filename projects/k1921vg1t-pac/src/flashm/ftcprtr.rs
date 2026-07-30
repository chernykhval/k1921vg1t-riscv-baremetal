#[doc = "Register `FTCPRTR` reader"]
pub type R = crate::R<FtcprtrSpec>;
#[doc = "Register `FTCPRTR` writer"]
pub type W = crate::W<FtcprtrSpec>;
#[doc = "Field `TRCVPROG` reader - Program recovery time value in cycles"]
pub type TrcvprogR = crate::FieldReader;
#[doc = "Field `TRCVPROG` writer - Program recovery time value in cycles"]
pub type TrcvprogW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Program recovery time value in cycles"]
    #[inline(always)]
    pub fn trcvprog(&self) -> TrcvprogR {
        TrcvprogR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Program recovery time value in cycles"]
    #[inline(always)]
    pub fn trcvprog(&mut self) -> TrcvprogW<'_, FtcprtrSpec> {
        TrcvprogW::new(self, 0)
    }
}
#[doc = "Flash Timing Control register (Program Recovery time)\n\nYou can [`read`](crate::Reg::read) this register and get [`ftcprtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftcprtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtcprtrSpec;
impl crate::RegisterSpec for FtcprtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftcprtr::R`](R) reader structure"]
impl crate::Readable for FtcprtrSpec {}
#[doc = "`write(|w| ..)` method takes [`ftcprtr::W`](W) writer structure"]
impl crate::Writable for FtcprtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTCPRTR to value 0x14"]
impl crate::Resettable for FtcprtrSpec {
    const RESET_VALUE: u32 = 0x14;
}
