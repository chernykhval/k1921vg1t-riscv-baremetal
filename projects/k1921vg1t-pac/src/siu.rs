#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    chipid: Chipid,
    servctl: Servctl,
    cpestat: Cpestat,
    lock: Lock,
    core1_clken: Core1Clken,
    core1_rstdis: Core1Rstdis,
    core1_rvect: Core1Rvect,
    core1_conf: Core1Conf,
    ssramscr: Ssramscr,
    ssrammcr: Ssrammcr,
    psramsmcr: Psramsmcr,
    ccsmcr: Ccsmcr,
    inten: Inten,
    ris: Ris,
    mis: Mis,
    iclr: Iclr,
    rcctrl: Rcctrl,
    ldoctrl: Ldoctrl,
    bgpctrl: Bgpctrl,
    tsensctrl: Tsensctrl,
    tmrcfg: Tmrcfg,
    tmren: Tmren,
    pwmen: Pwmen,
    pwmsync: Pwmsync,
    emmuxctrl: Emmuxctrl,
    timeout: Timeout,
    frsten: Frsten,
    fstat: Fstat,
    tmrmux: [Tmrmux; 16],
    _reserved29: [u8; 0x50],
    nodestest: Nodestest,
    _reserved30: [u8; 0x0c],
    node0ecnt: Node0ecnt,
    node0ctrl: Node0ctrl,
    node1ecnt: Node1ecnt,
    node1ctrl: Node1ctrl,
    node2ecnt: Node2ecnt,
    node2ctrl: Node2ctrl,
    node3ecnt: Node3ecnt,
    node3ctrl: Node3ctrl,
    node4ecnt: Node4ecnt,
    node4ctrl: Node4ctrl,
    _reserved40: [u8; 0x0c],
    reserved: Reserved,
}
impl RegisterBlock {
    #[doc = "0x00 - Chip identifier"]
    #[inline(always)]
    pub const fn chipid(&self) -> &Chipid {
        &self.chipid
    }
    #[doc = "0x04 - Service mode register"]
    #[inline(always)]
    pub const fn servctl(&self) -> &Servctl {
        &self.servctl
    }
    #[doc = "0x08 - CPE status register"]
    #[inline(always)]
    pub const fn cpestat(&self) -> &Cpestat {
        &self.cpestat
    }
    #[doc = "0x0c - Lock configuration register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x10 - Core 1 clock enable register"]
    #[inline(always)]
    pub const fn core1_clken(&self) -> &Core1Clken {
        &self.core1_clken
    }
    #[doc = "0x14 - Core 1 reset disable register"]
    #[inline(always)]
    pub const fn core1_rstdis(&self) -> &Core1Rstdis {
        &self.core1_rstdis
    }
    #[doc = "0x18 - Core 1 reset vector register"]
    #[inline(always)]
    pub const fn core1_rvect(&self) -> &Core1Rvect {
        &self.core1_rvect
    }
    #[doc = "0x1c - Core 1 configuration register"]
    #[inline(always)]
    pub const fn core1_conf(&self) -> &Core1Conf {
        &self.core1_conf
    }
    #[doc = "0x20 - System SRAM sleep control register"]
    #[inline(always)]
    pub const fn ssramscr(&self) -> &Ssramscr {
        &self.ssramscr
    }
    #[doc = "0x24 - System SRAM margin control register"]
    #[inline(always)]
    pub const fn ssrammcr(&self) -> &Ssrammcr {
        &self.ssrammcr
    }
    #[doc = "0x28 - Periperals SRAM sleep and margin control register"]
    #[inline(always)]
    pub const fn psramsmcr(&self) -> &Psramsmcr {
        &self.psramsmcr
    }
    #[doc = "0x2c - Core caches sleep and margin control register"]
    #[inline(always)]
    pub const fn ccsmcr(&self) -> &Ccsmcr {
        &self.ccsmcr
    }
    #[doc = "0x30 - Interrupt enable register"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x34 - Raw interrupt status register"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x38 - Masked interrupt status register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x3c - Interrupt clear register"]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x40 - RC generator control register"]
    #[inline(always)]
    pub const fn rcctrl(&self) -> &Rcctrl {
        &self.rcctrl
    }
    #[doc = "0x44 - LDO control register"]
    #[inline(always)]
    pub const fn ldoctrl(&self) -> &Ldoctrl {
        &self.ldoctrl
    }
    #[doc = "0x48 - Bandgap control register"]
    #[inline(always)]
    pub const fn bgpctrl(&self) -> &Bgpctrl {
        &self.bgpctrl
    }
    #[doc = "0x4c - Tempreture sensor control register"]
    #[inline(always)]
    pub const fn tsensctrl(&self) -> &Tsensctrl {
        &self.tsensctrl
    }
    #[doc = "0x50 - Timers external clock config"]
    #[inline(always)]
    pub const fn tmrcfg(&self) -> &Tmrcfg {
        &self.tmrcfg
    }
    #[doc = "0x54 - Timers counting enable register"]
    #[inline(always)]
    pub const fn tmren(&self) -> &Tmren {
        &self.tmren
    }
    #[doc = "0x58 - PWM counting enable register"]
    #[inline(always)]
    pub const fn pwmen(&self) -> &Pwmen {
        &self.pwmen
    }
    #[doc = "0x5c - Sync PWM register"]
    #[inline(always)]
    pub const fn pwmsync(&self) -> &Pwmsync {
        &self.pwmsync
    }
    #[doc = "0x60 - Extrernal memory multiplexor control register"]
    #[inline(always)]
    pub const fn emmuxctrl(&self) -> &Emmuxctrl {
        &self.emmuxctrl
    }
    #[doc = "0x64 - Timeout register"]
    #[inline(always)]
    pub const fn timeout(&self) -> &Timeout {
        &self.timeout
    }
    #[doc = "0x68 - Fault reset enable register"]
    #[inline(always)]
    pub const fn frsten(&self) -> &Frsten {
        &self.frsten
    }
    #[doc = "0x6c - Fault status regsiter"]
    #[inline(always)]
    pub const fn fstat(&self) -> &Fstat {
        &self.fstat
    }
    #[doc = "0x70..0xb0 - TMRMUX"]
    #[inline(always)]
    pub const fn tmrmux(&self, n: usize) -> &Tmrmux {
        &self.tmrmux[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0xb0 - TMRMUX"]
    #[inline(always)]
    pub fn tmrmux_iter(&self) -> impl Iterator<Item = &Tmrmux> {
        self.tmrmux.iter()
    }
    #[doc = "0x100 - Nodes selftest start and status register"]
    #[inline(always)]
    pub const fn nodestest(&self) -> &Nodestest {
        &self.nodestest
    }
    #[doc = "0x110 - Node 0 error counter register"]
    #[inline(always)]
    pub const fn node0ecnt(&self) -> &Node0ecnt {
        &self.node0ecnt
    }
    #[doc = "0x114 - Node 0 control register"]
    #[inline(always)]
    pub const fn node0ctrl(&self) -> &Node0ctrl {
        &self.node0ctrl
    }
    #[doc = "0x118 - Node 1 error counter register"]
    #[inline(always)]
    pub const fn node1ecnt(&self) -> &Node1ecnt {
        &self.node1ecnt
    }
    #[doc = "0x11c - Node 1 control register"]
    #[inline(always)]
    pub const fn node1ctrl(&self) -> &Node1ctrl {
        &self.node1ctrl
    }
    #[doc = "0x120 - Node 2 error counter register"]
    #[inline(always)]
    pub const fn node2ecnt(&self) -> &Node2ecnt {
        &self.node2ecnt
    }
    #[doc = "0x124 - Node 2 control register"]
    #[inline(always)]
    pub const fn node2ctrl(&self) -> &Node2ctrl {
        &self.node2ctrl
    }
    #[doc = "0x128 - Node 3 error counter register"]
    #[inline(always)]
    pub const fn node3ecnt(&self) -> &Node3ecnt {
        &self.node3ecnt
    }
    #[doc = "0x12c - Node 3 control register"]
    #[inline(always)]
    pub const fn node3ctrl(&self) -> &Node3ctrl {
        &self.node3ctrl
    }
    #[doc = "0x130 - Node 4 error counter register"]
    #[inline(always)]
    pub const fn node4ecnt(&self) -> &Node4ecnt {
        &self.node4ecnt
    }
    #[doc = "0x134 - Node 4 control register"]
    #[inline(always)]
    pub const fn node4ctrl(&self) -> &Node4ctrl {
        &self.node4ctrl
    }
    #[doc = "0x144 - "]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
}
#[doc = "CHIPID (r) register accessor: Chip identifier\n\nYou can [`read`](crate::Reg::read) this register and get [`chipid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chipid`] module"]
#[doc(alias = "CHIPID")]
pub type Chipid = crate::Reg<chipid::ChipidSpec>;
#[doc = "Chip identifier"]
pub mod chipid;
#[doc = "SERVCTL (rw) register accessor: Service mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`servctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`servctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@servctl`] module"]
#[doc(alias = "SERVCTL")]
pub type Servctl = crate::Reg<servctl::ServctlSpec>;
#[doc = "Service mode register"]
pub mod servctl;
#[doc = "CPESTAT (r) register accessor: CPE status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpestat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpestat`] module"]
#[doc(alias = "CPESTAT")]
pub type Cpestat = crate::Reg<cpestat::CpestatSpec>;
#[doc = "CPE status register"]
pub mod cpestat;
#[doc = "LOCK (rw) register accessor: Lock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Lock configuration register"]
pub mod lock;
#[doc = "CORE1_CLKEN (w) register accessor: Core 1 clock enable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_clken::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_clken`] module"]
#[doc(alias = "CORE1_CLKEN")]
pub type Core1Clken = crate::Reg<core1_clken::Core1ClkenSpec>;
#[doc = "Core 1 clock enable register"]
pub mod core1_clken;
#[doc = "CORE1_RSTDIS (w) register accessor: Core 1 reset disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_rstdis::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_rstdis`] module"]
#[doc(alias = "CORE1_RSTDIS")]
pub type Core1Rstdis = crate::Reg<core1_rstdis::Core1RstdisSpec>;
#[doc = "Core 1 reset disable register"]
pub mod core1_rstdis;
#[doc = "CORE1_RVECT (rw) register accessor: Core 1 reset vector register\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_rvect::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_rvect::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_rvect`] module"]
#[doc(alias = "CORE1_RVECT")]
pub type Core1Rvect = crate::Reg<core1_rvect::Core1RvectSpec>;
#[doc = "Core 1 reset vector register"]
pub mod core1_rvect;
#[doc = "CORE1_CONF (rw) register accessor: Core 1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`core1_conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core1_conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core1_conf`] module"]
#[doc(alias = "CORE1_CONF")]
pub type Core1Conf = crate::Reg<core1_conf::Core1ConfSpec>;
#[doc = "Core 1 configuration register"]
pub mod core1_conf;
#[doc = "SSRAMSCR (rw) register accessor: System SRAM sleep control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssramscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssramscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssramscr`] module"]
#[doc(alias = "SSRAMSCR")]
pub type Ssramscr = crate::Reg<ssramscr::SsramscrSpec>;
#[doc = "System SRAM sleep control register"]
pub mod ssramscr;
#[doc = "SSRAMMCR (rw) register accessor: System SRAM margin control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrammcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrammcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssrammcr`] module"]
#[doc(alias = "SSRAMMCR")]
pub type Ssrammcr = crate::Reg<ssrammcr::SsrammcrSpec>;
#[doc = "System SRAM margin control register"]
pub mod ssrammcr;
#[doc = "PSRAMSMCR (rw) register accessor: Periperals SRAM sleep and margin control register\n\nYou can [`read`](crate::Reg::read) this register and get [`psramsmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psramsmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psramsmcr`] module"]
#[doc(alias = "PSRAMSMCR")]
pub type Psramsmcr = crate::Reg<psramsmcr::PsramsmcrSpec>;
#[doc = "Periperals SRAM sleep and margin control register"]
pub mod psramsmcr;
#[doc = "CCSMCR (rw) register accessor: Core caches sleep and margin control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccsmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccsmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccsmcr`] module"]
#[doc(alias = "CCSMCR")]
pub type Ccsmcr = crate::Reg<ccsmcr::CcsmcrSpec>;
#[doc = "Core caches sleep and margin control register"]
pub mod ccsmcr;
#[doc = "INTEN (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt enable register"]
pub mod inten;
#[doc = "RIS (r) register accessor: Raw interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status register"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status register"]
pub mod mis;
#[doc = "ICLR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`] module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt clear register"]
pub mod iclr;
#[doc = "RCCTRL (rw) register accessor: RC generator control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcctrl`] module"]
#[doc(alias = "RCCTRL")]
pub type Rcctrl = crate::Reg<rcctrl::RcctrlSpec>;
#[doc = "RC generator control register"]
pub mod rcctrl;
#[doc = "LDOCTRL (rw) register accessor: LDO control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ldoctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldoctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldoctrl`] module"]
#[doc(alias = "LDOCTRL")]
pub type Ldoctrl = crate::Reg<ldoctrl::LdoctrlSpec>;
#[doc = "LDO control register"]
pub mod ldoctrl;
#[doc = "BGPCTRL (rw) register accessor: Bandgap control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgpctrl`] module"]
#[doc(alias = "BGPCTRL")]
pub type Bgpctrl = crate::Reg<bgpctrl::BgpctrlSpec>;
#[doc = "Bandgap control register"]
pub mod bgpctrl;
#[doc = "TSENSCTRL (rw) register accessor: Tempreture sensor control register\n\nYou can [`read`](crate::Reg::read) this register and get [`tsensctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsensctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsensctrl`] module"]
#[doc(alias = "TSENSCTRL")]
pub type Tsensctrl = crate::Reg<tsensctrl::TsensctrlSpec>;
#[doc = "Tempreture sensor control register"]
pub mod tsensctrl;
#[doc = "TMRCFG (rw) register accessor: Timers external clock config\n\nYou can [`read`](crate::Reg::read) this register and get [`tmrcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmrcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmrcfg`] module"]
#[doc(alias = "TMRCFG")]
pub type Tmrcfg = crate::Reg<tmrcfg::TmrcfgSpec>;
#[doc = "Timers external clock config"]
pub mod tmrcfg;
#[doc = "TMREN (rw) register accessor: Timers counting enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmren::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmren::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmren`] module"]
#[doc(alias = "TMREN")]
pub type Tmren = crate::Reg<tmren::TmrenSpec>;
#[doc = "Timers counting enable register"]
pub mod tmren;
#[doc = "PWMEN (rw) register accessor: PWM counting enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmen`] module"]
#[doc(alias = "PWMEN")]
pub type Pwmen = crate::Reg<pwmen::PwmenSpec>;
#[doc = "PWM counting enable register"]
pub mod pwmen;
#[doc = "PWMSYNC (rw) register accessor: Sync PWM register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmsync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmsync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwmsync`] module"]
#[doc(alias = "PWMSYNC")]
pub type Pwmsync = crate::Reg<pwmsync::PwmsyncSpec>;
#[doc = "Sync PWM register"]
pub mod pwmsync;
#[doc = "EMMUXCTRL (rw) register accessor: Extrernal memory multiplexor control register\n\nYou can [`read`](crate::Reg::read) this register and get [`emmuxctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emmuxctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emmuxctrl`] module"]
#[doc(alias = "EMMUXCTRL")]
pub type Emmuxctrl = crate::Reg<emmuxctrl::EmmuxctrlSpec>;
#[doc = "Extrernal memory multiplexor control register"]
pub mod emmuxctrl;
#[doc = "TIMEOUT (rw) register accessor: Timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timeout`] module"]
#[doc(alias = "TIMEOUT")]
pub type Timeout = crate::Reg<timeout::TimeoutSpec>;
#[doc = "Timeout register"]
pub mod timeout;
#[doc = "FRSTEN (rw) register accessor: Fault reset enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`frsten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frsten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frsten`] module"]
#[doc(alias = "FRSTEN")]
pub type Frsten = crate::Reg<frsten::FrstenSpec>;
#[doc = "Fault reset enable register"]
pub mod frsten;
#[doc = "FSTAT (rw) register accessor: Fault status regsiter\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fstat`] module"]
#[doc(alias = "FSTAT")]
pub type Fstat = crate::Reg<fstat::FstatSpec>;
#[doc = "Fault status regsiter"]
pub mod fstat;
#[doc = "TMRMUX"]
pub use self::tmrmux::Tmrmux;
#[doc = r"Cluster"]
#[doc = "TMRMUX"]
pub mod tmrmux;
#[doc = "NODESTEST (rw) register accessor: Nodes selftest start and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`nodestest::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nodestest::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nodestest`] module"]
#[doc(alias = "NODESTEST")]
pub type Nodestest = crate::Reg<nodestest::NodestestSpec>;
#[doc = "Nodes selftest start and status register"]
pub mod nodestest;
#[doc = "NODE0ECNT (r) register accessor: Node 0 error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`node0ecnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node0ecnt`] module"]
#[doc(alias = "NODE0ECNT")]
pub type Node0ecnt = crate::Reg<node0ecnt::Node0ecntSpec>;
#[doc = "Node 0 error counter register"]
pub mod node0ecnt;
#[doc = "NODE0CTRL (rw) register accessor: Node 0 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`node0ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`node0ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node0ctrl`] module"]
#[doc(alias = "NODE0CTRL")]
pub type Node0ctrl = crate::Reg<node0ctrl::Node0ctrlSpec>;
#[doc = "Node 0 control register"]
pub mod node0ctrl;
#[doc = "NODE1ECNT (r) register accessor: Node 1 error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`node1ecnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node1ecnt`] module"]
#[doc(alias = "NODE1ECNT")]
pub type Node1ecnt = crate::Reg<node1ecnt::Node1ecntSpec>;
#[doc = "Node 1 error counter register"]
pub mod node1ecnt;
#[doc = "NODE1CTRL (rw) register accessor: Node 1 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`node1ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`node1ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node1ctrl`] module"]
#[doc(alias = "NODE1CTRL")]
pub type Node1ctrl = crate::Reg<node1ctrl::Node1ctrlSpec>;
#[doc = "Node 1 control register"]
pub mod node1ctrl;
#[doc = "NODE2ECNT (r) register accessor: Node 2 error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`node2ecnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node2ecnt`] module"]
#[doc(alias = "NODE2ECNT")]
pub type Node2ecnt = crate::Reg<node2ecnt::Node2ecntSpec>;
#[doc = "Node 2 error counter register"]
pub mod node2ecnt;
#[doc = "NODE2CTRL (rw) register accessor: Node 2 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`node2ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`node2ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node2ctrl`] module"]
#[doc(alias = "NODE2CTRL")]
pub type Node2ctrl = crate::Reg<node2ctrl::Node2ctrlSpec>;
#[doc = "Node 2 control register"]
pub mod node2ctrl;
#[doc = "NODE3ECNT (r) register accessor: Node 3 error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`node3ecnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node3ecnt`] module"]
#[doc(alias = "NODE3ECNT")]
pub type Node3ecnt = crate::Reg<node3ecnt::Node3ecntSpec>;
#[doc = "Node 3 error counter register"]
pub mod node3ecnt;
#[doc = "NODE3CTRL (rw) register accessor: Node 3 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`node3ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`node3ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node3ctrl`] module"]
#[doc(alias = "NODE3CTRL")]
pub type Node3ctrl = crate::Reg<node3ctrl::Node3ctrlSpec>;
#[doc = "Node 3 control register"]
pub mod node3ctrl;
#[doc = "NODE4ECNT (r) register accessor: Node 4 error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`node4ecnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node4ecnt`] module"]
#[doc(alias = "NODE4ECNT")]
pub type Node4ecnt = crate::Reg<node4ecnt::Node4ecntSpec>;
#[doc = "Node 4 error counter register"]
pub mod node4ecnt;
#[doc = "NODE4CTRL (rw) register accessor: Node 4 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`node4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`node4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@node4ctrl`] module"]
#[doc(alias = "NODE4CTRL")]
pub type Node4ctrl = crate::Reg<node4ctrl::Node4ctrlSpec>;
#[doc = "Node 4 control register"]
pub mod node4ctrl;
#[doc = "Reserved (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`] module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = ""]
pub mod reserved;
