#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dtx: [Dtx; 8],
    drx: [Drx; 8],
    cr: Cr,
    sr: Sr,
    bl: Bl,
    srr: Srr,
    ptx: Ptx,
    prx: Prx,
    csrx: Csrx,
    im: Im,
    mis: Mis,
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - DTX"]
    #[inline(always)]
    pub const fn dtx(&self, n: usize) -> &Dtx {
        &self.dtx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - DTX"]
    #[inline(always)]
    pub fn dtx_iter(&self) -> impl Iterator<Item = &Dtx> {
        self.dtx.iter()
    }
    #[doc = "0x20..0x40 - DRX"]
    #[inline(always)]
    pub const fn drx(&self, n: usize) -> &Drx {
        &self.drx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - DRX"]
    #[inline(always)]
    pub fn drx_iter(&self) -> impl Iterator<Item = &Drx> {
        self.drx.iter()
    }
    #[doc = "0x40 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x44 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x48 - Register for setting the length of the BREAK field"]
    #[inline(always)]
    pub const fn bl(&self) -> &Bl {
        &self.bl
    }
    #[doc = "0x4c - Transfer rate setting register"]
    #[inline(always)]
    pub const fn srr(&self) -> &Srr {
        &self.srr
    }
    #[doc = "0x50 - Register of the PID being sent"]
    #[inline(always)]
    pub const fn ptx(&self) -> &Ptx {
        &self.ptx
    }
    #[doc = "0x54 - Register of the received PID"]
    #[inline(always)]
    pub const fn prx(&self) -> &Prx {
        &self.prx
    }
    #[doc = "0x58 - Register of the received checksum"]
    #[inline(always)]
    pub const fn csrx(&self) -> &Csrx {
        &self.csrx
    }
    #[doc = "0x5c - Mask interrupt register"]
    #[inline(always)]
    pub const fn im(&self) -> &Im {
        &self.im
    }
    #[doc = "0x60 - Masked interrupt status register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
}
#[doc = "DTX"]
pub use self::dtx::Dtx;
#[doc = r"Cluster"]
#[doc = "DTX"]
pub mod dtx;
#[doc = "DRX"]
pub use self::drx::Drx;
#[doc = r"Cluster"]
#[doc = "DRX"]
pub mod drx;
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
#[doc = "SR (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
#[doc = "BL (rw) register accessor: Register for setting the length of the BREAK field\n\nYou can [`read`](crate::Reg::read) this register and get [`bl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bl`] module"]
#[doc(alias = "BL")]
pub type Bl = crate::Reg<bl::BlSpec>;
#[doc = "Register for setting the length of the BREAK field"]
pub mod bl;
#[doc = "SRR (rw) register accessor: Transfer rate setting register\n\nYou can [`read`](crate::Reg::read) this register and get [`srr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srr`] module"]
#[doc(alias = "SRR")]
pub type Srr = crate::Reg<srr::SrrSpec>;
#[doc = "Transfer rate setting register"]
pub mod srr;
#[doc = "PTX (rw) register accessor: Register of the PID being sent\n\nYou can [`read`](crate::Reg::read) this register and get [`ptx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptx`] module"]
#[doc(alias = "PTX")]
pub type Ptx = crate::Reg<ptx::PtxSpec>;
#[doc = "Register of the PID being sent"]
pub mod ptx;
#[doc = "PRX (r) register accessor: Register of the received PID\n\nYou can [`read`](crate::Reg::read) this register and get [`prx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prx`] module"]
#[doc(alias = "PRX")]
pub type Prx = crate::Reg<prx::PrxSpec>;
#[doc = "Register of the received PID"]
pub mod prx;
#[doc = "CSRX (r) register accessor: Register of the received checksum\n\nYou can [`read`](crate::Reg::read) this register and get [`csrx::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csrx`] module"]
#[doc(alias = "CSRX")]
pub type Csrx = crate::Reg<csrx::CsrxSpec>;
#[doc = "Register of the received checksum"]
pub mod csrx;
#[doc = "IM (rw) register accessor: Mask interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`] module"]
#[doc(alias = "IM")]
pub type Im = crate::Reg<im::ImSpec>;
#[doc = "Mask interrupt register"]
pub mod im;
#[doc = "MIS (rw) register accessor: Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status register"]
pub mod mis;
