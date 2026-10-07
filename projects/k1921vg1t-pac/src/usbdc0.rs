#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    epo: [Epo; 4],
    _reserved1: [u8; 0xc0],
    epi: [Epi; 4],
    _reserved2: [u8; 0xc0],
    gctrl: Gctrl,
    gstat: Gstat,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - EPO"]
    #[inline(always)]
    pub const fn epo(&self, n: usize) -> &Epo {
        &self.epo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - EPO"]
    #[inline(always)]
    pub fn epo_iter(&self) -> impl Iterator<Item = &Epo> {
        self.epo.iter()
    }
    #[doc = "0x100..0x140 - EPI"]
    #[inline(always)]
    pub const fn epi(&self, n: usize) -> &Epi {
        &self.epi[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - EPI"]
    #[inline(always)]
    pub fn epi_iter(&self) -> impl Iterator<Item = &Epi> {
        self.epi.iter()
    }
    #[doc = "0x200 - USBDC Control register"]
    #[inline(always)]
    pub const fn gctrl(&self) -> &Gctrl {
        &self.gctrl
    }
    #[doc = "0x204 - USBDC Status register"]
    #[inline(always)]
    pub const fn gstat(&self) -> &Gstat {
        &self.gstat
    }
}
#[doc = "EPO"]
pub use self::epo::Epo;
#[doc = r"Cluster"]
#[doc = "EPO"]
pub mod epo;
#[doc = "EPI"]
pub use self::epi::Epi;
#[doc = r"Cluster"]
#[doc = "EPI"]
pub mod epi;
#[doc = "GCTRL (rw) register accessor: USBDC Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`gctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gctrl`] module"]
#[doc(alias = "GCTRL")]
pub type Gctrl = crate::Reg<gctrl::GctrlSpec>;
#[doc = "USBDC Control register"]
pub mod gctrl;
#[doc = "GSTAT (rw) register accessor: USBDC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`gstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gstat`] module"]
#[doc(alias = "GSTAT")]
pub type Gstat = crate::Reg<gstat::GstatSpec>;
#[doc = "USBDC Status register"]
pub mod gstat;
