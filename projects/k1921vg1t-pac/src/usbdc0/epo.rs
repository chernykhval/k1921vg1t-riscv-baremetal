#[repr(C)]
#[doc = "EPO"]
#[doc(alias = "EPO")]
pub struct Epo {
    ctrl: Ctrl,
    dmactrl: Dmactrl,
    dmadesc: Dmadesc,
    stat: Stat,
}
impl Epo {
    #[doc = "0x00 - OUT Endpoint control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - OUT DMA control register"]
    #[inline(always)]
    pub const fn dmactrl(&self) -> &Dmactrl {
        &self.dmactrl
    }
    #[doc = "0x08 - OUT Descriptor address register"]
    #[inline(always)]
    pub const fn dmadesc(&self) -> &Dmadesc {
        &self.dmadesc
    }
    #[doc = "0x0c - OUT Endpoint status register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
#[doc = "CTRL (rw) register accessor: OUT Endpoint control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "OUT Endpoint control register"]
pub mod ctrl;
#[doc = "DMACTRL (rw) register accessor: OUT DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactrl`] module"]
#[doc(alias = "DMACTRL")]
pub type Dmactrl = crate::Reg<dmactrl::DmactrlSpec>;
#[doc = "OUT DMA control register"]
pub mod dmactrl;
#[doc = "DMADESC (rw) register accessor: OUT Descriptor address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmadesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmadesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmadesc`] module"]
#[doc(alias = "DMADESC")]
pub type Dmadesc = crate::Reg<dmadesc::DmadescSpec>;
#[doc = "OUT Descriptor address register"]
pub mod dmadesc;
#[doc = "STAT (rw) register accessor: OUT Endpoint status register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`] module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "OUT Endpoint status register"]
pub mod stat;
