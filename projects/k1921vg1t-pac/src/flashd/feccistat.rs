#[doc = "Register `FECCISTAT` reader"]
pub type R = crate::R<FeccistatSpec>;
#[doc = "Register `FECCISTAT` writer"]
pub type W = crate::W<FeccistatSpec>;
#[doc = "Field `SEU` reader - Single error occurred"]
pub type SeuR = crate::BitReader;
#[doc = "Field `SEU` writer - Single error occurred"]
pub type SeuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEU` reader - Double error occurred"]
pub type DeuR = crate::BitReader;
#[doc = "Field `DEU` writer - Double error occurred"]
pub type DeuW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single error occurred"]
    #[inline(always)]
    pub fn seu(&self) -> SeuR {
        SeuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double error occurred"]
    #[inline(always)]
    pub fn deu(&self) -> DeuR {
        DeuR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single error occurred"]
    #[inline(always)]
    pub fn seu(&mut self) -> SeuW<'_, FeccistatSpec> {
        SeuW::new(self, 0)
    }
    #[doc = "Bit 1 - Double error occurred"]
    #[inline(always)]
    pub fn deu(&mut self) -> DeuW<'_, FeccistatSpec> {
        DeuW::new(self, 1)
    }
}
#[doc = "Flash ECC Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccistat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccistat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeccistatSpec;
impl crate::RegisterSpec for FeccistatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feccistat::R`](R) reader structure"]
impl crate::Readable for FeccistatSpec {}
#[doc = "`write(|w| ..)` method takes [`feccistat::W`](W) writer structure"]
impl crate::Writable for FeccistatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FECCISTAT to value 0"]
impl crate::Resettable for FeccistatSpec {}
