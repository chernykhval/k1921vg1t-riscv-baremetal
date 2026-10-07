#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dacctl: Dacctl,
    acmpctl: Acmpctl,
    acstatus: Acstatus,
    _reserved3: [u8; 0x04],
    inten: Inten,
    ris: Ris,
    mis: Mis,
    iclr: Iclr,
}
impl RegisterBlock {
    #[doc = "0x00 - DAC control register"]
    #[inline(always)]
    pub const fn dacctl(&self) -> &Dacctl {
        &self.dacctl
    }
    #[doc = "0x04 - ACMP control register"]
    #[inline(always)]
    pub const fn acmpctl(&self) -> &Acmpctl {
        &self.acmpctl
    }
    #[doc = "0x08 - ACMP status"]
    #[inline(always)]
    pub const fn acstatus(&self) -> &Acstatus {
        &self.acstatus
    }
    #[doc = "0x10 - Interrupt enable"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x14 - Raw interrupt status"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x18 - Masked interrupt status"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x1c - Clear interrupt status"]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
}
#[doc = "DACCTL (rw) register accessor: DAC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dacctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacctl`] module"]
#[doc(alias = "DACCTL")]
pub type Dacctl = crate::Reg<dacctl::DacctlSpec>;
#[doc = "DAC control register"]
pub mod dacctl;
#[doc = "ACMPCTL (rw) register accessor: ACMP control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acmpctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmpctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acmpctl`] module"]
#[doc(alias = "ACMPCTL")]
pub type Acmpctl = crate::Reg<acmpctl::AcmpctlSpec>;
#[doc = "ACMP control register"]
pub mod acmpctl;
#[doc = "ACSTATUS (r) register accessor: ACMP status\n\nYou can [`read`](crate::Reg::read) this register and get [`acstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acstatus`] module"]
#[doc(alias = "ACSTATUS")]
pub type Acstatus = crate::Reg<acstatus::AcstatusSpec>;
#[doc = "ACMP status"]
pub mod acstatus;
#[doc = "INTEN (rw) register accessor: Interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt enable"]
pub mod inten;
#[doc = "RIS (r) register accessor: Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status"]
pub mod mis;
#[doc = "ICLR (w) register accessor: Clear interrupt status\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`] module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Clear interrupt status"]
pub mod iclr;
