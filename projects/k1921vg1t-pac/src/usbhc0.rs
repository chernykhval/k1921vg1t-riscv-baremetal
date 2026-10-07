#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    caplength: Caplength,
    hcsparams: Hcsparams,
    hccparams: Hccparams,
    _reserved3: [u8; 0x08],
    ehccmd: Ehccmd,
    ehcsts: Ehcsts,
    ehcintr: Ehcintr,
    frindex: Frindex,
    ctrldsseg: Ctrldsseg,
    periodiclistbase: Periodiclistbase,
    asynclistaddr: Asynclistaddr,
    _reserved10: [u8; 0x24],
    configflag: Configflag,
    ehcportsc: Ehcportsc,
    _reserved12: [u8; 0x07a4],
    uhccmdsts: Uhccmdsts,
    uhciefrnum: Uhciefrnum,
    frbaseadd: Frbaseadd,
    sofmod: Sofmod,
    uhcportsc: Uhcportsc,
}
impl RegisterBlock {
    #[doc = "0x00 - USBHC Capabilities register"]
    #[inline(always)]
    pub const fn caplength(&self) -> &Caplength {
        &self.caplength
    }
    #[doc = "0x04 - USBHC Structural Parameters register"]
    #[inline(always)]
    pub const fn hcsparams(&self) -> &Hcsparams {
        &self.hcsparams
    }
    #[doc = "0x08 - USBHC Capability Parameters register"]
    #[inline(always)]
    pub const fn hccparams(&self) -> &Hccparams {
        &self.hccparams
    }
    #[doc = "0x14 - USBHC EHC Command register"]
    #[inline(always)]
    pub const fn ehccmd(&self) -> &Ehccmd {
        &self.ehccmd
    }
    #[doc = "0x18 - USBHC EHC Status register"]
    #[inline(always)]
    pub const fn ehcsts(&self) -> &Ehcsts {
        &self.ehcsts
    }
    #[doc = "0x1c - USBHC EHC Interrupt Enable register"]
    #[inline(always)]
    pub const fn ehcintr(&self) -> &Ehcintr {
        &self.ehcintr
    }
    #[doc = "0x20 - USBHC EHC Frame Index register"]
    #[inline(always)]
    pub const fn frindex(&self) -> &Frindex {
        &self.frindex
    }
    #[doc = "0x24 - USBHC EHC Control Data Structure Segment register"]
    #[inline(always)]
    pub const fn ctrldsseg(&self) -> &Ctrldsseg {
        &self.ctrldsseg
    }
    #[doc = "0x28 - USBHC EHC Periodic Frame List Base Address Register"]
    #[inline(always)]
    pub const fn periodiclistbase(&self) -> &Periodiclistbase {
        &self.periodiclistbase
    }
    #[doc = "0x2c - USBHC EHC Current Asynchronous List Address register"]
    #[inline(always)]
    pub const fn asynclistaddr(&self) -> &Asynclistaddr {
        &self.asynclistaddr
    }
    #[doc = "0x54 - USBHC EHC Configure Flag Register Bit Definitions register"]
    #[inline(always)]
    pub const fn configflag(&self) -> &Configflag {
        &self.configflag
    }
    #[doc = "0x58 - USBHC EHC Port Status and Control register"]
    #[inline(always)]
    pub const fn ehcportsc(&self) -> &Ehcportsc {
        &self.ehcportsc
    }
    #[doc = "0x800 - USBHC UHC Command and Status register"]
    #[inline(always)]
    pub const fn uhccmdsts(&self) -> &Uhccmdsts {
        &self.uhccmdsts
    }
    #[doc = "0x804 - USBHC UHC Interrupt Enable and Frame Number register"]
    #[inline(always)]
    pub const fn uhciefrnum(&self) -> &Uhciefrnum {
        &self.uhciefrnum
    }
    #[doc = "0x808 - USBHC UHC Frame List Base Address register"]
    #[inline(always)]
    pub const fn frbaseadd(&self) -> &Frbaseadd {
        &self.frbaseadd
    }
    #[doc = "0x80c - USBHC UHC Start of Frame Modify register"]
    #[inline(always)]
    pub const fn sofmod(&self) -> &Sofmod {
        &self.sofmod
    }
    #[doc = "0x810 - USBHC UHC Port Status and Control register"]
    #[inline(always)]
    pub const fn uhcportsc(&self) -> &Uhcportsc {
        &self.uhcportsc
    }
}
#[doc = "CAPLENGTH (r) register accessor: USBHC Capabilities register\n\nYou can [`read`](crate::Reg::read) this register and get [`caplength::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caplength`] module"]
#[doc(alias = "CAPLENGTH")]
pub type Caplength = crate::Reg<caplength::CaplengthSpec>;
#[doc = "USBHC Capabilities register"]
pub mod caplength;
#[doc = "HCSPARAMS (r) register accessor: USBHC Structural Parameters register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsparams::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsparams`] module"]
#[doc(alias = "HCSPARAMS")]
pub type Hcsparams = crate::Reg<hcsparams::HcsparamsSpec>;
#[doc = "USBHC Structural Parameters register"]
pub mod hcsparams;
#[doc = "HCCPARAMS (r) register accessor: USBHC Capability Parameters register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccparams::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccparams`] module"]
#[doc(alias = "HCCPARAMS")]
pub type Hccparams = crate::Reg<hccparams::HccparamsSpec>;
#[doc = "USBHC Capability Parameters register"]
pub mod hccparams;
#[doc = "EHCCMD (rw) register accessor: USBHC EHC Command register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehccmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehccmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehccmd`] module"]
#[doc(alias = "EHCCMD")]
pub type Ehccmd = crate::Reg<ehccmd::EhccmdSpec>;
#[doc = "USBHC EHC Command register"]
pub mod ehccmd;
#[doc = "EHCSTS (rw) register accessor: USBHC EHC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehcsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehcsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehcsts`] module"]
#[doc(alias = "EHCSTS")]
pub type Ehcsts = crate::Reg<ehcsts::EhcstsSpec>;
#[doc = "USBHC EHC Status register"]
pub mod ehcsts;
#[doc = "EHCINTR (rw) register accessor: USBHC EHC Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehcintr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehcintr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehcintr`] module"]
#[doc(alias = "EHCINTR")]
pub type Ehcintr = crate::Reg<ehcintr::EhcintrSpec>;
#[doc = "USBHC EHC Interrupt Enable register"]
pub mod ehcintr;
#[doc = "FRINDEX (rw) register accessor: USBHC EHC Frame Index register\n\nYou can [`read`](crate::Reg::read) this register and get [`frindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frindex`] module"]
#[doc(alias = "FRINDEX")]
pub type Frindex = crate::Reg<frindex::FrindexSpec>;
#[doc = "USBHC EHC Frame Index register"]
pub mod frindex;
#[doc = "CTRLDSSEG (rw) register accessor: USBHC EHC Control Data Structure Segment register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrldsseg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrldsseg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrldsseg`] module"]
#[doc(alias = "CTRLDSSEG")]
pub type Ctrldsseg = crate::Reg<ctrldsseg::CtrldssegSpec>;
#[doc = "USBHC EHC Control Data Structure Segment register"]
pub mod ctrldsseg;
#[doc = "PERIODICLISTBASE (rw) register accessor: USBHC EHC Periodic Frame List Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periodiclistbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`periodiclistbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periodiclistbase`] module"]
#[doc(alias = "PERIODICLISTBASE")]
pub type Periodiclistbase = crate::Reg<periodiclistbase::PeriodiclistbaseSpec>;
#[doc = "USBHC EHC Periodic Frame List Base Address Register"]
pub mod periodiclistbase;
#[doc = "ASYNCLISTADDR (rw) register accessor: USBHC EHC Current Asynchronous List Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`asynclistaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asynclistaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@asynclistaddr`] module"]
#[doc(alias = "ASYNCLISTADDR")]
pub type Asynclistaddr = crate::Reg<asynclistaddr::AsynclistaddrSpec>;
#[doc = "USBHC EHC Current Asynchronous List Address register"]
pub mod asynclistaddr;
#[doc = "CONFIGFLAG (rw) register accessor: USBHC EHC Configure Flag Register Bit Definitions register\n\nYou can [`read`](crate::Reg::read) this register and get [`configflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`configflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configflag`] module"]
#[doc(alias = "CONFIGFLAG")]
pub type Configflag = crate::Reg<configflag::ConfigflagSpec>;
#[doc = "USBHC EHC Configure Flag Register Bit Definitions register"]
pub mod configflag;
#[doc = "EHCPORTSC (rw) register accessor: USBHC EHC Port Status and Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ehcportsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ehcportsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ehcportsc`] module"]
#[doc(alias = "EHCPORTSC")]
pub type Ehcportsc = crate::Reg<ehcportsc::EhcportscSpec>;
#[doc = "USBHC EHC Port Status and Control register"]
pub mod ehcportsc;
#[doc = "UHCCMDSTS (rw) register accessor: USBHC UHC Command and Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhccmdsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhccmdsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhccmdsts`] module"]
#[doc(alias = "UHCCMDSTS")]
pub type Uhccmdsts = crate::Reg<uhccmdsts::UhccmdstsSpec>;
#[doc = "USBHC UHC Command and Status register"]
pub mod uhccmdsts;
#[doc = "UHCIEFRNUM (rw) register accessor: USBHC UHC Interrupt Enable and Frame Number register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhciefrnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhciefrnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhciefrnum`] module"]
#[doc(alias = "UHCIEFRNUM")]
pub type Uhciefrnum = crate::Reg<uhciefrnum::UhciefrnumSpec>;
#[doc = "USBHC UHC Interrupt Enable and Frame Number register"]
pub mod uhciefrnum;
#[doc = "FRBASEADD (rw) register accessor: USBHC UHC Frame List Base Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`frbaseadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frbaseadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frbaseadd`] module"]
#[doc(alias = "FRBASEADD")]
pub type Frbaseadd = crate::Reg<frbaseadd::FrbaseaddSpec>;
#[doc = "USBHC UHC Frame List Base Address register"]
pub mod frbaseadd;
#[doc = "SOFMOD (rw) register accessor: USBHC UHC Start of Frame Modify register\n\nYou can [`read`](crate::Reg::read) this register and get [`sofmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sofmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sofmod`] module"]
#[doc(alias = "SOFMOD")]
pub type Sofmod = crate::Reg<sofmod::SofmodSpec>;
#[doc = "USBHC UHC Start of Frame Modify register"]
pub mod sofmod;
#[doc = "UHCPORTSC (rw) register accessor: USBHC UHC Port Status and Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhcportsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhcportsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uhcportsc`] module"]
#[doc(alias = "UHCPORTSC")]
pub type Uhcportsc = crate::Reg<uhcportsc::UhcportscSpec>;
#[doc = "USBHC UHC Port Status and Control register"]
pub mod uhcportsc;
