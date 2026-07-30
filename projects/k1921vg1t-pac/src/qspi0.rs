#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hcr: Hcr,
    dcr: Dcr,
    dsr: Dsr,
    tcr: Tcr,
    tdr: Tdr,
    tds: Tds,
    qcc: Qcc,
    qad: Qad,
    qab: Qab,
    imr: Imr,
    icr: Icr,
    ris: Ris,
    fwm: Fwm,
    mis: Mis,
}
impl RegisterBlock {
    #[doc = "0x00 - Host Control Register"]
    #[inline(always)]
    pub const fn hcr(&self) -> &Hcr {
        &self.hcr
    }
    #[doc = "0x04 - Device Config Register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    #[doc = "0x08 - Device status register"]
    #[inline(always)]
    pub const fn dsr(&self) -> &Dsr {
        &self.dsr
    }
    #[doc = "0x0c - Transfer Config register"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x10 - Transfer Data register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    #[doc = "0x14 - Transfer Data size register"]
    #[inline(always)]
    pub const fn tds(&self) -> &Tds {
        &self.tds
    }
    #[doc = "0x18 - QSPI Communication Config register"]
    #[inline(always)]
    pub const fn qcc(&self) -> &Qcc {
        &self.qcc
    }
    #[doc = "0x1c - QSPI Address Register"]
    #[inline(always)]
    pub const fn qad(&self) -> &Qad {
        &self.qad
    }
    #[doc = "0x20 - QSPI Alternate Bytes Register"]
    #[inline(always)]
    pub const fn qab(&self) -> &Qab {
        &self.qab
    }
    #[doc = "0x24 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x28 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x2c - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x30 - Fifo's WaterMarks Register"]
    #[inline(always)]
    pub const fn fwm(&self) -> &Fwm {
        &self.fwm
    }
    #[doc = "0x34 - Masked Interrupt Status Register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
}
#[doc = "HCR (rw) register accessor: Host Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcr`] module"]
#[doc(alias = "HCR")]
pub type Hcr = crate::Reg<hcr::HcrSpec>;
#[doc = "Host Control Register"]
pub mod hcr;
#[doc = "DCR (rw) register accessor: Device Config Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`] module"]
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DcrSpec>;
#[doc = "Device Config Register"]
pub mod dcr;
#[doc = "DSR (r) register accessor: Device status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr`] module"]
#[doc(alias = "DSR")]
pub type Dsr = crate::Reg<dsr::DsrSpec>;
#[doc = "Device status register"]
pub mod dsr;
#[doc = "TCR (rw) register accessor: Transfer Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`] module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Transfer Config register"]
pub mod tcr;
#[doc = "TDR (rw) register accessor: Transfer Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`] module"]
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
#[doc = "Transfer Data register"]
pub mod tdr;
#[doc = "TDS (rw) register accessor: Transfer Data size register\n\nYou can [`read`](crate::Reg::read) this register and get [`tds::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tds::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tds`] module"]
#[doc(alias = "TDS")]
pub type Tds = crate::Reg<tds::TdsSpec>;
#[doc = "Transfer Data size register"]
pub mod tds;
#[doc = "QCC (rw) register accessor: QSPI Communication Config register\n\nYou can [`read`](crate::Reg::read) this register and get [`qcc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qcc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qcc`] module"]
#[doc(alias = "QCC")]
pub type Qcc = crate::Reg<qcc::QccSpec>;
#[doc = "QSPI Communication Config register"]
pub mod qcc;
#[doc = "QAD (rw) register accessor: QSPI Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qad`] module"]
#[doc(alias = "QAD")]
pub type Qad = crate::Reg<qad::QadSpec>;
#[doc = "QSPI Address Register"]
pub mod qad;
#[doc = "QAB (rw) register accessor: QSPI Alternate Bytes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qab::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qab::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@qab`] module"]
#[doc(alias = "QAB")]
pub type Qab = crate::Reg<qab::QabSpec>;
#[doc = "QSPI Alternate Bytes Register"]
pub mod qab;
#[doc = "IMR (rw) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ICR (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "RIS (r) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod ris;
#[doc = "FWM (rw) register accessor: Fifo's WaterMarks Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fwm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fwm`] module"]
#[doc(alias = "FWM")]
pub type Fwm = crate::Reg<fwm::FwmSpec>;
#[doc = "Fifo's WaterMarks Register"]
pub mod fwm;
#[doc = "MIS (r) register accessor: Masked Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked Interrupt Status Register"]
pub mod mis;
