#[doc = "Register `WDTCFG` reader"]
pub type R = crate::R<WdtcfgSpec>;
#[doc = "Register `WDTCFG` writer"]
pub type W = crate::W<WdtcfgSpec>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTDIS` reader - Reset disable"]
pub type RstdisR = crate::BitReader;
#[doc = "Field `RSTDIS` writer - Reset disable"]
pub type RstdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clksel {
    #[doc = "0: HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1"]
    Refclk = 0,
    #[doc = "1: HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1"]
    Srcclk = 1,
    #[doc = "2: Clock from PLL OUT\\[0\\]"]
    Syspll0clk = 2,
    #[doc = "3: external CLK from gpio pin"]
    Extclk = 3,
}
impl From<Clksel> for u8 {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clksel {
    type Ux = u8;
}
impl crate::IsEnum for Clksel {}
#[doc = "Field `CLKSEL` reader - Clock source select"]
pub type ClkselR = crate::FieldReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            0 => Clksel::Refclk,
            1 => Clksel::Srcclk,
            2 => Clksel::Syspll0clk,
            3 => Clksel::Extclk,
            _ => unreachable!(),
        }
    }
    #[doc = "HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1"]
    #[inline(always)]
    pub fn is_refclk(&self) -> bool {
        *self == Clksel::Refclk
    }
    #[doc = "HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1"]
    #[inline(always)]
    pub fn is_srcclk(&self) -> bool {
        *self == Clksel::Srcclk
    }
    #[doc = "Clock from PLL OUT\\[0\\]"]
    #[inline(always)]
    pub fn is_syspll0clk(&self) -> bool {
        *self == Clksel::Syspll0clk
    }
    #[doc = "external CLK from gpio pin"]
    #[inline(always)]
    pub fn is_extclk(&self) -> bool {
        *self == Clksel::Extclk
    }
}
#[doc = "Field `CLKSEL` writer - Clock source select"]
pub type ClkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clksel, crate::Safe>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "HSI 4MHz if CPE=0 or HSE(XTALL) if CPE=1"]
    #[inline(always)]
    pub fn refclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Refclk)
    }
    #[doc = "HSE(XTALL) if CPE=0 or HSI 4MHz if CPE=1"]
    #[inline(always)]
    pub fn srcclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Srcclk)
    }
    #[doc = "Clock from PLL OUT\\[0\\]"]
    #[inline(always)]
    pub fn syspll0clk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Syspll0clk)
    }
    #[doc = "external CLK from gpio pin"]
    #[inline(always)]
    pub fn extclk(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Extclk)
    }
}
#[doc = "Field `DIVEN` reader - Enable divider"]
pub type DivenR = crate::BitReader;
#[doc = "Field `DIVEN` writer - Enable divider"]
pub type DivenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVN` reader - Divider coefficient"]
pub type DivnR = crate::FieldReader;
#[doc = "Field `DIVN` writer - Divider coefficient"]
pub type DivnW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Reset disable"]
    #[inline(always)]
    pub fn rstdis(&self) -> RstdisR {
        RstdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock source select"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Enable divider"]
    #[inline(always)]
    pub fn diven(&self) -> DivenR {
        DivenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&self) -> DivnR {
        DivnR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, WdtcfgSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bit 4 - Reset disable"]
    #[inline(always)]
    pub fn rstdis(&mut self) -> RstdisW<'_, WdtcfgSpec> {
        RstdisW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Clock source select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, WdtcfgSpec> {
        ClkselW::new(self, 8)
    }
    #[doc = "Bit 12 - Enable divider"]
    #[inline(always)]
    pub fn diven(&mut self) -> DivenW<'_, WdtcfgSpec> {
        DivenW::new(self, 12)
    }
    #[doc = "Bits 16:21 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&mut self) -> DivnW<'_, WdtcfgSpec> {
        DivnW::new(self, 16)
    }
}
#[doc = "WatchDog Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdtcfgSpec;
impl crate::RegisterSpec for WdtcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdtcfg::R`](R) reader structure"]
impl crate::Readable for WdtcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`wdtcfg::W`](W) writer structure"]
impl crate::Writable for WdtcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDTCFG to value 0x003f_1000"]
impl crate::Resettable for WdtcfgSpec {
    const RESET_VALUE: u32 = 0x003f_1000;
}
