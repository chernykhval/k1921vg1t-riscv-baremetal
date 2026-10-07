#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    count: Count,
    clkdiv: Clkdiv,
    period: Period,
    im: Im,
    ris: Ris,
    mis: Mis,
    ic: Ic,
    dma_txim: DmaTxim,
    dma_rxim: DmaRxim,
    extevt_im: ExtevtIm,
    _reserved11: [u8; 0xd4],
    capcom: [Capcom; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Timer control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Current value timer register"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x08 - Clock divider register"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x0c - Timer period register"]
    #[inline(always)]
    pub const fn period(&self) -> &Period {
        &self.period
    }
    #[doc = "0x10 - Interrupt mask register"]
    #[inline(always)]
    pub const fn im(&self) -> &Im {
        &self.im
    }
    #[doc = "0x14 - Raw Interrupt Status register"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x18 - Masked Interrupt Status register"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x1c - Clear Interrupt Status register"]
    #[inline(always)]
    pub const fn ic(&self) -> &Ic {
        &self.ic
    }
    #[doc = "0x20 - DMA TX request mask register"]
    #[inline(always)]
    pub const fn dma_txim(&self) -> &DmaTxim {
        &self.dma_txim
    }
    #[doc = "0x24 - DMA RX request mask register"]
    #[inline(always)]
    pub const fn dma_rxim(&self) -> &DmaRxim {
        &self.dma_rxim
    }
    #[doc = "0x28 - External Event mask register"]
    #[inline(always)]
    pub const fn extevt_im(&self) -> &ExtevtIm {
        &self.extevt_im
    }
    #[doc = "0x100..0x140 - CAPCOM"]
    #[inline(always)]
    pub const fn capcom(&self, n: usize) -> &Capcom {
        &self.capcom[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - CAPCOM"]
    #[inline(always)]
    pub fn capcom_iter(&self) -> impl Iterator<Item = &Capcom> {
        self.capcom.iter()
    }
}
#[doc = "CTRL (rw) register accessor: Timer control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Timer control register"]
pub mod ctrl;
#[doc = "COUNT (rw) register accessor: Current value timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`] module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "Current value timer register"]
pub mod count;
#[doc = "CLKDIV (rw) register accessor: Clock divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "Clock divider register"]
pub mod clkdiv;
#[doc = "PERIOD (rw) register accessor: Timer period register\n\nYou can [`read`](crate::Reg::read) this register and get [`period::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`period::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@period`] module"]
#[doc(alias = "PERIOD")]
pub type Period = crate::Reg<period::PeriodSpec>;
#[doc = "Timer period register"]
pub mod period;
#[doc = "IM (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@im`] module"]
#[doc(alias = "IM")]
pub type Im = crate::Reg<im::ImSpec>;
#[doc = "Interrupt mask register"]
pub mod im;
#[doc = "RIS (r) register accessor: Raw Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`] module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw Interrupt Status register"]
pub mod ris;
#[doc = "MIS (r) register accessor: Masked Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`] module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked Interrupt Status register"]
pub mod mis;
#[doc = "IC (w) register accessor: Clear Interrupt Status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ic`] module"]
#[doc(alias = "IC")]
pub type Ic = crate::Reg<ic::IcSpec>;
#[doc = "Clear Interrupt Status register"]
pub mod ic;
#[doc = "DMA_TXIM (rw) register accessor: DMA TX request mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_txim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_txim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_txim`] module"]
#[doc(alias = "DMA_TXIM")]
pub type DmaTxim = crate::Reg<dma_txim::DmaTximSpec>;
#[doc = "DMA TX request mask register"]
pub mod dma_txim;
#[doc = "DMA_RXIM (rw) register accessor: DMA RX request mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_rxim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_rxim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_rxim`] module"]
#[doc(alias = "DMA_RXIM")]
pub type DmaRxim = crate::Reg<dma_rxim::DmaRximSpec>;
#[doc = "DMA RX request mask register"]
pub mod dma_rxim;
#[doc = "EXTEVT_IM (rw) register accessor: External Event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`extevt_im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extevt_im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extevt_im`] module"]
#[doc(alias = "EXTEVT_IM")]
pub type ExtevtIm = crate::Reg<extevt_im::ExtevtImSpec>;
#[doc = "External Event mask register"]
pub mod extevt_im;
#[doc = "CAPCOM"]
pub use self::capcom::Capcom;
#[doc = r"Cluster"]
#[doc = "CAPCOM"]
pub mod capcom;
