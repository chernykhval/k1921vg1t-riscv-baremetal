#[doc = "Register `PHYSTAT` reader"]
pub type R = crate::R<PhystatSpec>;
#[doc = "Field `ROUT_CODE` reader - TX_ROUT_CODE - transmitter output resistor tune code value"]
pub type RoutCodeR = crate::FieldReader;
#[doc = "Field `RSET_CODE` reader - CREF_RSOURCE_CODE - setting resistor tune code value"]
pub type RsetCodeR = crate::FieldReader;
#[doc = "Field `ROUT_LOCKED` reader - ROUT_LOCKED output status"]
pub type RoutLockedR = crate::BitReader;
#[doc = "Field `RS_LOCKED` reader - RSOURCE_LOCKED output status"]
pub type RsLockedR = crate::BitReader;
#[doc = "Field `UTMI_CLK_EN` reader - Clock out signal present on UTMI interface"]
pub type UtmiClkEnR = crate::BitReader;
#[doc = "Field `PLL_LOCK` reader - PLL_LOCK signal status"]
pub type PllLockR = crate::BitReader;
#[doc = "Field `PD_PLL_REQ` reader - PD_PLL_REQ signal status"]
pub type PdPllReqR = crate::BitReader;
#[doc = "Field `RESET` reader - RESET signal status"]
pub type ResetR = crate::BitReader;
#[doc = "Field `SUSPENDM` reader - SUSPENDM signal status"]
pub type SuspendmR = crate::BitReader;
#[doc = "Field `TERMSELECT` reader - TERMSELECT signal status"]
pub type TermselectR = crate::BitReader;
#[doc = "Field `HST_DISC` reader - HOST_DISCONNECT signal status"]
pub type HstDiscR = crate::BitReader;
#[doc = "Field `XCVRSELECT` reader - XCVRSELECT bus status"]
pub type XcvrselectR = crate::FieldReader;
#[doc = "Field `OPMODE` reader - OPMODE bus status"]
pub type OpmodeR = crate::FieldReader;
#[doc = "Field `EN_RPU` reader - EN_RPU signal status"]
pub type EnRpuR = crate::BitReader;
#[doc = "Field `TX_LSFS_EN` reader - TX_LSFS_EN signal status"]
pub type TxLsfsEnR = crate::BitReader;
#[doc = "Field `RX_SE_EN` reader - RX_SE_EN signal status"]
pub type RxSeEnR = crate::BitReader;
#[doc = "Field `RX_LSFS_EN` reader - RX_LSFS_EN signal status"]
pub type RxLsfsEnR = crate::BitReader;
#[doc = "Field `RECEIVE_EN` reader - Receiver status in digital part of phy"]
pub type ReceiveEnR = crate::BitReader;
#[doc = "Field `POL` reader - Polarity"]
pub type PolR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - TX_ROUT_CODE - transmitter output resistor tune code value"]
    #[inline(always)]
    pub fn rout_code(&self) -> RoutCodeR {
        RoutCodeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CREF_RSOURCE_CODE - setting resistor tune code value"]
    #[inline(always)]
    pub fn rset_code(&self) -> RsetCodeR {
        RsetCodeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 11 - ROUT_LOCKED output status"]
    #[inline(always)]
    pub fn rout_locked(&self) -> RoutLockedR {
        RoutLockedR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RSOURCE_LOCKED output status"]
    #[inline(always)]
    pub fn rs_locked(&self) -> RsLockedR {
        RsLockedR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Clock out signal present on UTMI interface"]
    #[inline(always)]
    pub fn utmi_clk_en(&self) -> UtmiClkEnR {
        UtmiClkEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - PLL_LOCK signal status"]
    #[inline(always)]
    pub fn pll_lock(&self) -> PllLockR {
        PllLockR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PD_PLL_REQ signal status"]
    #[inline(always)]
    pub fn pd_pll_req(&self) -> PdPllReqR {
        PdPllReqR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - RESET signal status"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SUSPENDM signal status"]
    #[inline(always)]
    pub fn suspendm(&self) -> SuspendmR {
        SuspendmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TERMSELECT signal status"]
    #[inline(always)]
    pub fn termselect(&self) -> TermselectR {
        TermselectR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - HOST_DISCONNECT signal status"]
    #[inline(always)]
    pub fn hst_disc(&self) -> HstDiscR {
        HstDiscR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - XCVRSELECT bus status"]
    #[inline(always)]
    pub fn xcvrselect(&self) -> XcvrselectR {
        XcvrselectR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - OPMODE bus status"]
    #[inline(always)]
    pub fn opmode(&self) -> OpmodeR {
        OpmodeR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - EN_RPU signal status"]
    #[inline(always)]
    pub fn en_rpu(&self) -> EnRpuR {
        EnRpuR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - TX_LSFS_EN signal status"]
    #[inline(always)]
    pub fn tx_lsfs_en(&self) -> TxLsfsEnR {
        TxLsfsEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - RX_SE_EN signal status"]
    #[inline(always)]
    pub fn rx_se_en(&self) -> RxSeEnR {
        RxSeEnR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - RX_LSFS_EN signal status"]
    #[inline(always)]
    pub fn rx_lsfs_en(&self) -> RxLsfsEnR {
        RxLsfsEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Receiver status in digital part of phy"]
    #[inline(always)]
    pub fn receive_en(&self) -> ReceiveEnR {
        ReceiveEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Polarity"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "USB PHY Layer Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`phystat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhystatSpec;
impl crate::RegisterSpec for PhystatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phystat::R`](R) reader structure"]
impl crate::Readable for PhystatSpec {}
#[doc = "`reset()` method sets PHYSTAT to value 0"]
impl crate::Resettable for PhystatSpec {}
