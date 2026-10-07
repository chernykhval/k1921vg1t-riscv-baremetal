#[repr(C)]
#[doc = "PLL"]
#[doc(alias = "PLL")]
pub struct Pll {
    cfg: Cfg,
    div: Div,
    frac: Frac,
    mod_: Mod,
}
impl Pll {
    #[doc = "0x00 - PLL Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x04 - PLL DIV Configuration Register"]
    #[inline(always)]
    pub const fn div(&self) -> &Div {
        &self.div
    }
    #[doc = "0x08 - PLL FRAC Configuration Register"]
    #[inline(always)]
    pub const fn frac(&self) -> &Frac {
        &self.frac
    }
    #[doc = "0x0c - PLL MOD Configuration Register"]
    #[inline(always)]
    pub const fn mod_(&self) -> &Mod {
        &self.mod_
    }
}
#[doc = "CFG (rw) register accessor: PLL Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "PLL Configuration Register"]
pub mod cfg;
#[doc = "DIV (rw) register accessor: PLL DIV Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`div::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`div::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@div`] module"]
#[doc(alias = "DIV")]
pub type Div = crate::Reg<div::DivSpec>;
#[doc = "PLL DIV Configuration Register"]
pub mod div;
#[doc = "FRAC (rw) register accessor: PLL FRAC Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frac`] module"]
#[doc(alias = "FRAC")]
pub type Frac = crate::Reg<frac::FracSpec>;
#[doc = "PLL FRAC Configuration Register"]
pub mod frac;
#[doc = "MOD (rw) register accessor: PLL MOD Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod_`] module"]
#[doc(alias = "MOD")]
pub type Mod = crate::Reg<mod_::ModSpec>;
#[doc = "PLL MOD Configuration Register"]
pub mod mod_;
