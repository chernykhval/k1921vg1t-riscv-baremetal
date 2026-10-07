#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: Dr,
    post: Post,
    cr: Cr,
    init: Init,
    pol: Pol,
}
impl RegisterBlock {
    #[doc = "0x00 - CRC Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x04 - CRC post register"]
    #[inline(always)]
    pub const fn post(&self) -> &Post {
        &self.post
    }
    #[doc = "0x08 - CRC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x0c - CRC initial value register"]
    #[inline(always)]
    pub const fn init(&self) -> &Init {
        &self.init
    }
    #[doc = "0x10 - CRC polynomial register"]
    #[inline(always)]
    pub const fn pol(&self) -> &Pol {
        &self.pol
    }
}
#[doc = "DR (rw) register accessor: CRC Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "CRC Data register"]
pub mod dr;
#[doc = "POST (r) register accessor: CRC post register\n\nYou can [`read`](crate::Reg::read) this register and get [`post::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@post`] module"]
#[doc(alias = "POST")]
pub type Post = crate::Reg<post::PostSpec>;
#[doc = "CRC post register"]
pub mod post;
#[doc = "CR (rw) register accessor: CRC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "CRC control register"]
pub mod cr;
#[doc = "INIT (rw) register accessor: CRC initial value register\n\nYou can [`read`](crate::Reg::read) this register and get [`init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@init`] module"]
#[doc(alias = "INIT")]
pub type Init = crate::Reg<init::InitSpec>;
#[doc = "CRC initial value register"]
pub mod init;
#[doc = "POL (rw) register accessor: CRC polynomial register\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pol`] module"]
#[doc(alias = "POL")]
pub type Pol = crate::Reg<pol::PolSpec>;
#[doc = "CRC polynomial register"]
pub mod pol;
