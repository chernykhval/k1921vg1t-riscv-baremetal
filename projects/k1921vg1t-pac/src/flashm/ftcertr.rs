#[doc = "Register `FTCERTR` reader"]
pub type R = crate::R<FtcertrSpec>;
#[doc = "Register `FTCERTR` writer"]
pub type W = crate::W<FtcertrSpec>;
#[doc = "Field `TRCVSECT` reader - Sector erase recovery time value in cycles"]
pub type TrcvsectR = crate::FieldReader<u16>;
#[doc = "Field `TRCVSECT` writer - Sector erase recovery time value in cycles"]
pub type TrcvsectW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TRCVCHIP` reader - Chip erase recovery time value in cycles"]
pub type TrcvchipR = crate::FieldReader<u16>;
#[doc = "Field `TRCVCHIP` writer - Chip erase recovery time value in cycles"]
pub type TrcvchipW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sector erase recovery time value in cycles"]
    #[inline(always)]
    pub fn trcvsect(&self) -> TrcvsectR {
        TrcvsectR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Chip erase recovery time value in cycles"]
    #[inline(always)]
    pub fn trcvchip(&self) -> TrcvchipR {
        TrcvchipR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sector erase recovery time value in cycles"]
    #[inline(always)]
    pub fn trcvsect(&mut self) -> TrcvsectW<'_, FtcertrSpec> {
        TrcvsectW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Chip erase recovery time value in cycles"]
    #[inline(always)]
    pub fn trcvchip(&mut self) -> TrcvchipW<'_, FtcertrSpec> {
        TrcvchipW::new(self, 16)
    }
}
#[doc = "Flash Timing Control register (Erase Recovery time)\n\nYou can [`read`](crate::Reg::read) this register and get [`ftcertr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftcertr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtcertrSpec;
impl crate::RegisterSpec for FtcertrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftcertr::R`](R) reader structure"]
impl crate::Readable for FtcertrSpec {}
#[doc = "`write(|w| ..)` method takes [`ftcertr::W`](W) writer structure"]
impl crate::Writable for FtcertrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTCERTR to value 0x0320_00c8"]
impl crate::Resettable for FtcertrSpec {
    const RESET_VALUE: u32 = 0x0320_00c8;
}
