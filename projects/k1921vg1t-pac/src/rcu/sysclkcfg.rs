#[doc = "Register `SYSCLKCFG` reader"]
pub type R = crate::R<SysclkcfgSpec>;
#[doc = "Register `SYSCLKCFG` writer"]
pub type W = crate::W<SysclkcfgSpec>;
#[doc = "System clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Src {
    #[doc = "0: HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1"]
    Refclk = 0,
    #[doc = "1: HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1"]
    Srcclk = 1,
    #[doc = "2: Clock from PLL OUT\\[0\\]"]
    Syspll0clk = 2,
    #[doc = "3: external CLK from gpio pin"]
    Extclk = 3,
}
impl From<Src> for u8 {
    #[inline(always)]
    fn from(variant: Src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Src {
    type Ux = u8;
}
impl crate::IsEnum for Src {}
#[doc = "Field `SRC` reader - System clock source"]
pub type SrcR = crate::FieldReader<Src>;
impl SrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Src {
        match self.bits {
            0 => Src::Refclk,
            1 => Src::Srcclk,
            2 => Src::Syspll0clk,
            3 => Src::Extclk,
            _ => unreachable!(),
        }
    }
    #[doc = "HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1"]
    #[inline(always)]
    pub fn is_refclk(&self) -> bool {
        *self == Src::Refclk
    }
    #[doc = "HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1"]
    #[inline(always)]
    pub fn is_srcclk(&self) -> bool {
        *self == Src::Srcclk
    }
    #[doc = "Clock from PLL OUT\\[0\\]"]
    #[inline(always)]
    pub fn is_syspll0clk(&self) -> bool {
        *self == Src::Syspll0clk
    }
    #[doc = "external CLK from gpio pin"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == Src::Extclk
    }
}
#[doc = "Field `SRC` writer - System clock source"]
pub type SrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Src, crate::Safe>;
impl<'a, REG> SrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1"]
    #[inline(always)]
    pub fn refclk(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Refclk)
    }
    #[doc = "HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1"]
    #[inline(always)]
    pub fn srcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Srcclk)
    }
    #[doc = "Clock from PLL OUT\\[0\\]"]
    #[inline(always)]
    pub fn syspll0clk(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Syspll0clk)
    }
    #[doc = "external CLK from gpio pin"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(Src::Extclk)
    }
}
#[doc = "Field `SECEN` reader - "]
pub type SecenR = crate::BitReader;
#[doc = "Field `SECEN` writer - "]
pub type SecenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - System clock source"]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn secen(&self) -> SecenR {
        SecenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock source"]
    #[inline(always)]
    pub fn src(&mut self) -> SrcW<'_, SysclkcfgSpec> {
        SrcW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn secen(&mut self) -> SecenW<'_, SysclkcfgSpec> {
        SecenW::new(self, 8)
    }
}
#[doc = "System clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysclkcfgSpec;
impl crate::RegisterSpec for SysclkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysclkcfg::R`](R) reader structure"]
impl crate::Readable for SysclkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sysclkcfg::W`](W) writer structure"]
impl crate::Writable for SysclkcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSCLKCFG to value 0"]
impl crate::Resettable for SysclkcfgSpec {}
