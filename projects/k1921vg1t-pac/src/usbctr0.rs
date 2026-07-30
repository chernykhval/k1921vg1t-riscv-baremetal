#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hosten: Hosten,
    phystat: Phystat,
    phycfg0: Phycfg0,
    phycfg1: Phycfg1,
}
impl RegisterBlock {
    #[doc = "0x00 - USB Host Enable register"]
    #[inline(always)]
    pub const fn hosten(&self) -> &Hosten {
        &self.hosten
    }
    #[doc = "0x04 - USB PHY Layer Status register"]
    #[inline(always)]
    pub const fn phystat(&self) -> &Phystat {
        &self.phystat
    }
    #[doc = "0x08 - USB Physical Layer Config 0 register"]
    #[inline(always)]
    pub const fn phycfg0(&self) -> &Phycfg0 {
        &self.phycfg0
    }
    #[doc = "0x0c - USB Physical Layer Config 1 register"]
    #[inline(always)]
    pub const fn phycfg1(&self) -> &Phycfg1 {
        &self.phycfg1
    }
}
#[doc = "HOSTEN (rw) register accessor: USB Host Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`hosten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hosten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hosten`] module"]
#[doc(alias = "HOSTEN")]
pub type Hosten = crate::Reg<hosten::HostenSpec>;
#[doc = "USB Host Enable register"]
pub mod hosten;
#[doc = "PHYSTAT (r) register accessor: USB PHY Layer Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`phystat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phystat`] module"]
#[doc(alias = "PHYSTAT")]
pub type Phystat = crate::Reg<phystat::PhystatSpec>;
#[doc = "USB PHY Layer Status register"]
pub mod phystat;
#[doc = "PHYCFG0 (rw) register accessor: USB Physical Layer Config 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`phycfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phycfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phycfg0`] module"]
#[doc(alias = "PHYCFG0")]
pub type Phycfg0 = crate::Reg<phycfg0::Phycfg0Spec>;
#[doc = "USB Physical Layer Config 0 register"]
pub mod phycfg0;
#[doc = "PHYCFG1 (rw) register accessor: USB Physical Layer Config 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`phycfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phycfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phycfg1`] module"]
#[doc(alias = "PHYCFG1")]
pub type Phycfg1 = crate::Reg<phycfg1::Phycfg1Spec>;
#[doc = "USB Physical Layer Config 1 register"]
pub mod phycfg1;
