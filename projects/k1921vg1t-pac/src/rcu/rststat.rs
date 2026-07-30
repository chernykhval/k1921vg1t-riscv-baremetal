#[doc = "Register `RSTSTAT` reader"]
pub type R = crate::R<RststatSpec>;
#[doc = "Register `RSTSTAT` writer"]
pub type W = crate::W<RststatSpec>;
#[doc = "Field `POR` reader - PowerOn Reset status"]
pub type PorR = crate::BitReader;
#[doc = "Field `POR` writer - PowerOn Reset status"]
pub type PorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG` reader - WatchDog Reset status"]
pub type WdogR = crate::BitReader;
#[doc = "Field `WDOG` writer - WatchDog Reset status"]
pub type WdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSRST` reader - System Reset Status"]
pub type SysrstR = crate::BitReader;
#[doc = "Field `SYSRST` writer - System Reset Status"]
pub type SysrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOR` reader - Brown-out reset status"]
pub type BorR = crate::BitReader;
#[doc = "Field `BOR` writer - Brown-out reset status"]
pub type BorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTEP` reader - Reset due to cores mode changing"]
pub type LstepR = crate::BitReader;
#[doc = "Field `LSTEP` writer - Reset due to cores mode changing"]
pub type LstepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAULT` reader - Reset due fault timeout"]
pub type FaultR = crate::BitReader;
#[doc = "Field `FAULT` writer - Reset due fault timeout"]
pub type FaultW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UVD11` reader - Reset due to DVDD upper voltage detect (20% up with nominal values)"]
pub type Uvd11R = crate::BitReader;
#[doc = "Field `UVD11` writer - Reset due to DVDD upper voltage detect (20% up with nominal values)"]
pub type Uvd11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UVD33` reader - Reset due to AVDD upper voltage detect (20% up with nominal values)"]
pub type Uvd33R = crate::BitReader;
#[doc = "Field `UVD33` writer - Reset due to AVDD upper voltage detect (20% up with nominal values)"]
pub type Uvd33W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PowerOn Reset status"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WatchDog Reset status"]
    #[inline(always)]
    pub fn wdog(&self) -> WdogR {
        WdogR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System Reset Status"]
    #[inline(always)]
    pub fn sysrst(&self) -> SysrstR {
        SysrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Brown-out reset status"]
    #[inline(always)]
    pub fn bor(&self) -> BorR {
        BorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset due to cores mode changing"]
    #[inline(always)]
    pub fn lstep(&self) -> LstepR {
        LstepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset due fault timeout"]
    #[inline(always)]
    pub fn fault(&self) -> FaultR {
        FaultR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset due to DVDD upper voltage detect (20% up with nominal values)"]
    #[inline(always)]
    pub fn uvd11(&self) -> Uvd11R {
        Uvd11R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset due to AVDD upper voltage detect (20% up with nominal values)"]
    #[inline(always)]
    pub fn uvd33(&self) -> Uvd33R {
        Uvd33R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PowerOn Reset status"]
    #[inline(always)]
    pub fn por(&mut self) -> PorW<'_, RststatSpec> {
        PorW::new(self, 0)
    }
    #[doc = "Bit 1 - WatchDog Reset status"]
    #[inline(always)]
    pub fn wdog(&mut self) -> WdogW<'_, RststatSpec> {
        WdogW::new(self, 1)
    }
    #[doc = "Bit 2 - System Reset Status"]
    #[inline(always)]
    pub fn sysrst(&mut self) -> SysrstW<'_, RststatSpec> {
        SysrstW::new(self, 2)
    }
    #[doc = "Bit 3 - Brown-out reset status"]
    #[inline(always)]
    pub fn bor(&mut self) -> BorW<'_, RststatSpec> {
        BorW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset due to cores mode changing"]
    #[inline(always)]
    pub fn lstep(&mut self) -> LstepW<'_, RststatSpec> {
        LstepW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset due fault timeout"]
    #[inline(always)]
    pub fn fault(&mut self) -> FaultW<'_, RststatSpec> {
        FaultW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset due to DVDD upper voltage detect (20% up with nominal values)"]
    #[inline(always)]
    pub fn uvd11(&mut self) -> Uvd11W<'_, RststatSpec> {
        Uvd11W::new(self, 6)
    }
    #[doc = "Bit 7 - Reset due to AVDD upper voltage detect (20% up with nominal values)"]
    #[inline(always)]
    pub fn uvd33(&mut self) -> Uvd33W<'_, RststatSpec> {
        Uvd33W::new(self, 7)
    }
}
#[doc = "Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rststat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rststat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RststatSpec;
impl crate::RegisterSpec for RststatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rststat::R`](R) reader structure"]
impl crate::Readable for RststatSpec {}
#[doc = "`write(|w| ..)` method takes [`rststat::W`](W) writer structure"]
impl crate::Writable for RststatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTSTAT to value 0"]
impl crate::Resettable for RststatSpec {}
