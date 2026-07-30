#[doc = "Register `CLKOUTCFG` reader"]
pub type R = crate::R<ClkoutcfgSpec>;
#[doc = "Register `CLKOUTCFG` writer"]
pub type W = crate::W<ClkoutcfgSpec>;
#[doc = "Field `CLKEN` reader - Clock enable"]
pub type ClkenR = crate::BitReader;
#[doc = "Field `CLKEN` writer - Clock enable"]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "3: 32kHz oscilator"]
    Lsi = 3,
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
            3 => Clksel::Lsi,
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
    #[doc = "32kHz oscilator"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == Clksel::Lsi
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
    #[doc = "32kHz oscilator"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Lsi)
    }
}
#[doc = "Field `DIVEN` reader - Enable divider"]
pub type DivenR = crate::BitReader;
#[doc = "Field `DIVEN` writer - Enable divider"]
pub type DivenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVN` reader - Divider coefficient"]
pub type DivnR = crate::FieldReader<u16>;
#[doc = "Field `DIVN` writer - Divider coefficient"]
pub type DivnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Clock enable"]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
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
    #[doc = "Bits 16:31 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&self) -> DivnR {
        DivnR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Clock enable"]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, ClkoutcfgSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Clock source select"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, ClkoutcfgSpec> {
        ClkselW::new(self, 8)
    }
    #[doc = "Bit 12 - Enable divider"]
    #[inline(always)]
    pub fn diven(&mut self) -> DivenW<'_, ClkoutcfgSpec> {
        DivenW::new(self, 12)
    }
    #[doc = "Bits 16:31 - Divider coefficient"]
    #[inline(always)]
    pub fn divn(&mut self) -> DivnW<'_, ClkoutcfgSpec> {
        DivnW::new(self, 16)
    }
}
#[doc = "Clockout Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkoutcfgSpec;
impl crate::RegisterSpec for ClkoutcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkoutcfg::R`](R) reader structure"]
impl crate::Readable for ClkoutcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkoutcfg::W`](W) writer structure"]
impl crate::Writable for ClkoutcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKOUTCFG to value 0"]
impl crate::Resettable for ClkoutcfgSpec {}
