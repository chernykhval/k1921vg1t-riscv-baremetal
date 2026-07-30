#[doc = "Register `FLAG_CTRL` writer"]
pub type W = crate::W<FlagCtrlSpec>;
#[doc = "Field `CLR_DF` writer - Clear Data Frame flag"]
pub type ClrDfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_CF` writer - Clear Command Frame flag"]
pub type ClrCfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_TF` writer - Clear passive Time Frame flag"]
pub type ClrTfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RD` writer - Clear Receive upstream Data flag"]
pub type ClrRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_UTO` writer - Clear Upstream Time-Out flag"]
pub type ClrUtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_DP` writer - Clear Data Pending flag"]
pub type ClrDpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_CP` writer - Clear Command pending flag"]
pub type ClrCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_DD` writer - Clear Downstream Disabled flag"]
pub type ClrDdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_DF` writer - Set Data Frame flag"]
pub type SetDfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_CF` writer - Set Command Frame flag"]
pub type SetCfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_TF` writer - Set passive Time Frame flag"]
pub type SetTfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_RD` writer - Set Receive upstream Data flag"]
pub type SetRdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_UTO` writer - Set Upstream Time-Out flag"]
pub type SetUtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_DP` writer - Set Data Pending flag"]
pub type SetDpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_CP` writer - vCommand pending flag"]
pub type SetCpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET_DD` writer - Set Downstream Disabled flag"]
pub type SetDdW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Data Frame flag"]
    #[inline(always)]
    pub fn clr_df(&mut self) -> ClrDfW<'_, FlagCtrlSpec> {
        ClrDfW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Command Frame flag"]
    #[inline(always)]
    pub fn clr_cf(&mut self) -> ClrCfW<'_, FlagCtrlSpec> {
        ClrCfW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear passive Time Frame flag"]
    #[inline(always)]
    pub fn clr_tf(&mut self) -> ClrTfW<'_, FlagCtrlSpec> {
        ClrTfW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Receive upstream Data flag"]
    #[inline(always)]
    pub fn clr_rd(&mut self) -> ClrRdW<'_, FlagCtrlSpec> {
        ClrRdW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Upstream Time-Out flag"]
    #[inline(always)]
    pub fn clr_uto(&mut self) -> ClrUtoW<'_, FlagCtrlSpec> {
        ClrUtoW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Data Pending flag"]
    #[inline(always)]
    pub fn clr_dp(&mut self) -> ClrDpW<'_, FlagCtrlSpec> {
        ClrDpW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Command pending flag"]
    #[inline(always)]
    pub fn clr_cp(&mut self) -> ClrCpW<'_, FlagCtrlSpec> {
        ClrCpW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear Downstream Disabled flag"]
    #[inline(always)]
    pub fn clr_dd(&mut self) -> ClrDdW<'_, FlagCtrlSpec> {
        ClrDdW::new(self, 7)
    }
    #[doc = "Bit 16 - Set Data Frame flag"]
    #[inline(always)]
    pub fn set_df(&mut self) -> SetDfW<'_, FlagCtrlSpec> {
        SetDfW::new(self, 16)
    }
    #[doc = "Bit 17 - Set Command Frame flag"]
    #[inline(always)]
    pub fn set_cf(&mut self) -> SetCfW<'_, FlagCtrlSpec> {
        SetCfW::new(self, 17)
    }
    #[doc = "Bit 18 - Set passive Time Frame flag"]
    #[inline(always)]
    pub fn set_tf(&mut self) -> SetTfW<'_, FlagCtrlSpec> {
        SetTfW::new(self, 18)
    }
    #[doc = "Bit 19 - Set Receive upstream Data flag"]
    #[inline(always)]
    pub fn set_rd(&mut self) -> SetRdW<'_, FlagCtrlSpec> {
        SetRdW::new(self, 19)
    }
    #[doc = "Bit 20 - Set Upstream Time-Out flag"]
    #[inline(always)]
    pub fn set_uto(&mut self) -> SetUtoW<'_, FlagCtrlSpec> {
        SetUtoW::new(self, 20)
    }
    #[doc = "Bit 21 - Set Data Pending flag"]
    #[inline(always)]
    pub fn set_dp(&mut self) -> SetDpW<'_, FlagCtrlSpec> {
        SetDpW::new(self, 21)
    }
    #[doc = "Bit 22 - vCommand pending flag"]
    #[inline(always)]
    pub fn set_cp(&mut self) -> SetCpW<'_, FlagCtrlSpec> {
        SetCpW::new(self, 22)
    }
    #[doc = "Bit 23 - Set Downstream Disabled flag"]
    #[inline(always)]
    pub fn set_dd(&mut self) -> SetDdW<'_, FlagCtrlSpec> {
        SetDdW::new(self, 23)
    }
}
#[doc = "Flags control register (set/clear)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flag_ctrl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlagCtrlSpec;
impl crate::RegisterSpec for FlagCtrlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`flag_ctrl::W`](W) writer structure"]
impl crate::Writable for FlagCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLAG_CTRL to value 0"]
impl crate::Resettable for FlagCtrlSpec {}
