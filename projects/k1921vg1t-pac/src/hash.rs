#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    datain: Datain,
    str: Str,
    imr: Imr,
    sr: Sr,
    _reserved5: [u8; 0x2c],
    hr: [Hr; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Data input register"]
    #[inline(always)]
    pub const fn datain(&self) -> &Datain {
        &self.datain
    }
    #[doc = "0x08 - Start register"]
    #[inline(always)]
    pub const fn str(&self) -> &Str {
        &self.str
    }
    #[doc = "0x0c - Interrupt enable register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x10 - Interrupt status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x40..0x60 - HR"]
    #[inline(always)]
    pub const fn hr(&self, n: usize) -> &Hr {
        &self.hr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x60 - HR"]
    #[inline(always)]
    pub fn hr_iter(&self) -> impl Iterator<Item = &Hr> {
        self.hr.iter()
    }
}
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
#[doc = "DATAIN (rw) register accessor: Data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`datain::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datain`] module"]
#[doc(alias = "DATAIN")]
pub type Datain = crate::Reg<datain::DatainSpec>;
#[doc = "Data input register"]
pub mod datain;
#[doc = "STR (rw) register accessor: Start register\n\nYou can [`read`](crate::Reg::read) this register and get [`str::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`str::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@str`] module"]
#[doc(alias = "STR")]
pub type Str = crate::Reg<str::StrSpec>;
#[doc = "Start register"]
pub mod str;
#[doc = "IMR (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt enable register"]
pub mod imr;
#[doc = "SR (rw) register accessor: Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Interrupt status register"]
pub mod sr;
#[doc = "HR"]
pub use self::hr::Hr;
#[doc = r"Cluster"]
#[doc = "HR"]
pub mod hr;
