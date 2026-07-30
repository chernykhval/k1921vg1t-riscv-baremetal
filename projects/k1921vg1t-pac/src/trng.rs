#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    fifolev: Fifolev,
    fifothr: Fifothr,
    fifodep: Fifodep,
    key: [Key; 4],
    testdata: Testdata,
    reptstcutoff: Reptstcutoff,
    proptestcutoff: Proptestcutoff,
    lfsrseed: Lfsrseed,
    stat: Stat,
    warmperiod: Warmperiod,
    disosc: Disosc,
    _reserved12: [u8; 0x08],
    samperiod: Samperiod,
    _reserved13: [u8; 0x10],
    hwcfg: Hwcfg,
    cooldperiod: Cooldperiod,
    autocorrtestcutoff0: Autocorrtestcutoff0,
    autocorrtestcutoff1: Autocorrtestcutoff1,
    corrtestcutoff0: Corrtestcutoff0,
    corrtestcutoff1: Corrtestcutoff1,
    autocorrtestfailed: Autocorrtestfailed,
    corrtestfailed: Corrtestfailed,
    _reserved21: [u8; 0x04],
    hwversion: Hwversion,
    fifo: [Fifo; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - FIFO Level register"]
    #[inline(always)]
    pub const fn fifolev(&self) -> &Fifolev {
        &self.fifolev
    }
    #[doc = "0x08 - FIFO Threshold register"]
    #[inline(always)]
    pub const fn fifothr(&self) -> &Fifothr {
        &self.fifothr
    }
    #[doc = "0x0c - FIFO Depth register"]
    #[inline(always)]
    pub const fn fifodep(&self) -> &Fifodep {
        &self.fifodep
    }
    #[doc = "0x10..0x20 - KEY"]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &Key {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - KEY"]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &Key> {
        self.key.iter()
    }
    #[doc = "0x20 - Test data register"]
    #[inline(always)]
    pub const fn testdata(&self) -> &Testdata {
        &self.testdata
    }
    #[doc = "0x24 - Cut-off value for the repetition tests register"]
    #[inline(always)]
    pub const fn reptstcutoff(&self) -> &Reptstcutoff {
        &self.reptstcutoff
    }
    #[doc = "0x28 - Proportion test cut-off value register"]
    #[inline(always)]
    pub const fn proptestcutoff(&self) -> &Proptestcutoff {
        &self.proptestcutoff
    }
    #[doc = "0x2c - LFSR Seed register"]
    #[inline(always)]
    pub const fn lfsrseed(&self) -> &Lfsrseed {
        &self.lfsrseed
    }
    #[doc = "0x30 - Status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x34 - Warmup Period register"]
    #[inline(always)]
    pub const fn warmperiod(&self) -> &Warmperiod {
        &self.warmperiod
    }
    #[doc = "0x38 - Disable Oscillator registers"]
    #[inline(always)]
    pub const fn disosc(&self) -> &Disosc {
        &self.disosc
    }
    #[doc = "0x44 - Sampling Period register"]
    #[inline(always)]
    pub const fn samperiod(&self) -> &Samperiod {
        &self.samperiod
    }
    #[doc = "0x58 - Hardware Configuration register"]
    #[inline(always)]
    pub const fn hwcfg(&self) -> &Hwcfg {
        &self.hwcfg
    }
    #[doc = "0x5c - Cooldown Period register"]
    #[inline(always)]
    pub const fn cooldperiod(&self) -> &Cooldperiod {
        &self.cooldperiod
    }
    #[doc = "0x60 - AutoCorrTestCutoff register 0"]
    #[inline(always)]
    pub const fn autocorrtestcutoff0(&self) -> &Autocorrtestcutoff0 {
        &self.autocorrtestcutoff0
    }
    #[doc = "0x64 - AutoCorrTestCutoff register 1"]
    #[inline(always)]
    pub const fn autocorrtestcutoff1(&self) -> &Autocorrtestcutoff1 {
        &self.autocorrtestcutoff1
    }
    #[doc = "0x68 - CorrTestCutoff register 0"]
    #[inline(always)]
    pub const fn corrtestcutoff0(&self) -> &Corrtestcutoff0 {
        &self.corrtestcutoff0
    }
    #[doc = "0x6c - CorrTestCutoff register 1"]
    #[inline(always)]
    pub const fn corrtestcutoff1(&self) -> &Corrtestcutoff1 {
        &self.corrtestcutoff1
    }
    #[doc = "0x70 - AutoCorrTestFailed register"]
    #[inline(always)]
    pub const fn autocorrtestfailed(&self) -> &Autocorrtestfailed {
        &self.autocorrtestfailed
    }
    #[doc = "0x74 - CorrTestFailed register"]
    #[inline(always)]
    pub const fn corrtestfailed(&self) -> &Corrtestfailed {
        &self.corrtestfailed
    }
    #[doc = "0x7c - Hardware Version register"]
    #[inline(always)]
    pub const fn hwversion(&self) -> &Hwversion {
        &self.hwversion
    }
    #[doc = "0x80..0x100 - FIFO"]
    #[inline(always)]
    pub const fn fifo(&self, n: usize) -> &Fifo {
        &self.fifo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - FIFO"]
    #[inline(always)]
    pub fn fifo_iter(&self) -> impl Iterator<Item = &Fifo> {
        self.fifo.iter()
    }
}
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
#[doc = "FIFOLEV (rw) register accessor: FIFO Level register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifolev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifolev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifolev`] module"]
#[doc(alias = "FIFOLEV")]
pub type Fifolev = crate::Reg<fifolev::FifolevSpec>;
#[doc = "FIFO Level register"]
pub mod fifolev;
#[doc = "FIFOTHR (rw) register accessor: FIFO Threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifothr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifothr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifothr`] module"]
#[doc(alias = "FIFOTHR")]
pub type Fifothr = crate::Reg<fifothr::FifothrSpec>;
#[doc = "FIFO Threshold register"]
pub mod fifothr;
#[doc = "FIFODEP (r) register accessor: FIFO Depth register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifodep::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifodep`] module"]
#[doc(alias = "FIFODEP")]
pub type Fifodep = crate::Reg<fifodep::FifodepSpec>;
#[doc = "FIFO Depth register"]
pub mod fifodep;
#[doc = "KEY"]
pub use self::key::Key;
#[doc = r"Cluster"]
#[doc = "KEY"]
pub mod key;
#[doc = "TESTDATA (w) register accessor: Test data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testdata`] module"]
#[doc(alias = "TESTDATA")]
pub type Testdata = crate::Reg<testdata::TestdataSpec>;
#[doc = "Test data register"]
pub mod testdata;
#[doc = "REPTSTCUTOFF (rw) register accessor: Cut-off value for the repetition tests register\n\nYou can [`read`](crate::Reg::read) this register and get [`reptstcutoff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reptstcutoff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reptstcutoff`] module"]
#[doc(alias = "REPTSTCUTOFF")]
pub type Reptstcutoff = crate::Reg<reptstcutoff::ReptstcutoffSpec>;
#[doc = "Cut-off value for the repetition tests register"]
pub mod reptstcutoff;
#[doc = "PROPTESTCUTOFF (rw) register accessor: Proportion test cut-off value register\n\nYou can [`read`](crate::Reg::read) this register and get [`proptestcutoff::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`proptestcutoff::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@proptestcutoff`] module"]
#[doc(alias = "PROPTESTCUTOFF")]
pub type Proptestcutoff = crate::Reg<proptestcutoff::ProptestcutoffSpec>;
#[doc = "Proportion test cut-off value register"]
pub mod proptestcutoff;
#[doc = "LFSRSEED (rw) register accessor: LFSR Seed register\n\nYou can [`read`](crate::Reg::read) this register and get [`lfsrseed::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfsrseed::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfsrseed`] module"]
#[doc(alias = "LFSRSEED")]
pub type Lfsrseed = crate::Reg<lfsrseed::LfsrseedSpec>;
#[doc = "LFSR Seed register"]
pub mod lfsrseed;
#[doc = "STAT (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status register"]
pub mod stat;
#[doc = "WARMPERIOD (rw) register accessor: Warmup Period register\n\nYou can [`read`](crate::Reg::read) this register and get [`warmperiod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`warmperiod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@warmperiod`] module"]
#[doc(alias = "WARMPERIOD")]
pub type Warmperiod = crate::Reg<warmperiod::WarmperiodSpec>;
#[doc = "Warmup Period register"]
pub mod warmperiod;
#[doc = "DISOSC (rw) register accessor: Disable Oscillator registers\n\nYou can [`read`](crate::Reg::read) this register and get [`disosc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disosc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disosc`] module"]
#[doc(alias = "DISOSC")]
pub type Disosc = crate::Reg<disosc::DisoscSpec>;
#[doc = "Disable Oscillator registers"]
pub mod disosc;
#[doc = "SAMPERIOD (rw) register accessor: Sampling Period register\n\nYou can [`read`](crate::Reg::read) this register and get [`samperiod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samperiod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@samperiod`] module"]
#[doc(alias = "SAMPERIOD")]
pub type Samperiod = crate::Reg<samperiod::SamperiodSpec>;
#[doc = "Sampling Period register"]
pub mod samperiod;
#[doc = "HWCFG (r) register accessor: Hardware Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwcfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfg`] module"]
#[doc(alias = "HWCFG")]
pub type Hwcfg = crate::Reg<hwcfg::HwcfgSpec>;
#[doc = "Hardware Configuration register"]
pub mod hwcfg;
#[doc = "COOLDPERIOD (rw) register accessor: Cooldown Period register\n\nYou can [`read`](crate::Reg::read) this register and get [`cooldperiod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cooldperiod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cooldperiod`] module"]
#[doc(alias = "COOLDPERIOD")]
pub type Cooldperiod = crate::Reg<cooldperiod::CooldperiodSpec>;
#[doc = "Cooldown Period register"]
pub mod cooldperiod;
#[doc = "AUTOCORRTESTCUTOFF0 (rw) register accessor: AutoCorrTestCutoff register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorrtestcutoff0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorrtestcutoff0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocorrtestcutoff0`] module"]
#[doc(alias = "AUTOCORRTESTCUTOFF0")]
pub type Autocorrtestcutoff0 = crate::Reg<autocorrtestcutoff0::Autocorrtestcutoff0Spec>;
#[doc = "AutoCorrTestCutoff register 0"]
pub mod autocorrtestcutoff0;
#[doc = "AUTOCORRTESTCUTOFF1 (rw) register accessor: AutoCorrTestCutoff register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorrtestcutoff1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`autocorrtestcutoff1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocorrtestcutoff1`] module"]
#[doc(alias = "AUTOCORRTESTCUTOFF1")]
pub type Autocorrtestcutoff1 = crate::Reg<autocorrtestcutoff1::Autocorrtestcutoff1Spec>;
#[doc = "AutoCorrTestCutoff register 1"]
pub mod autocorrtestcutoff1;
#[doc = "CORRTESTCUTOFF0 (rw) register accessor: CorrTestCutoff register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`corrtestcutoff0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corrtestcutoff0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@corrtestcutoff0`] module"]
#[doc(alias = "CORRTESTCUTOFF0")]
pub type Corrtestcutoff0 = crate::Reg<corrtestcutoff0::Corrtestcutoff0Spec>;
#[doc = "CorrTestCutoff register 0"]
pub mod corrtestcutoff0;
#[doc = "CORRTESTCUTOFF1 (rw) register accessor: CorrTestCutoff register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`corrtestcutoff1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`corrtestcutoff1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@corrtestcutoff1`] module"]
#[doc(alias = "CORRTESTCUTOFF1")]
pub type Corrtestcutoff1 = crate::Reg<corrtestcutoff1::Corrtestcutoff1Spec>;
#[doc = "CorrTestCutoff register 1"]
pub mod corrtestcutoff1;
#[doc = "AUTOCORRTESTFAILED (r) register accessor: AutoCorrTestFailed register\n\nYou can [`read`](crate::Reg::read) this register and get [`autocorrtestfailed::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@autocorrtestfailed`] module"]
#[doc(alias = "AUTOCORRTESTFAILED")]
pub type Autocorrtestfailed = crate::Reg<autocorrtestfailed::AutocorrtestfailedSpec>;
#[doc = "AutoCorrTestFailed register"]
pub mod autocorrtestfailed;
#[doc = "CORRTESTFAILED (r) register accessor: CorrTestFailed register\n\nYou can [`read`](crate::Reg::read) this register and get [`corrtestfailed::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@corrtestfailed`] module"]
#[doc(alias = "CORRTESTFAILED")]
pub type Corrtestfailed = crate::Reg<corrtestfailed::CorrtestfailedSpec>;
#[doc = "CorrTestFailed register"]
pub mod corrtestfailed;
#[doc = "HWVERSION (r) register accessor: Hardware Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwversion`] module"]
#[doc(alias = "HWVERSION")]
pub type Hwversion = crate::Reg<hwversion::HwversionSpec>;
#[doc = "Hardware Version register"]
pub mod hwversion;
#[doc = "FIFO"]
pub use self::fifo::Fifo;
#[doc = r"Cluster"]
#[doc = "FIFO"]
pub mod fifo;
