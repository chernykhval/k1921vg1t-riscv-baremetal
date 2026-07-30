#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    seqen: Seqen,
    seqsync: Seqsync,
    fstat: Fstat,
    bstat: Bstat,
    dctrig: Dctrig,
    dcev: Dcev,
    cicnt: Cicnt,
    emux: Emux,
    ris: Ris,
    im: Im,
    mis: Mis,
    ic: Ic,
    _reserved12: [u8; 0x10],
    seq: [Seq; 8],
    _reserved13: [u8; 0x01e0],
    dc: [Dc; 24],
    _reserved14: [u8; 0x20],
    actl: [Actl; 4],
    chctl: [Chctl; 64],
    chdelay: [Chdelay; 64],
}
impl RegisterBlock {
    #[doc = "0x00 - Enable sequencer register"]
    #[inline(always)]
    pub const fn seqen(&self) -> &Seqen {
        &self.seqen
    }
    #[doc = "0x04 - Sequencer sync register"]
    #[inline(always)]
    pub const fn seqsync(&self) -> &Seqsync {
        &self.seqsync
    }
    #[doc = "0x08 - FIFO overflow status register"]
    #[inline(always)]
    pub const fn fstat(&self) -> &Fstat {
        &self.fstat
    }
    #[doc = "0x0c - Busy status register"]
    #[inline(always)]
    pub const fn bstat(&self) -> &Bstat {
        &self.bstat
    }
    #[doc = "0x10 - Digital comparator output trigger status register"]
    #[inline(always)]
    pub const fn dctrig(&self) -> &Dctrig {
        &self.dctrig
    }
    #[doc = "0x14 - Digital comparator compare event status register"]
    #[inline(always)]
    pub const fn dcev(&self) -> &Dcev {
        &self.dcev
    }
    #[doc = "0x18 - Interrupt counter clear control"]
    #[inline(always)]
    pub const fn cicnt(&self) -> &Cicnt {
        &self.cicnt
    }
    #[doc = "0x1c - Sequencer start event selection register"]
    #[inline(always)]
    pub const fn emux(&self) -> &Emux {
        &self.emux
    }
    #[doc = "0x20 - Raw interrupt status register"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x24 - Interrupt mask register"]
    #[inline(always)]
    pub const fn im(&self) -> &Im {
        &self.im
    }
    #[doc = "0x28 - Masked interrupt status and clear register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x2c - Interrupt clear register"]
    #[inline(always)]
    pub const fn ic(&self) -> &Ic {
        &self.ic
    }
    #[doc = "0x40..0x220 - SEQ"]
    #[inline(always)]
    pub const fn seq(&self, n: usize) -> &Seq {
        &self.seq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x220 - SEQ"]
    #[inline(always)]
    pub fn seq_iter(&self) -> impl Iterator<Item = &Seq> {
        self.seq.iter()
    }
    #[doc = "0x400..0x520 - DC"]
    #[inline(always)]
    pub const fn dc(&self, n: usize) -> &Dc {
        &self.dc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x520 - DC"]
    #[inline(always)]
    pub fn dc_iter(&self) -> impl Iterator<Item = &Dc> {
        self.dc.iter()
    }
    #[doc = "0x540..0x580 - ACTL"]
    #[inline(always)]
    pub const fn actl(&self, n: usize) -> &Actl {
        &self.actl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x540..0x580 - ACTL"]
    #[inline(always)]
    pub fn actl_iter(&self) -> impl Iterator<Item = &Actl> {
        self.actl.iter()
    }
    #[doc = "0x580..0x680 - CHCTL"]
    #[inline(always)]
    pub const fn chctl(&self, n: usize) -> &Chctl {
        &self.chctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x580..0x680 - CHCTL"]
    #[inline(always)]
    pub fn chctl_iter(&self) -> impl Iterator<Item = &Chctl> {
        self.chctl.iter()
    }
    #[doc = "0x680..0x780 - CHDELAY"]
    #[inline(always)]
    pub const fn chdelay(&self, n: usize) -> &Chdelay {
        &self.chdelay[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x680..0x780 - CHDELAY"]
    #[inline(always)]
    pub fn chdelay_iter(&self) -> impl Iterator<Item = &Chdelay> {
        self.chdelay.iter()
    }
}
#[doc = "SEQEN (rw) register accessor: Enable sequencer register\n\nYou can [`read`](crate::Reg::read) this register and get [`seqen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqen`] module"]
#[doc(alias = "SEQEN")]
pub type Seqen = crate::Reg<seqen::SeqenSpec>;
#[doc = "Enable sequencer register"]
pub mod seqen;
#[doc = "SEQSYNC (rw) register accessor: Sequencer sync register\n\nYou can [`read`](crate::Reg::read) this register and get [`seqsync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqsync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seqsync`] module"]
#[doc(alias = "SEQSYNC")]
pub type Seqsync = crate::Reg<seqsync::SeqsyncSpec>;
#[doc = "Sequencer sync register"]
pub mod seqsync;
#[doc = "FSTAT (rw) register accessor: FIFO overflow status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fstat`] module"]
#[doc(alias = "FSTAT")]
pub type Fstat = crate::Reg<fstat::FstatSpec>;
#[doc = "FIFO overflow status register"]
pub mod fstat;
#[doc = "BSTAT (r) register accessor: Busy status register\n\nYou can [`read`](crate::Reg::read) this register and get [`bstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bstat`] module"]
#[doc(alias = "BSTAT")]
pub type Bstat = crate::Reg<bstat::BstatSpec>;
#[doc = "Busy status register"]
pub mod bstat;
#[doc = "DCTRIG (w) register accessor: Digital comparator output trigger status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctrig`] module"]
#[doc(alias = "DCTRIG")]
pub type Dctrig = crate::Reg<dctrig::DctrigSpec>;
#[doc = "Digital comparator output trigger status register"]
pub mod dctrig;
#[doc = "DCEV (w) register accessor: Digital comparator compare event status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcev::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcev`] module"]
#[doc(alias = "DCEV")]
pub type Dcev = crate::Reg<dcev::DcevSpec>;
#[doc = "Digital comparator compare event status register"]
pub mod dcev;
#[doc = "CICNT (rw) register accessor: Interrupt counter clear control\n\nYou can [`read`](crate::Reg::read) this register and get [`cicnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicnt`] module"]
#[doc(alias = "CICNT")]
pub type Cicnt = crate::Reg<cicnt::CicntSpec>;
#[doc = "Interrupt counter clear control"]
pub mod cicnt;
#[doc = "EMUX (rw) register accessor: Sequencer start event selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`emux::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emux::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emux`] module"]
#[doc(alias = "EMUX")]
pub type Emux = crate::Reg<emux::EmuxSpec>;
#[doc = "Sequencer start event selection register"]
pub mod emux;
#[doc = "RIS (r) register accessor: Raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status register"]
pub mod ris;
#[doc = "IM (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`] module"]
#[doc(alias = "IM")]
pub type Im = crate::Reg<im::ImSpec>;
#[doc = "Interrupt mask register"]
pub mod im;
#[doc = "MIS (r) register accessor: Masked interrupt status and clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status and clear register"]
pub mod mis;
#[doc = "IC (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ic`] module"]
#[doc(alias = "IC")]
pub type Ic = crate::Reg<ic::IcSpec>;
#[doc = "Interrupt clear register"]
pub mod ic;
#[doc = "SEQ"]
pub use self::seq::Seq;
#[doc = r"Cluster"]
#[doc = "SEQ"]
pub mod seq;
#[doc = "DC"]
pub use self::dc::Dc;
#[doc = r"Cluster"]
#[doc = "DC"]
pub mod dc;
#[doc = "ACTL"]
pub use self::actl::Actl;
#[doc = r"Cluster"]
#[doc = "ACTL"]
pub mod actl;
#[doc = "CHCTL"]
pub use self::chctl::Chctl;
#[doc = r"Cluster"]
#[doc = "CHCTL"]
pub mod chctl;
#[doc = "CHDELAY"]
pub use self::chdelay::Chdelay;
#[doc = r"Cluster"]
#[doc = "CHDELAY"]
pub mod chdelay;
