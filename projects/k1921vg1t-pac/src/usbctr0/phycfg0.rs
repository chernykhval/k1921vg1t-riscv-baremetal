#[doc = "Register `PHYCFG0` reader"]
pub type R = crate::R<Phycfg0Spec>;
#[doc = "Register `PHYCFG0` writer"]
pub type W = crate::W<Phycfg0Spec>;
#[doc = "Field `EXTPHY` reader - Use external PHY Layer"]
pub type ExtphyR = crate::BitReader;
#[doc = "Field `EXTPHY` writer - Use external PHY Layer"]
pub type ExtphyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Polarity"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Polarity"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "ATP select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Atpsel {
    #[doc = "0: Internal analog mux is power off"]
    Off = 0,
    #[doc = "1: atp_0 connects to ground and atp_1 connects to comp_vr from calibration block"]
    CompVr = 1,
    #[doc = "2: atp_0 connects to ground and atp_1 connects to comp_vref from calibration block"]
    CompVref = 2,
    #[doc = "7: atp_0 connects to ground and atp_1 connects to VDDH"]
    Vddh = 7,
}
impl From<Atpsel> for u8 {
    #[inline(always)]
    fn from(variant: Atpsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Atpsel {
    type Ux = u8;
}
impl crate::IsEnum for Atpsel {}
#[doc = "Field `ATPSEL` reader - ATP select"]
pub type AtpselR = crate::FieldReader<Atpsel>;
impl AtpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Atpsel> {
        match self.bits {
            0 => Some(Atpsel::Off),
            1 => Some(Atpsel::CompVr),
            2 => Some(Atpsel::CompVref),
            7 => Some(Atpsel::Vddh),
            _ => None,
        }
    }
    #[doc = "Internal analog mux is power off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Atpsel::Off
    }
    #[doc = "atp_0 connects to ground and atp_1 connects to comp_vr from calibration block"]
    #[inline(always)]
    pub fn is_comp_vr(&self) -> bool {
        *self == Atpsel::CompVr
    }
    #[doc = "atp_0 connects to ground and atp_1 connects to comp_vref from calibration block"]
    #[inline(always)]
    pub fn is_comp_vref(&self) -> bool {
        *self == Atpsel::CompVref
    }
    #[doc = "atp_0 connects to ground and atp_1 connects to VDDH"]
    #[inline(always)]
    pub fn is_vddh(&self) -> bool {
        *self == Atpsel::Vddh
    }
}
#[doc = "Field `ATPSEL` writer - ATP select"]
pub type AtpselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Atpsel>;
impl<'a, REG> AtpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal analog mux is power off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Atpsel::Off)
    }
    #[doc = "atp_0 connects to ground and atp_1 connects to comp_vr from calibration block"]
    #[inline(always)]
    pub fn comp_vr(self) -> &'a mut crate::W<REG> {
        self.variant(Atpsel::CompVr)
    }
    #[doc = "atp_0 connects to ground and atp_1 connects to comp_vref from calibration block"]
    #[inline(always)]
    pub fn comp_vref(self) -> &'a mut crate::W<REG> {
        self.variant(Atpsel::CompVref)
    }
    #[doc = "atp_0 connects to ground and atp_1 connects to VDDH"]
    #[inline(always)]
    pub fn vddh(self) -> &'a mut crate::W<REG> {
        self.variant(Atpsel::Vddh)
    }
}
#[doc = "DTP select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtpsel {
    #[doc = "0: Internal analog mux is power off"]
    Off = 0,
    #[doc = "1: dtp connects to comparator output in calibration block"]
    OdtCompOut = 1,
    #[doc = "2: dtp connects to calibration block enable signal"]
    OdtEn = 2,
    #[doc = "3: dtp connects to DP output of the receiver"]
    RxSeDp = 3,
    #[doc = "4: dtp connects to DM output of the receiver"]
    RxSeDm = 4,
    #[doc = "5: dtp connects to diff output of the receiver"]
    RxLsfsSdata = 5,
    #[doc = "6: dtp connects to enable signal of bias current source"]
    CrefEn = 6,
    #[doc = "7: dtp connect RPU resistor"]
    EnRpu = 7,
    #[doc = "8: dtp connect RPU resistor"]
    EnRpuLow = 8,
    #[doc = "9: dtp connects to transmitter data"]
    TxSdata = 9,
    #[doc = "10: dtp connects to enable transmitter signal"]
    TxLsfsEn = 10,
    #[doc = "11: dtp connects to output enable signal of the transmitter"]
    TxLsfsOeb = 11,
    #[doc = "12: dtp connects to SE0"]
    TxLsfsSe0 = 12,
    #[doc = "13: dtp connects to enable receiver signal"]
    RxSeEn = 13,
    #[doc = "14: dtp connects to receiver enable signal"]
    RxLsfsEn = 14,
    #[doc = "15: dtp connects to compensation buffer ready signal"]
    EbufValid = 15,
    #[doc = "16: dtp connects to compensation buffer error signal"]
    EbufError = 16,
    #[doc = "17: dtp connects to SYNC pattern detector"]
    Sync = 17,
    #[doc = "18: dtp connects to EOP pattern detector"]
    Eop = 18,
    #[doc = "19: dtp connects to reset signal"]
    Reset = 19,
    #[doc = "20: dtp connects to pll lock signal"]
    Plllock = 20,
    #[doc = "21: dtp connects to RXActive signal"]
    Rxactive = 21,
    #[doc = "22: dtp connects tp RXError signal"]
    Rxerror = 22,
}
impl From<Dtpsel> for u8 {
    #[inline(always)]
    fn from(variant: Dtpsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtpsel {
    type Ux = u8;
}
impl crate::IsEnum for Dtpsel {}
#[doc = "Field `DTPSEL` reader - DTP select"]
pub type DtpselR = crate::FieldReader<Dtpsel>;
impl DtpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dtpsel> {
        match self.bits {
            0 => Some(Dtpsel::Off),
            1 => Some(Dtpsel::OdtCompOut),
            2 => Some(Dtpsel::OdtEn),
            3 => Some(Dtpsel::RxSeDp),
            4 => Some(Dtpsel::RxSeDm),
            5 => Some(Dtpsel::RxLsfsSdata),
            6 => Some(Dtpsel::CrefEn),
            7 => Some(Dtpsel::EnRpu),
            8 => Some(Dtpsel::EnRpuLow),
            9 => Some(Dtpsel::TxSdata),
            10 => Some(Dtpsel::TxLsfsEn),
            11 => Some(Dtpsel::TxLsfsOeb),
            12 => Some(Dtpsel::TxLsfsSe0),
            13 => Some(Dtpsel::RxSeEn),
            14 => Some(Dtpsel::RxLsfsEn),
            15 => Some(Dtpsel::EbufValid),
            16 => Some(Dtpsel::EbufError),
            17 => Some(Dtpsel::Sync),
            18 => Some(Dtpsel::Eop),
            19 => Some(Dtpsel::Reset),
            20 => Some(Dtpsel::Plllock),
            21 => Some(Dtpsel::Rxactive),
            22 => Some(Dtpsel::Rxerror),
            _ => None,
        }
    }
    #[doc = "Internal analog mux is power off"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Dtpsel::Off
    }
    #[doc = "dtp connects to comparator output in calibration block"]
    #[inline(always)]
    pub fn is_odt_comp_out(&self) -> bool {
        *self == Dtpsel::OdtCompOut
    }
    #[doc = "dtp connects to calibration block enable signal"]
    #[inline(always)]
    pub fn is_odt_en(&self) -> bool {
        *self == Dtpsel::OdtEn
    }
    #[doc = "dtp connects to DP output of the receiver"]
    #[inline(always)]
    pub fn is_rx_se_dp(&self) -> bool {
        *self == Dtpsel::RxSeDp
    }
    #[doc = "dtp connects to DM output of the receiver"]
    #[inline(always)]
    pub fn is_rx_se_dm(&self) -> bool {
        *self == Dtpsel::RxSeDm
    }
    #[doc = "dtp connects to diff output of the receiver"]
    #[inline(always)]
    pub fn is_rx_lsfs_sdata(&self) -> bool {
        *self == Dtpsel::RxLsfsSdata
    }
    #[doc = "dtp connects to enable signal of bias current source"]
    #[inline(always)]
    pub fn is_cref_en(&self) -> bool {
        *self == Dtpsel::CrefEn
    }
    #[doc = "dtp connect RPU resistor"]
    #[inline(always)]
    pub fn is_en_rpu(&self) -> bool {
        *self == Dtpsel::EnRpu
    }
    #[doc = "dtp connect RPU resistor"]
    #[inline(always)]
    pub fn is_en_rpu_low(&self) -> bool {
        *self == Dtpsel::EnRpuLow
    }
    #[doc = "dtp connects to transmitter data"]
    #[inline(always)]
    pub fn is_tx_sdata(&self) -> bool {
        *self == Dtpsel::TxSdata
    }
    #[doc = "dtp connects to enable transmitter signal"]
    #[inline(always)]
    pub fn is_tx_lsfs_en(&self) -> bool {
        *self == Dtpsel::TxLsfsEn
    }
    #[doc = "dtp connects to output enable signal of the transmitter"]
    #[inline(always)]
    pub fn is_tx_lsfs_oeb(&self) -> bool {
        *self == Dtpsel::TxLsfsOeb
    }
    #[doc = "dtp connects to SE0"]
    #[inline(always)]
    pub fn is_tx_lsfs_se0(&self) -> bool {
        *self == Dtpsel::TxLsfsSe0
    }
    #[doc = "dtp connects to enable receiver signal"]
    #[inline(always)]
    pub fn is_rx_se_en(&self) -> bool {
        *self == Dtpsel::RxSeEn
    }
    #[doc = "dtp connects to receiver enable signal"]
    #[inline(always)]
    pub fn is_rx_lsfs_en(&self) -> bool {
        *self == Dtpsel::RxLsfsEn
    }
    #[doc = "dtp connects to compensation buffer ready signal"]
    #[inline(always)]
    pub fn is_ebuf_valid(&self) -> bool {
        *self == Dtpsel::EbufValid
    }
    #[doc = "dtp connects to compensation buffer error signal"]
    #[inline(always)]
    pub fn is_ebuf_error(&self) -> bool {
        *self == Dtpsel::EbufError
    }
    #[doc = "dtp connects to SYNC pattern detector"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Dtpsel::Sync
    }
    #[doc = "dtp connects to EOP pattern detector"]
    #[inline(always)]
    pub fn is_eop(&self) -> bool {
        *self == Dtpsel::Eop
    }
    #[doc = "dtp connects to reset signal"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Dtpsel::Reset
    }
    #[doc = "dtp connects to pll lock signal"]
    #[inline(always)]
    pub fn is_plllock(&self) -> bool {
        *self == Dtpsel::Plllock
    }
    #[doc = "dtp connects to RXActive signal"]
    #[inline(always)]
    pub fn is_rxactive(&self) -> bool {
        *self == Dtpsel::Rxactive
    }
    #[doc = "dtp connects tp RXError signal"]
    #[inline(always)]
    pub fn is_rxerror(&self) -> bool {
        *self == Dtpsel::Rxerror
    }
}
#[doc = "Field `DTPSEL` writer - DTP select"]
pub type DtpselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Dtpsel>;
impl<'a, REG> DtpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal analog mux is power off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::Off)
    }
    #[doc = "dtp connects to comparator output in calibration block"]
    #[inline(always)]
    pub fn odt_comp_out(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::OdtCompOut)
    }
    #[doc = "dtp connects to calibration block enable signal"]
    #[inline(always)]
    pub fn odt_en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::OdtEn)
    }
    #[doc = "dtp connects to DP output of the receiver"]
    #[inline(always)]
    pub fn rx_se_dp(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::RxSeDp)
    }
    #[doc = "dtp connects to DM output of the receiver"]
    #[inline(always)]
    pub fn rx_se_dm(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::RxSeDm)
    }
    #[doc = "dtp connects to diff output of the receiver"]
    #[inline(always)]
    pub fn rx_lsfs_sdata(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::RxLsfsSdata)
    }
    #[doc = "dtp connects to enable signal of bias current source"]
    #[inline(always)]
    pub fn cref_en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::CrefEn)
    }
    #[doc = "dtp connect RPU resistor"]
    #[inline(always)]
    pub fn en_rpu(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::EnRpu)
    }
    #[doc = "dtp connect RPU resistor"]
    #[inline(always)]
    pub fn en_rpu_low(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::EnRpuLow)
    }
    #[doc = "dtp connects to transmitter data"]
    #[inline(always)]
    pub fn tx_sdata(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::TxSdata)
    }
    #[doc = "dtp connects to enable transmitter signal"]
    #[inline(always)]
    pub fn tx_lsfs_en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::TxLsfsEn)
    }
    #[doc = "dtp connects to output enable signal of the transmitter"]
    #[inline(always)]
    pub fn tx_lsfs_oeb(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::TxLsfsOeb)
    }
    #[doc = "dtp connects to SE0"]
    #[inline(always)]
    pub fn tx_lsfs_se0(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::TxLsfsSe0)
    }
    #[doc = "dtp connects to enable receiver signal"]
    #[inline(always)]
    pub fn rx_se_en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::RxSeEn)
    }
    #[doc = "dtp connects to receiver enable signal"]
    #[inline(always)]
    pub fn rx_lsfs_en(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::RxLsfsEn)
    }
    #[doc = "dtp connects to compensation buffer ready signal"]
    #[inline(always)]
    pub fn ebuf_valid(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::EbufValid)
    }
    #[doc = "dtp connects to compensation buffer error signal"]
    #[inline(always)]
    pub fn ebuf_error(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::EbufError)
    }
    #[doc = "dtp connects to SYNC pattern detector"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::Sync)
    }
    #[doc = "dtp connects to EOP pattern detector"]
    #[inline(always)]
    pub fn eop(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::Eop)
    }
    #[doc = "dtp connects to reset signal"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::Reset)
    }
    #[doc = "dtp connects to pll lock signal"]
    #[inline(always)]
    pub fn plllock(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::Plllock)
    }
    #[doc = "dtp connects to RXActive signal"]
    #[inline(always)]
    pub fn rxactive(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::Rxactive)
    }
    #[doc = "dtp connects tp RXError signal"]
    #[inline(always)]
    pub fn rxerror(self) -> &'a mut crate::W<REG> {
        self.variant(Dtpsel::Rxerror)
    }
}
impl R {
    #[doc = "Bit 0 - Use external PHY Layer"]
    #[inline(always)]
    pub fn extphy(&self) -> ExtphyR {
        ExtphyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Polarity"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - ATP select"]
    #[inline(always)]
    pub fn atpsel(&self) -> AtpselR {
        AtpselR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DTP select"]
    #[inline(always)]
    pub fn dtpsel(&self) -> DtpselR {
        DtpselR::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Use external PHY Layer"]
    #[inline(always)]
    pub fn extphy(&mut self) -> ExtphyW<'_, Phycfg0Spec> {
        ExtphyW::new(self, 0)
    }
    #[doc = "Bit 1 - Polarity"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<'_, Phycfg0Spec> {
        PolW::new(self, 1)
    }
    #[doc = "Bits 2:6 - ATP select"]
    #[inline(always)]
    pub fn atpsel(&mut self) -> AtpselW<'_, Phycfg0Spec> {
        AtpselW::new(self, 2)
    }
    #[doc = "Bits 8:12 - DTP select"]
    #[inline(always)]
    pub fn dtpsel(&mut self) -> DtpselW<'_, Phycfg0Spec> {
        DtpselW::new(self, 8)
    }
}
#[doc = "USB Physical Layer Config 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`phycfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phycfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Phycfg0Spec;
impl crate::RegisterSpec for Phycfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phycfg0::R`](R) reader structure"]
impl crate::Readable for Phycfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`phycfg0::W`](W) writer structure"]
impl crate::Writable for Phycfg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PHYCFG0 to value 0"]
impl crate::Resettable for Phycfg0Spec {}
