#[doc = "Register `FTCETR` reader"]
pub type R = crate::R<FtcetrSpec>;
#[doc = "Register `FTCETR` writer"]
pub type W = crate::W<FtcetrSpec>;
#[doc = "Field `TSCE` reader - Erase time value in cycles"]
pub type TsceR = crate::FieldReader<u32>;
#[doc = "Field `TSCE` writer - Erase time value in cycles"]
pub type TsceW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Erase time value in cycles"]
    #[inline(always)]
    pub fn tsce(&self) -> TsceR {
        TsceR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Erase time value in cycles"]
    #[inline(always)]
    pub fn tsce(&mut self) -> TsceW<'_, FtcetrSpec> {
        TsceW::new(self, 0)
    }
}
#[doc = "Flash Timing Control register (Erase time)\n\nYou can [`read`](crate::Reg::read) this register and get [`ftcetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftcetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtcetrSpec;
impl crate::RegisterSpec for FtcetrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftcetr::R`](R) reader structure"]
impl crate::Readable for FtcetrSpec {}
#[doc = "`write(|w| ..)` method takes [`ftcetr::W`](W) writer structure"]
impl crate::Writable for FtcetrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTCETR to value 0xea60"]
impl crate::Resettable for FtcetrSpec {
    const RESET_VALUE: u32 = 0xea60;
}
