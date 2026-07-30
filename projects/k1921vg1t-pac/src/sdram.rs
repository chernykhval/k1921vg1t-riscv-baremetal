#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timing: Timing,
    refresh: Refresh,
    mode: Mode,
    control: Control,
    status: Status,
    _reserved5: [u8; 0x04],
    reserved: Reserved,
}
impl RegisterBlock {
    #[doc = "0x00 - SDRAM Timing register"]
    #[inline(always)]
    pub const fn timing(&self) -> &Timing {
        &self.timing
    }
    #[doc = "0x04 - SDRAM Refresh register"]
    #[inline(always)]
    pub const fn refresh(&self) -> &Refresh {
        &self.refresh
    }
    #[doc = "0x08 - SDRAM Mode register"]
    #[inline(always)]
    pub const fn mode(&self) -> &Mode {
        &self.mode
    }
    #[doc = "0x0c - SDRAM Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x10 - SDRAM Status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
}
#[doc = "TIMING (rw) register accessor: SDRAM Timing register\n\nYou can [`read`](crate::Reg::read) this register and get [`timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing`] module"]
#[doc(alias = "TIMING")]
pub type Timing = crate::Reg<timing::TimingSpec>;
#[doc = "SDRAM Timing register"]
pub mod timing;
#[doc = "REFRESH (rw) register accessor: SDRAM Refresh register\n\nYou can [`read`](crate::Reg::read) this register and get [`refresh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refresh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refresh`] module"]
#[doc(alias = "REFRESH")]
pub type Refresh = crate::Reg<refresh::RefreshSpec>;
#[doc = "SDRAM Refresh register"]
pub mod refresh;
#[doc = "MODE (rw) register accessor: SDRAM Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode`] module"]
#[doc(alias = "MODE")]
pub type Mode = crate::Reg<mode::ModeSpec>;
#[doc = "SDRAM Mode register"]
pub mod mode;
#[doc = "CONTROL (rw) register accessor: SDRAM Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "SDRAM Control register"]
pub mod control;
#[doc = "STATUS (r) register accessor: SDRAM Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "SDRAM Status register"]
pub mod status;
#[doc = "Reserved (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`] module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = ""]
pub mod reserved;
