#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    time: Time,
    alarm1: Alarm1,
    alarm2: Alarm2,
    control: Control,
    status: Status,
    rctrim: Rctrim,
    sectrim: Sectrim,
    _reserved7: [u8; 0x04],
    gpr: [Gpr; 16],
}
impl RegisterBlock {
    #[doc = "0x00 - Time register"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x04 - Alarm 1 register"]
    #[inline(always)]
    pub const fn alarm1(&self) -> &Alarm1 {
        &self.alarm1
    }
    #[doc = "0x08 - Alarm 2 register"]
    #[inline(always)]
    pub const fn alarm2(&self) -> &Alarm2 {
        &self.alarm2
    }
    #[doc = "0x0c - Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x10 - Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - RC trim register"]
    #[inline(always)]
    pub const fn rctrim(&self) -> &Rctrim {
        &self.rctrim
    }
    #[doc = "0x18 - One second clock trim register"]
    #[inline(always)]
    pub const fn sectrim(&self) -> &Sectrim {
        &self.sectrim
    }
    #[doc = "0x20..0x60 - GPR"]
    #[inline(always)]
    pub const fn gpr(&self, n: usize) -> &Gpr {
        &self.gpr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x60 - GPR"]
    #[inline(always)]
    pub fn gpr_iter(&self) -> impl Iterator<Item = &Gpr> {
        self.gpr.iter()
    }
}
#[doc = "TIME (rw) register accessor: Time register\n\nYou can [`read`](crate::Reg::read) this register and get [`time::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`] module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "Time register"]
pub mod time;
#[doc = "ALARM1 (rw) register accessor: Alarm 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm1`] module"]
#[doc(alias = "ALARM1")]
pub type Alarm1 = crate::Reg<alarm1::Alarm1Spec>;
#[doc = "Alarm 1 register"]
pub mod alarm1;
#[doc = "ALARM2 (rw) register accessor: Alarm 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`alarm2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alarm2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alarm2`] module"]
#[doc(alias = "ALARM2")]
pub type Alarm2 = crate::Reg<alarm2::Alarm2Spec>;
#[doc = "Alarm 2 register"]
pub mod alarm2;
#[doc = "CONTROL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control register"]
pub mod control;
#[doc = "STATUS (rw) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register"]
pub mod status;
#[doc = "RCTRIM (rw) register accessor: RC trim register\n\nYou can [`read`](crate::Reg::read) this register and get [`rctrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rctrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rctrim`] module"]
#[doc(alias = "RCTRIM")]
pub type Rctrim = crate::Reg<rctrim::RctrimSpec>;
#[doc = "RC trim register"]
pub mod rctrim;
#[doc = "SECTRIM (rw) register accessor: One second clock trim register\n\nYou can [`read`](crate::Reg::read) this register and get [`sectrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sectrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sectrim`] module"]
#[doc(alias = "SECTRIM")]
pub type Sectrim = crate::Reg<sectrim::SectrimSpec>;
#[doc = "One second clock trim register"]
pub mod sectrim;
#[doc = "GPR"]
pub use self::gpr::Gpr;
#[doc = r"Cluster"]
#[doc = "GPR"]
pub mod gpr;
