#[doc = "Register `INTSTAT` writer"]
pub type W = crate::W<IntstatSpec>;
#[doc = "Field `PLLLOCK` writer - "]
pub type PlllockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLFAIL` writer - "]
pub type PllfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGOOD1` writer - "]
pub type Clkgood1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGOOD2` writer - "]
pub type Clkgood2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGOOD3` writer - "]
pub type Clkgood3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKERR1` writer - "]
pub type Clkerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKERR2` writer - "]
pub type Clkerr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKERR3` writer - "]
pub type Clkerr3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn plllock(&mut self) -> PlllockW<'_, IntstatSpec> {
        PlllockW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pllfail(&mut self) -> PllfailW<'_, IntstatSpec> {
        PllfailW::new(self, 1)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkgood1(&mut self) -> Clkgood1W<'_, IntstatSpec> {
        Clkgood1W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkgood2(&mut self) -> Clkgood2W<'_, IntstatSpec> {
        Clkgood2W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn clkgood3(&mut self) -> Clkgood3W<'_, IntstatSpec> {
        Clkgood3W::new(self, 11)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn clkerr1(&mut self) -> Clkerr1W<'_, IntstatSpec> {
        Clkerr1W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn clkerr2(&mut self) -> Clkerr2W<'_, IntstatSpec> {
        Clkerr2W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn clkerr3(&mut self) -> Clkerr3W<'_, IntstatSpec> {
        Clkerr3W::new(self, 19)
    }
}
#[doc = "Interrupt status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intstat::W`](W) writer structure"]
impl crate::Writable for IntstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {}
