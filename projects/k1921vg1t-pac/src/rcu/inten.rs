#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `PLLLOCK` reader - Enable PLL lock interrupt"]
pub type PlllockR = crate::BitReader;
#[doc = "Field `PLLLOCK` writer - Enable PLL lock interrupt"]
pub type PlllockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLFAIL` reader - Enable PLL fail to lock interrupt"]
pub type PllfailR = crate::BitReader;
#[doc = "Field `PLLFAIL` writer - Enable PLL fail to lock interrupt"]
pub type PllfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGOOD1` reader - "]
pub type Clkgood1R = crate::BitReader;
#[doc = "Field `CLKGOOD1` writer - "]
pub type Clkgood1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGOOD2` reader - "]
pub type Clkgood2R = crate::BitReader;
#[doc = "Field `CLKGOOD2` writer - "]
pub type Clkgood2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGOOD3` reader - "]
pub type Clkgood3R = crate::BitReader;
#[doc = "Field `CLKGOOD3` writer - "]
pub type Clkgood3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKERR1` reader - "]
pub type Clkerr1R = crate::BitReader;
#[doc = "Field `CLKERR1` writer - "]
pub type Clkerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKERR2` reader - "]
pub type Clkerr2R = crate::BitReader;
#[doc = "Field `CLKERR2` writer - "]
pub type Clkerr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKERR3` reader - "]
pub type Clkerr3R = crate::BitReader;
#[doc = "Field `CLKERR3` writer - "]
pub type Clkerr3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable PLL lock interrupt"]
    #[inline(always)]
    pub fn plllock(&self) -> PlllockR {
        PlllockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable PLL fail to lock interrupt"]
    #[inline(always)]
    pub fn pllfail(&self) -> PllfailR {
        PllfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkgood1(&self) -> Clkgood1R {
        Clkgood1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkgood2(&self) -> Clkgood2R {
        Clkgood2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clkgood3(&self) -> Clkgood3R {
        Clkgood3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clkerr1(&self) -> Clkerr1R {
        Clkerr1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clkerr2(&self) -> Clkerr2R {
        Clkerr2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clkerr3(&self) -> Clkerr3R {
        Clkerr3R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PLL lock interrupt"]
    #[inline(always)]
    pub fn plllock(&mut self) -> PlllockW<'_, IntenSpec> {
        PlllockW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable PLL fail to lock interrupt"]
    #[inline(always)]
    pub fn pllfail(&mut self) -> PllfailW<'_, IntenSpec> {
        PllfailW::new(self, 1)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkgood1(&mut self) -> Clkgood1W<'_, IntenSpec> {
        Clkgood1W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkgood2(&mut self) -> Clkgood2W<'_, IntenSpec> {
        Clkgood2W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clkgood3(&mut self) -> Clkgood3W<'_, IntenSpec> {
        Clkgood3W::new(self, 11)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clkerr1(&mut self) -> Clkerr1W<'_, IntenSpec> {
        Clkerr1W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clkerr2(&mut self) -> Clkerr2W<'_, IntenSpec> {
        Clkerr2W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clkerr3(&mut self) -> Clkerr3W<'_, IntenSpec> {
        Clkerr3W::new(self, 19)
    }
}
#[doc = "Interrupt enable reigster\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
