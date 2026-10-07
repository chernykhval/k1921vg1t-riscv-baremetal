#[repr(C)]
#[doc = "SEQ"]
#[doc(alias = "SEQ")]
pub struct Seq {
    srqsel0: Srqsel0,
    srqsel1: Srqsel1,
    srqsel2: Srqsel2,
    srqsel3: Srqsel3,
    srqsel4: Srqsel4,
    srqsel5: Srqsel5,
    srqctl: Srqctl,
    srqstat: Srqstat,
    sdmactl: Sdmactl,
    scctl: Scctl,
    scval: Scval,
    sdc: Sdc,
    srtmr: Srtmr,
    sfload: Sfload,
    sfifo: Sfifo,
}
impl Seq {
    #[doc = "0x00 - Sequencer request ADC channels selection register 0"]
    #[inline(always)]
    pub const fn srqsel0(&self) -> &Srqsel0 {
        &self.srqsel0
    }
    #[doc = "0x04 - Sequencer request ADC channels selection register 1"]
    #[inline(always)]
    pub const fn srqsel1(&self) -> &Srqsel1 {
        &self.srqsel1
    }
    #[doc = "0x08 - Sequencer request ADC channels selection register 2"]
    #[inline(always)]
    pub const fn srqsel2(&self) -> &Srqsel2 {
        &self.srqsel2
    }
    #[doc = "0x0c - Sequencer request ADC channels selection register 3"]
    #[inline(always)]
    pub const fn srqsel3(&self) -> &Srqsel3 {
        &self.srqsel3
    }
    #[doc = "0x10 - Sequencer request ADC channels selection register 4"]
    #[inline(always)]
    pub const fn srqsel4(&self) -> &Srqsel4 {
        &self.srqsel4
    }
    #[doc = "0x14 - Sequencer request ADC channels selection register 5"]
    #[inline(always)]
    pub const fn srqsel5(&self) -> &Srqsel5 {
        &self.srqsel5
    }
    #[doc = "0x18 - Sequencer request control register"]
    #[inline(always)]
    pub const fn srqctl(&self) -> &Srqctl {
        &self.srqctl
    }
    #[doc = "0x1c - Sequencer request status register"]
    #[inline(always)]
    pub const fn srqstat(&self) -> &Srqstat {
        &self.srqstat
    }
    #[doc = "0x20 - Sequencer DMA control register"]
    #[inline(always)]
    pub const fn sdmactl(&self) -> &Sdmactl {
        &self.sdmactl
    }
    #[doc = "0x24 - Sequencer ADC interrupt and restart counter control register"]
    #[inline(always)]
    pub const fn scctl(&self) -> &Scctl {
        &self.scctl
    }
    #[doc = "0x28 - Sequencer ADC interrupt and restart counter current value register"]
    #[inline(always)]
    pub const fn scval(&self) -> &Scval {
        &self.scval
    }
    #[doc = "0x2c - Sequencer digital comparator selection register"]
    #[inline(always)]
    pub const fn sdc(&self) -> &Sdc {
        &self.sdc
    }
    #[doc = "0x30 - Sequencer ADC restart timer register"]
    #[inline(always)]
    pub const fn srtmr(&self) -> &Srtmr {
        &self.srtmr
    }
    #[doc = "0x34 - Sequencer FIFO load status register"]
    #[inline(always)]
    pub const fn sfload(&self) -> &Sfload {
        &self.sfload
    }
    #[doc = "0x38 - Sequencer FIFO register"]
    #[inline(always)]
    pub const fn sfifo(&self) -> &Sfifo {
        &self.sfifo
    }
}
#[doc = "SRQSEL0 (rw) register accessor: Sequencer request ADC channels selection register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqsel0`] module"]
#[doc(alias = "SRQSEL0")]
pub type Srqsel0 = crate::Reg<srqsel0::Srqsel0Spec>;
#[doc = "Sequencer request ADC channels selection register 0"]
pub mod srqsel0;
#[doc = "SRQSEL1 (rw) register accessor: Sequencer request ADC channels selection register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqsel1`] module"]
#[doc(alias = "SRQSEL1")]
pub type Srqsel1 = crate::Reg<srqsel1::Srqsel1Spec>;
#[doc = "Sequencer request ADC channels selection register 1"]
pub mod srqsel1;
#[doc = "SRQSEL2 (rw) register accessor: Sequencer request ADC channels selection register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqsel2`] module"]
#[doc(alias = "SRQSEL2")]
pub type Srqsel2 = crate::Reg<srqsel2::Srqsel2Spec>;
#[doc = "Sequencer request ADC channels selection register 2"]
pub mod srqsel2;
#[doc = "SRQSEL3 (rw) register accessor: Sequencer request ADC channels selection register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqsel3`] module"]
#[doc(alias = "SRQSEL3")]
pub type Srqsel3 = crate::Reg<srqsel3::Srqsel3Spec>;
#[doc = "Sequencer request ADC channels selection register 3"]
pub mod srqsel3;
#[doc = "SRQSEL4 (rw) register accessor: Sequencer request ADC channels selection register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqsel4`] module"]
#[doc(alias = "SRQSEL4")]
pub type Srqsel4 = crate::Reg<srqsel4::Srqsel4Spec>;
#[doc = "Sequencer request ADC channels selection register 4"]
pub mod srqsel4;
#[doc = "SRQSEL5 (rw) register accessor: Sequencer request ADC channels selection register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`srqsel5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqsel5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqsel5`] module"]
#[doc(alias = "SRQSEL5")]
pub type Srqsel5 = crate::Reg<srqsel5::Srqsel5Spec>;
#[doc = "Sequencer request ADC channels selection register 5"]
pub mod srqsel5;
#[doc = "SRQCTL (rw) register accessor: Sequencer request control register\n\nYou can [`read`](crate::Reg::read) this register and get [`srqctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srqctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqctl`] module"]
#[doc(alias = "SRQCTL")]
pub type Srqctl = crate::Reg<srqctl::SrqctlSpec>;
#[doc = "Sequencer request control register"]
pub mod srqctl;
#[doc = "SRQSTAT (r) register accessor: Sequencer request status register\n\nYou can [`read`](crate::Reg::read) this register and get [`srqstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srqstat`] module"]
#[doc(alias = "SRQSTAT")]
pub type Srqstat = crate::Reg<srqstat::SrqstatSpec>;
#[doc = "Sequencer request status register"]
pub mod srqstat;
#[doc = "SDMACTL (rw) register accessor: Sequencer DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmactl`] module"]
#[doc(alias = "SDMACTL")]
pub type Sdmactl = crate::Reg<sdmactl::SdmactlSpec>;
#[doc = "Sequencer DMA control register"]
pub mod sdmactl;
#[doc = "SCCTL (rw) register accessor: Sequencer ADC interrupt and restart counter control register\n\nYou can [`read`](crate::Reg::read) this register and get [`scctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scctl`] module"]
#[doc(alias = "SCCTL")]
pub type Scctl = crate::Reg<scctl::ScctlSpec>;
#[doc = "Sequencer ADC interrupt and restart counter control register"]
pub mod scctl;
#[doc = "SCVAL (rw) register accessor: Sequencer ADC interrupt and restart counter current value register\n\nYou can [`read`](crate::Reg::read) this register and get [`scval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scval`] module"]
#[doc(alias = "SCVAL")]
pub type Scval = crate::Reg<scval::ScvalSpec>;
#[doc = "Sequencer ADC interrupt and restart counter current value register"]
pub mod scval;
#[doc = "SDC (rw) register accessor: Sequencer digital comparator selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdc`] module"]
#[doc(alias = "SDC")]
pub type Sdc = crate::Reg<sdc::SdcSpec>;
#[doc = "Sequencer digital comparator selection register"]
pub mod sdc;
#[doc = "SRTMR (rw) register accessor: Sequencer ADC restart timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`srtmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srtmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srtmr`] module"]
#[doc(alias = "SRTMR")]
pub type Srtmr = crate::Reg<srtmr::SrtmrSpec>;
#[doc = "Sequencer ADC restart timer register"]
pub mod srtmr;
#[doc = "SFLOAD (r) register accessor: Sequencer FIFO load status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfload::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfload`] module"]
#[doc(alias = "SFLOAD")]
pub type Sfload = crate::Reg<sfload::SfloadSpec>;
#[doc = "Sequencer FIFO load status register"]
pub mod sfload;
#[doc = "SFIFO (r) register accessor: Sequencer FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`sfifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfifo`] module"]
#[doc(alias = "SFIFO")]
pub type Sfifo = crate::Reg<sfifo::SfifoSpec>;
#[doc = "Sequencer FIFO register"]
pub mod sfifo;
