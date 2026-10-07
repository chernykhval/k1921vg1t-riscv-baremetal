#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_addr: [u8; 0x04],
    _reserved1: [u8; 0x0c],
    data: [Data; 4],
    _reserved2: [u8; 0x20],
    cmd: Cmd,
    stat: Stat,
    ctrl: Ctrl,
    _reserved5: [u8; 0xb4],
    ftcetr: Ftcetr,
    ftcprt: Ftcprt,
    ftcertr: Ftcertr,
    ftcprtr: Ftcprtr,
    _reserved9: [u8; 0x60],
    errmask: [Errmask; 4],
    fecccr: Fecccr,
    feccistat: Feccistat,
    _reserved12: [u8; 0x08],
    feccsec: Feccsec,
    feccdec: Feccdec,
    feccsea: Feccsea,
    feccdea: Feccdea,
}
impl RegisterBlock {
    #[doc = "0x00 - Address Register with expanded fields"]
    #[inline(always)]
    pub const fn expand_addr(&self) -> &ExpandAddr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Address Register for NVR_CFG access"]
    #[inline(always)]
    pub const fn nvrcfg_addr(&self) -> &NvrcfgAddr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Address Register for NVR access"]
    #[inline(always)]
    pub const fn nvr_addr(&self) -> &NvrAddr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Address Register"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x10..0x20 - DATA"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - DATA"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
    #[doc = "0x40 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x44 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x48 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x100 - Flash Timing Control register (Erase time)"]
    #[inline(always)]
    pub const fn ftcetr(&self) -> &Ftcetr {
        &self.ftcetr
    }
    #[doc = "0x104 - Flash Timing Control register (Program time)"]
    #[inline(always)]
    pub const fn ftcprt(&self) -> &Ftcprt {
        &self.ftcprt
    }
    #[doc = "0x108 - Flash Timing Control register (Erase Recovery time)"]
    #[inline(always)]
    pub const fn ftcertr(&self) -> &Ftcertr {
        &self.ftcertr
    }
    #[doc = "0x10c - Flash Timing Control register (Program Recovery time)"]
    #[inline(always)]
    pub const fn ftcprtr(&self) -> &Ftcprtr {
        &self.ftcprtr
    }
    #[doc = "0x170..0x180 - ERRMASK"]
    #[inline(always)]
    pub const fn errmask(&self, n: usize) -> &Errmask {
        &self.errmask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x170..0x180 - ERRMASK"]
    #[inline(always)]
    pub fn errmask_iter(&self) -> impl Iterator<Item = &Errmask> {
        self.errmask.iter()
    }
    #[doc = "0x180 - Flash ECC Control register"]
    #[inline(always)]
    pub const fn fecccr(&self) -> &Fecccr {
        &self.fecccr
    }
    #[doc = "0x184 - Flash ECC Interrupt Status register"]
    #[inline(always)]
    pub const fn feccistat(&self) -> &Feccistat {
        &self.feccistat
    }
    #[doc = "0x190 - Flash ECC Single Error Counter register"]
    #[inline(always)]
    pub const fn feccsec(&self) -> &Feccsec {
        &self.feccsec
    }
    #[doc = "0x194 - Flash ECC Double Error Counter register"]
    #[inline(always)]
    pub const fn feccdec(&self) -> &Feccdec {
        &self.feccdec
    }
    #[doc = "0x198 - Flash ECC Single Error Address register"]
    #[inline(always)]
    pub const fn feccsea(&self) -> &Feccsea {
        &self.feccsea
    }
    #[doc = "0x19c - Flash ECC Double Error Address register"]
    #[inline(always)]
    pub const fn feccdea(&self) -> &Feccdea {
        &self.feccdea
    }
}
#[doc = "ADDR (rw) register accessor: Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`] module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "Address Register"]
pub mod addr;
#[doc = "NVR_ADDR (rw) register accessor: Address Register for NVR access\n\nYou can [`read`](crate::Reg::read) this register and get [`nvr_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvr_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvr_addr`] module"]
#[doc(alias = "NVR_ADDR")]
pub type NvrAddr = crate::Reg<nvr_addr::NvrAddrSpec>;
#[doc = "Address Register for NVR access"]
pub mod nvr_addr;
#[doc = "NVRCFG_ADDR (rw) register accessor: Address Register for NVR_CFG access\n\nYou can [`read`](crate::Reg::read) this register and get [`nvrcfg_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nvrcfg_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nvrcfg_addr`] module"]
#[doc(alias = "NVRCFG_ADDR")]
pub type NvrcfgAddr = crate::Reg<nvrcfg_addr::NvrcfgAddrSpec>;
#[doc = "Address Register for NVR_CFG access"]
pub mod nvrcfg_addr;
#[doc = "EXPAND_ADDR (rw) register accessor: Address Register with expanded fields\n\nYou can [`read`](crate::Reg::read) this register and get [`expand_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`expand_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@expand_addr`] module"]
#[doc(alias = "EXPAND_ADDR")]
pub type ExpandAddr = crate::Reg<expand_addr::ExpandAddrSpec>;
#[doc = "Address Register with expanded fields"]
pub mod expand_addr;
#[doc = "DATA"]
pub use self::data::Data;
#[doc = r"Cluster"]
#[doc = "DATA"]
pub mod data;
#[doc = "CMD (rw) register accessor: Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "STAT (rw) register accessor: Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "FTCETR (rw) register accessor: Flash Timing Control register (Erase time)\n\nYou can [`read`](crate::Reg::read) this register and get [`ftcetr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftcetr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftcetr`] module"]
#[doc(alias = "FTCETR")]
pub type Ftcetr = crate::Reg<ftcetr::FtcetrSpec>;
#[doc = "Flash Timing Control register (Erase time)"]
pub mod ftcetr;
#[doc = "FTCPRT (rw) register accessor: Flash Timing Control register (Program time)\n\nYou can [`read`](crate::Reg::read) this register and get [`ftcprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftcprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftcprt`] module"]
#[doc(alias = "FTCPRT")]
pub type Ftcprt = crate::Reg<ftcprt::FtcprtSpec>;
#[doc = "Flash Timing Control register (Program time)"]
pub mod ftcprt;
#[doc = "FTCERTR (rw) register accessor: Flash Timing Control register (Erase Recovery time)\n\nYou can [`read`](crate::Reg::read) this register and get [`ftcertr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftcertr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftcertr`] module"]
#[doc(alias = "FTCERTR")]
pub type Ftcertr = crate::Reg<ftcertr::FtcertrSpec>;
#[doc = "Flash Timing Control register (Erase Recovery time)"]
pub mod ftcertr;
#[doc = "FTCPRTR (rw) register accessor: Flash Timing Control register (Program Recovery time)\n\nYou can [`read`](crate::Reg::read) this register and get [`ftcprtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftcprtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftcprtr`] module"]
#[doc(alias = "FTCPRTR")]
pub type Ftcprtr = crate::Reg<ftcprtr::FtcprtrSpec>;
#[doc = "Flash Timing Control register (Program Recovery time)"]
pub mod ftcprtr;
#[doc = "ERRMASK"]
pub use self::errmask::Errmask;
#[doc = r"Cluster"]
#[doc = "ERRMASK"]
pub mod errmask;
#[doc = "FECCCR (rw) register accessor: Flash ECC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fecccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fecccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fecccr`] module"]
#[doc(alias = "FECCCR")]
pub type Fecccr = crate::Reg<fecccr::FecccrSpec>;
#[doc = "Flash ECC Control register"]
pub mod fecccr;
#[doc = "FECCISTAT (rw) register accessor: Flash ECC Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccistat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccistat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feccistat`] module"]
#[doc(alias = "FECCISTAT")]
pub type Feccistat = crate::Reg<feccistat::FeccistatSpec>;
#[doc = "Flash ECC Interrupt Status register"]
pub mod feccistat;
#[doc = "FECCSEC (rw) register accessor: Flash ECC Single Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccsec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccsec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feccsec`] module"]
#[doc(alias = "FECCSEC")]
pub type Feccsec = crate::Reg<feccsec::FeccsecSpec>;
#[doc = "Flash ECC Single Error Counter register"]
pub mod feccsec;
#[doc = "FECCDEC (rw) register accessor: Flash ECC Double Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccdec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccdec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feccdec`] module"]
#[doc(alias = "FECCDEC")]
pub type Feccdec = crate::Reg<feccdec::FeccdecSpec>;
#[doc = "Flash ECC Double Error Counter register"]
pub mod feccdec;
#[doc = "FECCSEA (rw) register accessor: Flash ECC Single Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccsea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccsea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feccsea`] module"]
#[doc(alias = "FECCSEA")]
pub type Feccsea = crate::Reg<feccsea::FeccseaSpec>;
#[doc = "Flash ECC Single Error Address register"]
pub mod feccsea;
#[doc = "FECCDEA (rw) register accessor: Flash ECC Double Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccdea::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccdea::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@feccdea`] module"]
#[doc(alias = "FECCDEA")]
pub type Feccdea = crate::Reg<feccdea::FeccdeaSpec>;
#[doc = "Flash ECC Double Error Address register"]
pub mod feccdea;
