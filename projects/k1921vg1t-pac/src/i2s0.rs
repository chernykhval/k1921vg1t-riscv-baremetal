#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2sctrl: I2sctrl,
    i2sclkdiv1: I2sclkdiv1,
    i2sclkdiv2: I2sclkdiv2,
    i2sfrmctrl1: I2sfrmctrl1,
    i2sfrmctrl2: I2sfrmctrl2,
    i2srecbufused: I2srecbufused,
    i2stransdata: I2stransdata,
    i2smclkdiv: I2smclkdiv,
    i2srecdata: I2srecdata,
    i2sshiftdata: I2sshiftdata,
    i2sstate: I2sstate,
    i2sstateclr: I2sstateclr,
    i2strbufused: I2strbufused,
}
impl RegisterBlock {
    #[doc = "0x00 - I2S Control register"]
    #[inline(always)]
    pub const fn i2sctrl(&self) -> &I2sctrl {
        &self.i2sctrl
    }
    #[doc = "0x04 - I2S Clock Divider 1 register"]
    #[inline(always)]
    pub const fn i2sclkdiv1(&self) -> &I2sclkdiv1 {
        &self.i2sclkdiv1
    }
    #[doc = "0x08 - I2S Clock Divider 2 register"]
    #[inline(always)]
    pub const fn i2sclkdiv2(&self) -> &I2sclkdiv2 {
        &self.i2sclkdiv2
    }
    #[doc = "0x0c - I2S Frame control 1 register"]
    #[inline(always)]
    pub const fn i2sfrmctrl1(&self) -> &I2sfrmctrl1 {
        &self.i2sfrmctrl1
    }
    #[doc = "0x10 - I2S Frame control 2 register"]
    #[inline(always)]
    pub const fn i2sfrmctrl2(&self) -> &I2sfrmctrl2 {
        &self.i2sfrmctrl2
    }
    #[doc = "0x14 - I2S Receiver Buffer Counts register"]
    #[inline(always)]
    pub const fn i2srecbufused(&self) -> &I2srecbufused {
        &self.i2srecbufused
    }
    #[doc = "0x18 - I2S Transmitter Data register"]
    #[inline(always)]
    pub const fn i2stransdata(&self) -> &I2stransdata {
        &self.i2stransdata
    }
    #[doc = "0x1c - I2S Master Clock Divider register"]
    #[inline(always)]
    pub const fn i2smclkdiv(&self) -> &I2smclkdiv {
        &self.i2smclkdiv
    }
    #[doc = "0x20 - I2S Receiver Data register"]
    #[inline(always)]
    pub const fn i2srecdata(&self) -> &I2srecdata {
        &self.i2srecdata
    }
    #[doc = "0x24 - I2S Shift Data register"]
    #[inline(always)]
    pub const fn i2sshiftdata(&self) -> &I2sshiftdata {
        &self.i2sshiftdata
    }
    #[doc = "0x28 - I2S State register"]
    #[inline(always)]
    pub const fn i2sstate(&self) -> &I2sstate {
        &self.i2sstate
    }
    #[doc = "0x2c - I2S State Clear register"]
    #[inline(always)]
    pub const fn i2sstateclr(&self) -> &I2sstateclr {
        &self.i2sstateclr
    }
    #[doc = "0x30 - I2S Transmitter Buffer Counts register"]
    #[inline(always)]
    pub const fn i2strbufused(&self) -> &I2strbufused {
        &self.i2strbufused
    }
}
#[doc = "I2SCTRL (rw) register accessor: I2S Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctrl`] module"]
#[doc(alias = "I2SCTRL")]
pub type I2sctrl = crate::Reg<i2sctrl::I2sctrlSpec>;
#[doc = "I2S Control register"]
pub mod i2sctrl;
#[doc = "I2SCLKDIV1 (rw) register accessor: I2S Clock Divider 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sclkdiv1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sclkdiv1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sclkdiv1`] module"]
#[doc(alias = "I2SCLKDIV1")]
pub type I2sclkdiv1 = crate::Reg<i2sclkdiv1::I2sclkdiv1Spec>;
#[doc = "I2S Clock Divider 1 register"]
pub mod i2sclkdiv1;
#[doc = "I2SCLKDIV2 (rw) register accessor: I2S Clock Divider 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sclkdiv2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sclkdiv2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sclkdiv2`] module"]
#[doc(alias = "I2SCLKDIV2")]
pub type I2sclkdiv2 = crate::Reg<i2sclkdiv2::I2sclkdiv2Spec>;
#[doc = "I2S Clock Divider 2 register"]
pub mod i2sclkdiv2;
#[doc = "I2SFRMCTRL1 (rw) register accessor: I2S Frame control 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sfrmctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sfrmctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sfrmctrl1`] module"]
#[doc(alias = "I2SFRMCTRL1")]
pub type I2sfrmctrl1 = crate::Reg<i2sfrmctrl1::I2sfrmctrl1Spec>;
#[doc = "I2S Frame control 1 register"]
pub mod i2sfrmctrl1;
#[doc = "I2SFRMCTRL2 (rw) register accessor: I2S Frame control 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sfrmctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sfrmctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sfrmctrl2`] module"]
#[doc(alias = "I2SFRMCTRL2")]
pub type I2sfrmctrl2 = crate::Reg<i2sfrmctrl2::I2sfrmctrl2Spec>;
#[doc = "I2S Frame control 2 register"]
pub mod i2sfrmctrl2;
#[doc = "I2SRECBUFUSED (r) register accessor: I2S Receiver Buffer Counts register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2srecbufused::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2srecbufused`] module"]
#[doc(alias = "I2SRECBUFUSED")]
pub type I2srecbufused = crate::Reg<i2srecbufused::I2srecbufusedSpec>;
#[doc = "I2S Receiver Buffer Counts register"]
pub mod i2srecbufused;
#[doc = "I2STRANSDATA (w) register accessor: I2S Transmitter Data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2stransdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2stransdata`] module"]
#[doc(alias = "I2STRANSDATA")]
pub type I2stransdata = crate::Reg<i2stransdata::I2stransdataSpec>;
#[doc = "I2S Transmitter Data register"]
pub mod i2stransdata;
#[doc = "I2SMCLKDIV (rw) register accessor: I2S Master Clock Divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2smclkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2smclkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2smclkdiv`] module"]
#[doc(alias = "I2SMCLKDIV")]
pub type I2smclkdiv = crate::Reg<i2smclkdiv::I2smclkdivSpec>;
#[doc = "I2S Master Clock Divider register"]
pub mod i2smclkdiv;
#[doc = "I2SRECDATA (r) register accessor: I2S Receiver Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2srecdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2srecdata`] module"]
#[doc(alias = "I2SRECDATA")]
pub type I2srecdata = crate::Reg<i2srecdata::I2srecdataSpec>;
#[doc = "I2S Receiver Data register"]
pub mod i2srecdata;
#[doc = "I2SSHIFTDATA (rw) register accessor: I2S Shift Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sshiftdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sshiftdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sshiftdata`] module"]
#[doc(alias = "I2SSHIFTDATA")]
pub type I2sshiftdata = crate::Reg<i2sshiftdata::I2sshiftdataSpec>;
#[doc = "I2S Shift Data register"]
pub mod i2sshiftdata;
#[doc = "I2SSTATE (r) register accessor: I2S State register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sstate::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sstate`] module"]
#[doc(alias = "I2SSTATE")]
pub type I2sstate = crate::Reg<i2sstate::I2sstateSpec>;
#[doc = "I2S State register"]
pub mod i2sstate;
#[doc = "I2SSTATECLR (w) register accessor: I2S State Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sstateclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sstateclr`] module"]
#[doc(alias = "I2SSTATECLR")]
pub type I2sstateclr = crate::Reg<i2sstateclr::I2sstateclrSpec>;
#[doc = "I2S State Clear register"]
pub mod i2sstateclr;
#[doc = "I2STRBUFUSED (r) register accessor: I2S Transmitter Buffer Counts register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2strbufused::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2strbufused`] module"]
#[doc(alias = "I2STRBUFUSED")]
pub type I2strbufused = crate::Reg<i2strbufused::I2strbufusedSpec>;
#[doc = "I2S Transmitter Buffer Counts register"]
pub mod i2strbufused;
