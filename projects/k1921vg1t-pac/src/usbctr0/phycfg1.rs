#[doc = "Register `PHYCFG1` reader"]
pub type R = crate::R<Phycfg1Spec>;
#[doc = "Register `PHYCFG1` writer"]
pub type W = crate::W<Phycfg1Spec>;
#[doc = "Field `ROUT_CODE` reader - Transmitter output resistor tune code"]
pub type RoutCodeR = crate::FieldReader;
#[doc = "Field `ROUT_CODE` writer - Transmitter output resistor tune code"]
pub type RoutCodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ROUT_TUNE_EN` reader - Automatic transmitter output resistor calibration"]
pub type RoutTuneEnR = crate::BitReader;
#[doc = "Field `ROUT_TUNE_EN` writer - Automatic transmitter output resistor calibration"]
pub type RoutTuneEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSET_CODE` reader - Setting resistor tune code"]
pub type RsetCodeR = crate::FieldReader;
#[doc = "Field `RSET_CODE` writer - Setting resistor tune code"]
pub type RsetCodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RSET_TUNE_EN` reader - Automatic calibration of resistor in current source"]
pub type RsetTuneEnR = crate::BitReader;
#[doc = "Field `RSET_TUNE_EN` writer - Automatic calibration of resistor in current source"]
pub type RsetTuneEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Capture point in bit interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DllPhase {
    #[doc = "0: 80% of bit interval"]
    Percent80 = 0,
    #[doc = "1: 40% of bit interval"]
    Percent40 = 1,
    #[doc = "2: 60% of bit interval"]
    Percent60 = 2,
    #[doc = "3: 100% of bit interval"]
    Percent100 = 3,
}
impl From<DllPhase> for u8 {
    #[inline(always)]
    fn from(variant: DllPhase) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DllPhase {
    type Ux = u8;
}
impl crate::IsEnum for DllPhase {}
#[doc = "Field `DLL_PHASE` reader - Capture point in bit interval"]
pub type DllPhaseR = crate::FieldReader<DllPhase>;
impl DllPhaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DllPhase {
        match self.bits {
            0 => DllPhase::Percent80,
            1 => DllPhase::Percent40,
            2 => DllPhase::Percent60,
            3 => DllPhase::Percent100,
            _ => unreachable!(),
        }
    }
    #[doc = "80% of bit interval"]
    #[inline(always)]
    pub fn is_percent_80(&self) -> bool {
        *self == DllPhase::Percent80
    }
    #[doc = "40% of bit interval"]
    #[inline(always)]
    pub fn is_percent_40(&self) -> bool {
        *self == DllPhase::Percent40
    }
    #[doc = "60% of bit interval"]
    #[inline(always)]
    pub fn is_percent_60(&self) -> bool {
        *self == DllPhase::Percent60
    }
    #[doc = "100% of bit interval"]
    #[inline(always)]
    pub fn is_percent_100(&self) -> bool {
        *self == DllPhase::Percent100
    }
}
#[doc = "Field `DLL_PHASE` writer - Capture point in bit interval"]
pub type DllPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 2, DllPhase, crate::Safe>;
impl<'a, REG> DllPhaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "80% of bit interval"]
    #[inline(always)]
    pub fn percent_80(self) -> &'a mut crate::W<REG> {
        self.variant(DllPhase::Percent80)
    }
    #[doc = "40% of bit interval"]
    #[inline(always)]
    pub fn percent_40(self) -> &'a mut crate::W<REG> {
        self.variant(DllPhase::Percent40)
    }
    #[doc = "60% of bit interval"]
    #[inline(always)]
    pub fn percent_60(self) -> &'a mut crate::W<REG> {
        self.variant(DllPhase::Percent60)
    }
    #[doc = "100% of bit interval"]
    #[inline(always)]
    pub fn percent_100(self) -> &'a mut crate::W<REG> {
        self.variant(DllPhase::Percent100)
    }
}
#[doc = "Field `FORCE_RX` reader - Debug signal: do not shut off receiver when the packet has come"]
pub type ForceRxR = crate::BitReader;
#[doc = "Field `FORCE_RX` writer - Debug signal: do not shut off receiver when the packet has come"]
pub type ForceRxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CREFSEL` reader - Select type of bsae current"]
pub type CrefselR = crate::BitReader;
#[doc = "Field `CREFSEL` writer - Select type of bsae current"]
pub type CrefselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_MODE` reader - Transmitter mode"]
pub type TxModeR = crate::BitReader;
#[doc = "Field `TX_MODE` writer - Transmitter mode"]
pub type TxModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RPU compensation mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rpumode {
    #[doc = "0: Default value"]
    Default = 0,
    #[doc = "2: TX_RPU_COMP_EN always 0"]
    Low = 2,
    #[doc = "3: TX_RPU_COMP_EN always 1"]
    High = 3,
}
impl From<Rpumode> for u8 {
    #[inline(always)]
    fn from(variant: Rpumode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rpumode {
    type Ux = u8;
}
impl crate::IsEnum for Rpumode {}
#[doc = "Field `RPUMODE` reader - RPU compensation mode"]
pub type RpumodeR = crate::FieldReader<Rpumode>;
impl RpumodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpumode {
        match self.bits {
            0 => Rpumode::Default,
            2 => Rpumode::Low,
            3 => Rpumode::High,
            _ => unreachable!(),
        }
    }
    #[doc = "Default value"]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Rpumode::Default
    }
    #[doc = "TX_RPU_COMP_EN always 0"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Rpumode::Low
    }
    #[doc = "TX_RPU_COMP_EN always 1"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Rpumode::High
    }
}
#[doc = "Field `RPUMODE` writer - RPU compensation mode"]
pub type RpumodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rpumode>;
impl<'a, REG> RpumodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default value"]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Rpumode::Default)
    }
    #[doc = "TX_RPU_COMP_EN always 0"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Rpumode::Low)
    }
    #[doc = "TX_RPU_COMP_EN always 1"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Rpumode::High)
    }
}
#[doc = "Field `TUNE_EN` reader - Tune enable"]
pub type TuneEnR = crate::BitReader;
#[doc = "Field `TUNE_EN` writer - Tune enable"]
pub type TuneEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Transmitter output resistor tune code"]
    #[inline(always)]
    pub fn rout_code(&self) -> RoutCodeR {
        RoutCodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Automatic transmitter output resistor calibration"]
    #[inline(always)]
    pub fn rout_tune_en(&self) -> RoutTuneEnR {
        RoutTuneEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - Setting resistor tune code"]
    #[inline(always)]
    pub fn rset_code(&self) -> RsetCodeR {
        RsetCodeR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Automatic calibration of resistor in current source"]
    #[inline(always)]
    pub fn rset_tune_en(&self) -> RsetTuneEnR {
        RsetTuneEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Capture point in bit interval"]
    #[inline(always)]
    pub fn dll_phase(&self) -> DllPhaseR {
        DllPhaseR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Debug signal: do not shut off receiver when the packet has come"]
    #[inline(always)]
    pub fn force_rx(&self) -> ForceRxR {
        ForceRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 20 - Select type of bsae current"]
    #[inline(always)]
    pub fn crefsel(&self) -> CrefselR {
        CrefselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmitter mode"]
    #[inline(always)]
    pub fn tx_mode(&self) -> TxModeR {
        TxModeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - RPU compensation mode"]
    #[inline(always)]
    pub fn rpumode(&self) -> RpumodeR {
        RpumodeR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 31 - Tune enable"]
    #[inline(always)]
    pub fn tune_en(&self) -> TuneEnR {
        TuneEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter output resistor tune code"]
    #[inline(always)]
    pub fn rout_code(&mut self) -> RoutCodeW<'_, Phycfg1Spec> {
        RoutCodeW::new(self, 0)
    }
    #[doc = "Bit 4 - Automatic transmitter output resistor calibration"]
    #[inline(always)]
    pub fn rout_tune_en(&mut self) -> RoutTuneEnW<'_, Phycfg1Spec> {
        RoutTuneEnW::new(self, 4)
    }
    #[doc = "Bits 5:8 - Setting resistor tune code"]
    #[inline(always)]
    pub fn rset_code(&mut self) -> RsetCodeW<'_, Phycfg1Spec> {
        RsetCodeW::new(self, 5)
    }
    #[doc = "Bit 9 - Automatic calibration of resistor in current source"]
    #[inline(always)]
    pub fn rset_tune_en(&mut self) -> RsetTuneEnW<'_, Phycfg1Spec> {
        RsetTuneEnW::new(self, 9)
    }
    #[doc = "Bits 13:14 - Capture point in bit interval"]
    #[inline(always)]
    pub fn dll_phase(&mut self) -> DllPhaseW<'_, Phycfg1Spec> {
        DllPhaseW::new(self, 13)
    }
    #[doc = "Bit 15 - Debug signal: do not shut off receiver when the packet has come"]
    #[inline(always)]
    pub fn force_rx(&mut self) -> ForceRxW<'_, Phycfg1Spec> {
        ForceRxW::new(self, 15)
    }
    #[doc = "Bit 20 - Select type of bsae current"]
    #[inline(always)]
    pub fn crefsel(&mut self) -> CrefselW<'_, Phycfg1Spec> {
        CrefselW::new(self, 20)
    }
    #[doc = "Bit 23 - Transmitter mode"]
    #[inline(always)]
    pub fn tx_mode(&mut self) -> TxModeW<'_, Phycfg1Spec> {
        TxModeW::new(self, 23)
    }
    #[doc = "Bits 24:25 - RPU compensation mode"]
    #[inline(always)]
    pub fn rpumode(&mut self) -> RpumodeW<'_, Phycfg1Spec> {
        RpumodeW::new(self, 24)
    }
    #[doc = "Bit 31 - Tune enable"]
    #[inline(always)]
    pub fn tune_en(&mut self) -> TuneEnW<'_, Phycfg1Spec> {
        TuneEnW::new(self, 31)
    }
}
#[doc = "USB Physical Layer Config 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`phycfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phycfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phycfg1Spec;
impl crate::RegisterSpec for Phycfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phycfg1::R`](R) reader structure"]
impl crate::Readable for Phycfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`phycfg1::W`](W) writer structure"]
impl crate::Writable for Phycfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHYCFG1 to value 0"]
impl crate::Resettable for Phycfg1Spec {}
