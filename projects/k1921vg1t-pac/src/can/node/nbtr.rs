#[doc = "Register `NBTR` reader"]
pub type R = crate::R<NbtrSpec>;
#[doc = "Register `NBTR` writer"]
pub type W = crate::W<NbtrSpec>;
#[doc = "Field `BRP` reader - Prescaler rate"]
pub type BrpR = crate::FieldReader;
#[doc = "Field `BRP` writer - Prescaler rate"]
pub type BrpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SJW` reader - Transition width resynchronization"]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - Transition width resynchronization"]
pub type SjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSEG1` reader - Parameter 1"]
pub type Tseg1R = crate::FieldReader;
#[doc = "Field `TSEG1` writer - Parameter 1"]
pub type Tseg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TSEG2` reader - Parameter 2"]
pub type Tseg2R = crate::FieldReader;
#[doc = "Field `TSEG2` writer - Parameter 2"]
pub type Tseg2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIV8` reader - Frequency divider by 8"]
pub type Div8R = crate::BitReader;
#[doc = "Field `DIV8` writer - Frequency divider by 8"]
pub type Div8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Prescaler rate"]
    #[inline(always)]
    pub fn brp(&self) -> BrpR {
        BrpR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Transition width resynchronization"]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Parameter 1"]
    #[inline(always)]
    pub fn tseg1(&self) -> Tseg1R {
        Tseg1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Parameter 2"]
    #[inline(always)]
    pub fn tseg2(&self) -> Tseg2R {
        Tseg2R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Frequency divider by 8"]
    #[inline(always)]
    pub fn div8(&self) -> Div8R {
        Div8R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Prescaler rate"]
    #[inline(always)]
    pub fn brp(&mut self) -> BrpW<'_, NbtrSpec> {
        BrpW::new(self, 0)
    }
    #[doc = "Bits 6:7 - Transition width resynchronization"]
    #[inline(always)]
    pub fn sjw(&mut self) -> SjwW<'_, NbtrSpec> {
        SjwW::new(self, 6)
    }
    #[doc = "Bits 8:11 - Parameter 1"]
    #[inline(always)]
    pub fn tseg1(&mut self) -> Tseg1W<'_, NbtrSpec> {
        Tseg1W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Parameter 2"]
    #[inline(always)]
    pub fn tseg2(&mut self) -> Tseg2W<'_, NbtrSpec> {
        Tseg2W::new(self, 12)
    }
    #[doc = "Bit 15 - Frequency divider by 8"]
    #[inline(always)]
    pub fn div8(&mut self) -> Div8W<'_, NbtrSpec> {
        Div8W::new(self, 15)
    }
}
#[doc = "Timing register bits 0\n\nYou can [`read`](crate::Reg::read) this register and get [`nbtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NbtrSpec;
impl crate::RegisterSpec for NbtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nbtr::R`](R) reader structure"]
impl crate::Readable for NbtrSpec {}
#[doc = "`write(|w| ..)` method takes [`nbtr::W`](W) writer structure"]
impl crate::Writable for NbtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NBTR to value 0"]
impl crate::Resettable for NbtrSpec {}
