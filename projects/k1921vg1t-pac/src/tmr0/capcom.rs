#[repr(C)]
#[doc = "CAPCOM"]
#[doc(alias = "CAPCOM")]
pub struct Capcom {
    ctrl: Ctrl,
    val0: Val0,
    val1: Val1,
    reserved: Reserved,
}
impl Capcom {
    #[doc = "0x00 - Capture / Compare Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Capture/Compare Value 0 register"]
    #[inline(always)]
    pub const fn val0(&self) -> &Val0 {
        &self.val0
    }
    #[doc = "0x08 - Capture/Compare Value 1 register"]
    #[inline(always)]
    pub const fn val1(&self) -> &Val1 {
        &self.val1
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
}
#[doc = "CTRL (rw) register accessor: Capture / Compare Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Capture / Compare Control register"]
pub mod ctrl;
#[doc = "VAL0 (rw) register accessor: Capture/Compare Value 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`val0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`val0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@val0`] module"]
#[doc(alias = "VAL0")]
pub type Val0 = crate::Reg<val0::Val0Spec>;
#[doc = "Capture/Compare Value 0 register"]
pub mod val0;
#[doc = "VAL1 (rw) register accessor: Capture/Compare Value 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`val1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`val1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@val1`] module"]
#[doc(alias = "VAL1")]
pub type Val1 = crate::Reg<val1::Val1Spec>;
#[doc = "Capture/Compare Value 1 register"]
pub mod val1;
#[doc = "Reserved (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`] module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = ""]
pub mod reserved;
