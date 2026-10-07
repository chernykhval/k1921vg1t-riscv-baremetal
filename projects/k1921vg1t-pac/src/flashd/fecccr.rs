#[doc = "Register `FECCCR` reader"]
pub type R = crate::R<FecccrSpec>;
#[doc = "Register `FECCCR` writer"]
pub type W = crate::W<FecccrSpec>;
#[doc = "Field `ECCEN` reader - Enable ECC detect and correction"]
pub type EccenR = crate::BitReader;
#[doc = "Field `ECCEN` writer - Enable ECC detect and correction"]
pub type EccenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISEUEN` reader - Enable interrupt when single error detected"]
pub type IseuenR = crate::BitReader;
#[doc = "Field `ISEUEN` writer - Enable interrupt when single error detected"]
pub type IseuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDEUEN` reader - Enable interrupt when double errors detected"]
pub type IdeuenR = crate::BitReader;
#[doc = "Field `IDEUEN` writer - Enable interrupt when double errors detected"]
pub type IdeuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRINJEN` reader - Enable error injection on write data bus"]
pub type ErrinjenR = crate::BitReader;
#[doc = "Field `ERRINJEN` writer - Enable error injection on write data bus"]
pub type ErrinjenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable ECC detect and correction"]
    #[inline(always)]
    pub fn eccen(&self) -> EccenR {
        EccenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Enable interrupt when single error detected"]
    #[inline(always)]
    pub fn iseuen(&self) -> IseuenR {
        IseuenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable interrupt when double errors detected"]
    #[inline(always)]
    pub fn ideuen(&self) -> IdeuenR {
        IdeuenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable error injection on write data bus"]
    #[inline(always)]
    pub fn errinjen(&self) -> ErrinjenR {
        ErrinjenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable ECC detect and correction"]
    #[inline(always)]
    pub fn eccen(&mut self) -> EccenW<'_, FecccrSpec> {
        EccenW::new(self, 0)
    }
    #[doc = "Bit 4 - Enable interrupt when single error detected"]
    #[inline(always)]
    pub fn iseuen(&mut self) -> IseuenW<'_, FecccrSpec> {
        IseuenW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable interrupt when double errors detected"]
    #[inline(always)]
    pub fn ideuen(&mut self) -> IdeuenW<'_, FecccrSpec> {
        IdeuenW::new(self, 5)
    }
    #[doc = "Bit 8 - Enable error injection on write data bus"]
    #[inline(always)]
    pub fn errinjen(&mut self) -> ErrinjenW<'_, FecccrSpec> {
        ErrinjenW::new(self, 8)
    }
}
#[doc = "Flash ECC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fecccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fecccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FecccrSpec;
impl crate::RegisterSpec for FecccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fecccr::R`](R) reader structure"]
impl crate::Readable for FecccrSpec {}
#[doc = "`write(|w| ..)` method takes [`fecccr::W`](W) writer structure"]
impl crate::Writable for FecccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FECCCR to value 0"]
impl crate::Resettable for FecccrSpec {}
