#[doc = "Register `CLKSTAT` reader"]
pub type R = crate::R<ClkstatSpec>;
#[doc = "Field `SRC` reader - REFCLK:0:HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1;SRCCLK:1:HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1;SYSPLL0CLK:2: Clock from PLL OUT\\[0\\];EXTCLK:3: external CLK from gpio pin;"]
pub type SrcR = crate::FieldReader;
#[doc = "Field `CLKGOOD0` reader - "]
pub type Clkgood0R = crate::BitReader;
#[doc = "Field `CLKGOOD1` reader - "]
pub type Clkgood1R = crate::BitReader;
#[doc = "Field `CLKGOOD2` reader - "]
pub type Clkgood2R = crate::BitReader;
#[doc = "Field `CLKGOOD3` reader - "]
pub type Clkgood3R = crate::BitReader;
#[doc = "Field `CLKERR0` reader - "]
pub type Clkerr0R = crate::BitReader;
#[doc = "Field `CLKERR1` reader - "]
pub type Clkerr1R = crate::BitReader;
#[doc = "Field `CLKERR2` reader - "]
pub type Clkerr2R = crate::BitReader;
#[doc = "Field `CLKERR3` reader - "]
pub type Clkerr3R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - REFCLK:0:HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1;SRCCLK:1:HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1;SYSPLL0CLK:2: Clock from PLL OUT\\[0\\];EXTCLK:3: external CLK from gpio pin;"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkgood0(&self) -> Clkgood0R {
        Clkgood0R::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn clkerr0(&self) -> Clkerr0R {
        Clkerr0R::new(((self.bits >> 16) & 1) != 0)
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
#[doc = "System clock status register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkstatSpec;
impl crate::RegisterSpec for ClkstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkstat::R`](R) reader structure"]
impl crate::Readable for ClkstatSpec {}
#[doc = "`reset()` method sets CLKSTAT to value 0"]
impl crate::Resettable for ClkstatSpec {}
