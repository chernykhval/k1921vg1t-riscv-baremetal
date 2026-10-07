#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cfg: Cfg,
    irqen: Irqen,
    irqstat: Irqstat,
    error_pos: ErrorPos,
    seu_cntr: SeuCntr,
    deu_cntr: DeuCntr,
    seu_addr: SeuAddr,
    deu_addr: DeuAddr,
    latency: Latency,
    errmask: Errmask,
    reserved: Reserved,
}
impl RegisterBlock {
    #[doc = "0x00 - SRAM ECC Control register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - SRAM ECC Interrupt Enable register"]
    #[inline(always)]
    pub const fn irqen(&self) -> &Irqen {
        &self.irqen
    }
    #[doc = "0x08 - SRAM ECC Interrupt Status register"]
    #[inline(always)]
    pub const fn irqstat(&self) -> &Irqstat {
        &self.irqstat
    }
    #[doc = "0x0c - SRAM Error Position in Byte register"]
    #[inline(always)]
    pub const fn error_pos(&self) -> &ErrorPos {
        &self.error_pos
    }
    #[doc = "0x10 - SRAM Single Error Counter register"]
    #[inline(always)]
    pub const fn seu_cntr(&self) -> &SeuCntr {
        &self.seu_cntr
    }
    #[doc = "0x14 - SRAM Double Error Counter register"]
    #[inline(always)]
    pub const fn deu_cntr(&self) -> &DeuCntr {
        &self.deu_cntr
    }
    #[doc = "0x18 - SRAM Single Error Address register"]
    #[inline(always)]
    pub const fn seu_addr(&self) -> &SeuAddr {
        &self.seu_addr
    }
    #[doc = "0x1c - SRAM Double Error Address register"]
    #[inline(always)]
    pub const fn deu_addr(&self) -> &DeuAddr {
        &self.deu_addr
    }
    #[doc = "0x20 - SRAM Read and Write Latency register"]
    #[inline(always)]
    pub const fn latency(&self) -> &Latency {
        &self.latency
    }
    #[doc = "0x24 - SRAM Data Mask register"]
    #[inline(always)]
    pub const fn errmask(&self) -> &Errmask {
        &self.errmask
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
}
#[doc = "CFG (rw) register accessor: SRAM ECC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "SRAM ECC Control register"]
pub mod cfg;
#[doc = "IRQEN (rw) register accessor: SRAM ECC Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqen`] module"]
#[doc(alias = "IRQEN")]
pub type Irqen = crate::Reg<irqen::IrqenSpec>;
#[doc = "SRAM ECC Interrupt Enable register"]
pub mod irqen;
#[doc = "IRQSTAT (rw) register accessor: SRAM ECC Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`irqstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqstat`] module"]
#[doc(alias = "IRQSTAT")]
pub type Irqstat = crate::Reg<irqstat::IrqstatSpec>;
#[doc = "SRAM ECC Interrupt Status register"]
pub mod irqstat;
#[doc = "ERROR_POS (rw) register accessor: SRAM Error Position in Byte register\n\nYou can [`read`](crate::Reg::read) this register and get [`error_pos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`error_pos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error_pos`] module"]
#[doc(alias = "ERROR_POS")]
pub type ErrorPos = crate::Reg<error_pos::ErrorPosSpec>;
#[doc = "SRAM Error Position in Byte register"]
pub mod error_pos;
#[doc = "SEU_CNTR (rw) register accessor: SRAM Single Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`seu_cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seu_cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seu_cntr`] module"]
#[doc(alias = "SEU_CNTR")]
pub type SeuCntr = crate::Reg<seu_cntr::SeuCntrSpec>;
#[doc = "SRAM Single Error Counter register"]
pub mod seu_cntr;
#[doc = "DEU_CNTR (rw) register accessor: SRAM Double Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`deu_cntr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deu_cntr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deu_cntr`] module"]
#[doc(alias = "DEU_CNTR")]
pub type DeuCntr = crate::Reg<deu_cntr::DeuCntrSpec>;
#[doc = "SRAM Double Error Counter register"]
pub mod deu_cntr;
#[doc = "SEU_ADDR (rw) register accessor: SRAM Single Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`seu_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seu_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seu_addr`] module"]
#[doc(alias = "SEU_ADDR")]
pub type SeuAddr = crate::Reg<seu_addr::SeuAddrSpec>;
#[doc = "SRAM Single Error Address register"]
pub mod seu_addr;
#[doc = "DEU_ADDR (rw) register accessor: SRAM Double Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`deu_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deu_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deu_addr`] module"]
#[doc(alias = "DEU_ADDR")]
pub type DeuAddr = crate::Reg<deu_addr::DeuAddrSpec>;
#[doc = "SRAM Double Error Address register"]
pub mod deu_addr;
#[doc = "LATENCY (rw) register accessor: SRAM Read and Write Latency register\n\nYou can [`read`](crate::Reg::read) this register and get [`latency::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`latency::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@latency`] module"]
#[doc(alias = "LATENCY")]
pub type Latency = crate::Reg<latency::LatencySpec>;
#[doc = "SRAM Read and Write Latency register"]
pub mod latency;
#[doc = "ERRMASK (rw) register accessor: SRAM Data Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`errmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@errmask`] module"]
#[doc(alias = "ERRMASK")]
pub type Errmask = crate::Reg<errmask::ErrmaskSpec>;
#[doc = "SRAM Data Mask register"]
pub mod errmask;
#[doc = "Reserved (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`] module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = ""]
pub mod reserved;
