#[doc = "Register `DIV` reader"]
pub type R = crate::R<DivSpec>;
#[doc = "Register `DIV` writer"]
pub type W = crate::W<DivSpec>;
#[doc = "Field `DIV1A` reader - Divider 1A"]
pub type Div1aR = crate::FieldReader;
#[doc = "Field `DIV1A` writer - Divider 1A"]
pub type Div1aW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIV1B` reader - Divider 1B"]
pub type Div1bR = crate::FieldReader;
#[doc = "Field `DIV1B` writer - Divider 1B"]
pub type Div1bW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PREDIV` reader - Predivisor coefficient"]
pub type PredivR = crate::FieldReader;
#[doc = "Field `PREDIV` writer - Predivisor coefficient"]
pub type PredivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NNCLR` reader - N-divider enable"]
pub type NnclrR = crate::BitReader;
#[doc = "Field `NNCLR` writer - N-divider enable"]
pub type NnclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNCLR` reader - R-divider enable"]
pub type RnclrR = crate::BitReader;
#[doc = "Field `RNCLR` writer - R-divider enable"]
pub type RnclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIV` reader - R-divider coefficient"]
pub type RdivR = crate::FieldReader;
#[doc = "Field `RDIV` writer - R-divider coefficient"]
pub type RdivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NDIV` reader - Integer part of divider"]
pub type NdivR = crate::FieldReader;
#[doc = "Field `NDIV` writer - Integer part of divider"]
pub type NdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Divider 1A"]
    #[inline(always)]
    pub fn div1a(&self) -> Div1aR {
        Div1aR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:9 - Divider 1B"]
    #[inline(always)]
    pub fn div1b(&self) -> Div1bR {
        Div1bR::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 12:13 - Predivisor coefficient"]
    #[inline(always)]
    pub fn prediv(&self) -> PredivR {
        PredivR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - N-divider enable"]
    #[inline(always)]
    pub fn nnclr(&self) -> NnclrR {
        NnclrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - R-divider enable"]
    #[inline(always)]
    pub fn rnclr(&self) -> RnclrR {
        RnclrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - R-divider coefficient"]
    #[inline(always)]
    pub fn rdiv(&self) -> RdivR {
        RdivR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - Integer part of divider"]
    #[inline(always)]
    pub fn ndiv(&self) -> NdivR {
        NdivR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Divider 1A"]
    #[inline(always)]
    pub fn div1a(&mut self) -> Div1aW<'_, DivSpec> {
        Div1aW::new(self, 0)
    }
    #[doc = "Bits 4:9 - Divider 1B"]
    #[inline(always)]
    pub fn div1b(&mut self) -> Div1bW<'_, DivSpec> {
        Div1bW::new(self, 4)
    }
    #[doc = "Bits 12:13 - Predivisor coefficient"]
    #[inline(always)]
    pub fn prediv(&mut self) -> PredivW<'_, DivSpec> {
        PredivW::new(self, 12)
    }
    #[doc = "Bit 14 - N-divider enable"]
    #[inline(always)]
    pub fn nnclr(&mut self) -> NnclrW<'_, DivSpec> {
        NnclrW::new(self, 14)
    }
    #[doc = "Bit 15 - R-divider enable"]
    #[inline(always)]
    pub fn rnclr(&mut self) -> RnclrW<'_, DivSpec> {
        RnclrW::new(self, 15)
    }
    #[doc = "Bits 16:21 - R-divider coefficient"]
    #[inline(always)]
    pub fn rdiv(&mut self) -> RdivW<'_, DivSpec> {
        RdivW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Integer part of divider"]
    #[inline(always)]
    pub fn ndiv(&mut self) -> NdivW<'_, DivSpec> {
        NdivW::new(self, 24)
    }
}
#[doc = "PLL DIV Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivSpec;
impl crate::RegisterSpec for DivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`div::R`](R) reader structure"]
impl crate::Readable for DivSpec {}
#[doc = "`write(|w| ..)` method takes [`div::W`](W) writer structure"]
impl crate::Writable for DivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DivSpec {}
