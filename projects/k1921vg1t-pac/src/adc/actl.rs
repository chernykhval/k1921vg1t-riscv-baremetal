#[repr(C)]
#[doc = "ACTL"]
#[doc(alias = "ACTL")]
pub struct Actl {
    ctrl: Ctrl,
    tacc0: Tacc0,
    tacc1: Tacc1,
    reserved: Reserved,
}
impl Actl {
    #[doc = "0x00 - ADC module control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - ADC test access register 0"]
    #[inline(always)]
    pub const fn tacc0(&self) -> &Tacc0 {
        &self.tacc0
    }
    #[doc = "0x08 - ADC test access register 1"]
    #[inline(always)]
    pub const fn tacc1(&self) -> &Tacc1 {
        &self.tacc1
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
}
#[doc = "CTRL (rw) register accessor: ADC module control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "ADC module control register"]
pub mod ctrl;
#[doc = "TACC0 (rw) register accessor: ADC test access register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tacc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tacc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacc0`] module"]
#[doc(alias = "TACC0")]
pub type Tacc0 = crate::Reg<tacc0::Tacc0Spec>;
#[doc = "ADC test access register 0"]
pub mod tacc0;
#[doc = "TACC1 (rw) register accessor: ADC test access register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tacc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tacc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tacc1`] module"]
#[doc(alias = "TACC1")]
pub type Tacc1 = crate::Reg<tacc1::Tacc1Spec>;
#[doc = "ADC test access register 1"]
pub mod tacc1;
#[doc = "Reserved (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`] module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = ""]
pub mod reserved;
