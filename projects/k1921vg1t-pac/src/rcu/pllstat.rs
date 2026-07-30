#[doc = "Register `PLLSTAT` reader"]
pub type R = crate::R<PllstatSpec>;
#[doc = "Field `SYSPLLLOCK` reader - "]
pub type SysplllockR = crate::BitReader;
#[doc = "Field `PLL1LOCK` reader - "]
pub type Pll1lockR = crate::BitReader;
#[doc = "Field `PLL2LOCK` reader - "]
pub type Pll2lockR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sysplllock(&self) -> SysplllockR {
        SysplllockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pll1lock(&self) -> Pll1lockR {
        Pll1lockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pll2lock(&self) -> Pll2lockR {
        Pll2lockR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "PLLs status regisoter\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllstatSpec;
impl crate::RegisterSpec for PllstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllstat::R`](R) reader structure"]
impl crate::Readable for PllstatSpec {}
#[doc = "`reset()` method sets PLLSTAT to value 0"]
impl crate::Resettable for PllstatSpec {}
