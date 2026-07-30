#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `PD` reader - Powerdown enable"]
pub type PdR = crate::BitReader;
#[doc = "Field `PD` writer - Powerdown enable"]
pub type PdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS` reader - Bypass enable"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - Bypass enable"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOUTEN` reader - Fout enable"]
pub type FoutenR = crate::BitReader;
#[doc = "Field `FOUTEN` writer - Fout enable"]
pub type FoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Select reference clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksel {
    #[doc = "0: HSE oscillator(src_clk)"]
    Hse = 0,
    #[doc = "1: external clock from gpio"]
    Ext = 1,
}
impl From<Clksel> for bool {
    #[inline(always)]
    fn from(variant: Clksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSEL` reader - Select reference clock"]
pub type ClkselR = crate::BitReader<Clksel>;
impl ClkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksel {
        match self.bits {
            false => Clksel::Hse,
            true => Clksel::Ext,
        }
    }
    #[doc = "HSE oscillator(src_clk)"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == Clksel::Hse
    }
    #[doc = "external clock from gpio"]
    #[inline(always)]
    pub fn is_ext(&self) -> bool {
        *self == Clksel::Ext
    }
}
#[doc = "Field `CLKSEL` writer - Select reference clock"]
pub type ClkselW<'a, REG> = crate::BitWriter<'a, REG, Clksel>;
impl<'a, REG> ClkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HSE oscillator(src_clk)"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Hse)
    }
    #[doc = "external clock from gpio"]
    #[inline(always)]
    pub fn ext(self) -> &'a mut crate::W<REG> {
        self.variant(Clksel::Ext)
    }
}
#[doc = "Field `SP` reader - Song-Park mode"]
pub type SpR = crate::BitReader;
#[doc = "Field `SP` writer - Song-Park mode"]
pub type SpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ST` reader - Sigma-delta modulator mode"]
pub type StR = crate::FieldReader;
#[doc = "Field `ST` writer - Sigma-delta modulator mode"]
pub type StW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CP` reader - Output current of current switch"]
pub type CpR = crate::FieldReader;
#[doc = "Field `CP` writer - Output current of current switch"]
pub type CpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PFD` reader - Mode of current switch"]
pub type PfdR = crate::FieldReader;
#[doc = "Field `PFD` writer - Mode of current switch"]
pub type PfdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPF` reader - Loop filter band"]
pub type LpfR = crate::FieldReader;
#[doc = "Field `LPF` writer - Loop filter band"]
pub type LpfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VCOMODE` reader - Subband selection signal"]
pub type VcomodeR = crate::FieldReader;
#[doc = "Field `VCOMODE` writer - Subband selection signal"]
pub type VcomodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VCOMA` reader - Ajust amplitude of GUN"]
pub type VcomaR = crate::BitReader;
#[doc = "Field `VCOMA` writer - Ajust amplitude of GUN"]
pub type VcomaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCOMF` reader - Ajust lower range of GUN"]
pub type VcomfR = crate::BitReader;
#[doc = "Field `VCOMF` writer - Ajust lower range of GUN"]
pub type VcomfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLDMODE` reader - Frequency detector delay"]
pub type DldmodeR = crate::BitReader;
#[doc = "Field `DLDMODE` writer - Frequency detector delay"]
pub type DldmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DT` reader - Fractional noise randomization mode"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - Fractional noise randomization mode"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MXEN` reader - Test mux enable"]
pub type MxenR = crate::BitReader;
#[doc = "Field `MXEN` writer - Test mux enable"]
pub type MxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MX` reader - Test mux signal select"]
pub type MxR = crate::FieldReader;
#[doc = "Field `MX` writer - Test mux signal select"]
pub type MxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Powerdown enable"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bypass enable"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fout enable"]
    #[inline(always)]
    pub fn fouten(&self) -> FoutenR {
        FoutenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select reference clock"]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Song-Park mode"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Sigma-delta modulator mode"]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Output current of current switch"]
    #[inline(always)]
    pub fn cp(&self) -> CpR {
        CpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - Mode of current switch"]
    #[inline(always)]
    pub fn pfd(&self) -> PfdR {
        PfdR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Loop filter band"]
    #[inline(always)]
    pub fn lpf(&self) -> LpfR {
        LpfR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Subband selection signal"]
    #[inline(always)]
    pub fn vcomode(&self) -> VcomodeR {
        VcomodeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Ajust amplitude of GUN"]
    #[inline(always)]
    pub fn vcoma(&self) -> VcomaR {
        VcomaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Ajust lower range of GUN"]
    #[inline(always)]
    pub fn vcomf(&self) -> VcomfR {
        VcomfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Frequency detector delay"]
    #[inline(always)]
    pub fn dldmode(&self) -> DldmodeR {
        DldmodeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Fractional noise randomization mode"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Test mux enable"]
    #[inline(always)]
    pub fn mxen(&self) -> MxenR {
        MxenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Test mux signal select"]
    #[inline(always)]
    pub fn mx(&self) -> MxR {
        MxR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Powerdown enable"]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<'_, CfgSpec> {
        PdW::new(self, 0)
    }
    #[doc = "Bit 1 - Bypass enable"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<'_, CfgSpec> {
        BypassW::new(self, 1)
    }
    #[doc = "Bit 2 - Fout enable"]
    #[inline(always)]
    pub fn fouten(&mut self) -> FoutenW<'_, CfgSpec> {
        FoutenW::new(self, 2)
    }
    #[doc = "Bit 3 - Select reference clock"]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, CfgSpec> {
        ClkselW::new(self, 3)
    }
    #[doc = "Bit 4 - Song-Park mode"]
    #[inline(always)]
    pub fn sp(&mut self) -> SpW<'_, CfgSpec> {
        SpW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Sigma-delta modulator mode"]
    #[inline(always)]
    pub fn st(&mut self) -> StW<'_, CfgSpec> {
        StW::new(self, 5)
    }
    #[doc = "Bits 8:11 - Output current of current switch"]
    #[inline(always)]
    pub fn cp(&mut self) -> CpW<'_, CfgSpec> {
        CpW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Mode of current switch"]
    #[inline(always)]
    pub fn pfd(&mut self) -> PfdW<'_, CfgSpec> {
        PfdW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Loop filter band"]
    #[inline(always)]
    pub fn lpf(&mut self) -> LpfW<'_, CfgSpec> {
        LpfW::new(self, 14)
    }
    #[doc = "Bits 16:19 - Subband selection signal"]
    #[inline(always)]
    pub fn vcomode(&mut self) -> VcomodeW<'_, CfgSpec> {
        VcomodeW::new(self, 16)
    }
    #[doc = "Bit 20 - Ajust amplitude of GUN"]
    #[inline(always)]
    pub fn vcoma(&mut self) -> VcomaW<'_, CfgSpec> {
        VcomaW::new(self, 20)
    }
    #[doc = "Bit 21 - Ajust lower range of GUN"]
    #[inline(always)]
    pub fn vcomf(&mut self) -> VcomfW<'_, CfgSpec> {
        VcomfW::new(self, 21)
    }
    #[doc = "Bit 22 - Frequency detector delay"]
    #[inline(always)]
    pub fn dldmode(&mut self) -> DldmodeW<'_, CfgSpec> {
        DldmodeW::new(self, 22)
    }
    #[doc = "Bits 24:25 - Fractional noise randomization mode"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<'_, CfgSpec> {
        DtW::new(self, 24)
    }
    #[doc = "Bit 28 - Test mux enable"]
    #[inline(always)]
    pub fn mxen(&mut self) -> MxenW<'_, CfgSpec> {
        MxenW::new(self, 28)
    }
    #[doc = "Bits 29:31 - Test mux signal select"]
    #[inline(always)]
    pub fn mx(&mut self) -> MxW<'_, CfgSpec> {
        MxW::new(self, 29)
    }
}
#[doc = "PLL Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0x9010_f001"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x9010_f001;
}
