#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    status: Status,
    macmsb: Macmsb,
    maclsb: Maclsb,
    mdiocsr: Mdiocsr,
    txbase: Txbase,
    rxbase: Rxbase,
    _reserved7: [u8; 0x24],
    rgmii_stat: RgmiiStat,
    rgmii_if: RgmiiIf,
    rgmii_im: RgmiiIm,
    wrap_100m: Wrap100m,
    edge1_100m: Edge1_100m,
    edge2_100m: Edge2_100m,
    wrap_10m: Wrap10m,
    edge1_10m: Edge1_10m,
    edge2_10m: Edge2_10m,
    bit_pattern: BitPattern,
    neg_delay: NegDelay,
    pos_delay: PosDelay,
    bit_swap: BitSwap,
    data_swap: DataSwap,
    ctrl_swap: CtrlSwap,
    phycfg: Phycfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet control and capabilities register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Ethernet status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Ethernet MAC address high bytes"]
    #[inline(always)]
    pub const fn macmsb(&self) -> &Macmsb {
        &self.macmsb
    }
    #[doc = "0x0c - Ethernet MAC address low bytes"]
    #[inline(always)]
    pub const fn maclsb(&self) -> &Maclsb {
        &self.maclsb
    }
    #[doc = "0x10 - MDIO control & status register"]
    #[inline(always)]
    pub const fn mdiocsr(&self) -> &Mdiocsr {
        &self.mdiocsr
    }
    #[doc = "0x14 - Transmitter descriptor table base address pointer"]
    #[inline(always)]
    pub const fn txbase(&self) -> &Txbase {
        &self.txbase
    }
    #[doc = "0x18 - Receiver descriptor table base address pointer"]
    #[inline(always)]
    pub const fn rxbase(&self) -> &Rxbase {
        &self.rxbase
    }
    #[doc = "0x40 - RGMII Status register"]
    #[inline(always)]
    pub const fn rgmii_stat(&self) -> &RgmiiStat {
        &self.rgmii_stat
    }
    #[doc = "0x44 - RGMII Interrupt flag register"]
    #[inline(always)]
    pub const fn rgmii_if(&self) -> &RgmiiIf {
        &self.rgmii_if
    }
    #[doc = "0x48 - RGMII interrupt mask register"]
    #[inline(always)]
    pub const fn rgmii_im(&self) -> &RgmiiIm {
        &self.rgmii_im
    }
    #[doc = "0x4c - RGMII 25MHz clock wrap register"]
    #[inline(always)]
    pub const fn wrap_100m(&self) -> &Wrap100m {
        &self.wrap_100m
    }
    #[doc = "0x50 - RGMII 25 MHz first edge register"]
    #[inline(always)]
    pub const fn edge1_100m(&self) -> &Edge1_100m {
        &self.edge1_100m
    }
    #[doc = "0x54 - RGMII 25 MHz second edge register"]
    #[inline(always)]
    pub const fn edge2_100m(&self) -> &Edge2_100m {
        &self.edge2_100m
    }
    #[doc = "0x58 - RGMII 2.5MHz clock wrap register"]
    #[inline(always)]
    pub const fn wrap_10m(&self) -> &Wrap10m {
        &self.wrap_10m
    }
    #[doc = "0x5c - RGMII 2.5 MHz first edge register"]
    #[inline(always)]
    pub const fn edge1_10m(&self) -> &Edge1_10m {
        &self.edge1_10m
    }
    #[doc = "0x60 - RGMII 2.5 MHz second edge register"]
    #[inline(always)]
    pub const fn edge2_10m(&self) -> &Edge2_10m {
        &self.edge2_10m
    }
    #[doc = "0x64 - RGMII bit pattern register"]
    #[inline(always)]
    pub const fn bit_pattern(&self) -> &BitPattern {
        &self.bit_pattern
    }
    #[doc = "0x68 - RGMII positive clock data receiver delay register"]
    #[inline(always)]
    pub const fn neg_delay(&self) -> &NegDelay {
        &self.neg_delay
    }
    #[doc = "0x6c - RGMII negative clock data receiver delay register"]
    #[inline(always)]
    pub const fn pos_delay(&self) -> &PosDelay {
        &self.pos_delay
    }
    #[doc = "0x70 - RGMII receiver sample order swap register"]
    #[inline(always)]
    pub const fn bit_swap(&self) -> &BitSwap {
        &self.bit_swap
    }
    #[doc = "0x74 - RGMII receiver data swap register"]
    #[inline(always)]
    pub const fn data_swap(&self) -> &DataSwap {
        &self.data_swap
    }
    #[doc = "0x78 - RGMII receiver control signal swap register"]
    #[inline(always)]
    pub const fn ctrl_swap(&self) -> &CtrlSwap {
        &self.ctrl_swap
    }
    #[doc = "0x7c - ETH PHY control register"]
    #[inline(always)]
    pub const fn phycfg(&self) -> &Phycfg {
        &self.phycfg
    }
}
#[doc = "CTRL (rw) register accessor: Ethernet control and capabilities register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Ethernet control and capabilities register"]
pub mod ctrl;
#[doc = "STATUS (rw) register accessor: Ethernet status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Ethernet status register"]
pub mod status;
#[doc = "MACMSB (rw) register accessor: Ethernet MAC address high bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`macmsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@macmsb`] module"]
#[doc(alias = "MACMSB")]
pub type Macmsb = crate::Reg<macmsb::MacmsbSpec>;
#[doc = "Ethernet MAC address high bytes"]
pub mod macmsb;
#[doc = "MACLSB (rw) register accessor: Ethernet MAC address low bytes\n\nYou can [`read`](crate::Reg::read) this register and get [`maclsb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclsb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maclsb`] module"]
#[doc(alias = "MACLSB")]
pub type Maclsb = crate::Reg<maclsb::MaclsbSpec>;
#[doc = "Ethernet MAC address low bytes"]
pub mod maclsb;
#[doc = "MDIOCSR (rw) register accessor: MDIO control & status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdiocsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdiocsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdiocsr`] module"]
#[doc(alias = "MDIOCSR")]
pub type Mdiocsr = crate::Reg<mdiocsr::MdiocsrSpec>;
#[doc = "MDIO control & status register"]
pub mod mdiocsr;
#[doc = "TXBASE (rw) register accessor: Transmitter descriptor table base address pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`txbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbase`] module"]
#[doc(alias = "TXBASE")]
pub type Txbase = crate::Reg<txbase::TxbaseSpec>;
#[doc = "Transmitter descriptor table base address pointer"]
pub mod txbase;
#[doc = "RXBASE (rw) register accessor: Receiver descriptor table base address pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbase`] module"]
#[doc(alias = "RXBASE")]
pub type Rxbase = crate::Reg<rxbase::RxbaseSpec>;
#[doc = "Receiver descriptor table base address pointer"]
pub mod rxbase;
#[doc = "RGMII_STAT (r) register accessor: RGMII Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgmii_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgmii_stat`] module"]
#[doc(alias = "RGMII_STAT")]
pub type RgmiiStat = crate::Reg<rgmii_stat::RgmiiStatSpec>;
#[doc = "RGMII Status register"]
pub mod rgmii_stat;
#[doc = "RGMII_IF (r) register accessor: RGMII Interrupt flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgmii_if::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgmii_if`] module"]
#[doc(alias = "RGMII_IF")]
pub type RgmiiIf = crate::Reg<rgmii_if::RgmiiIfSpec>;
#[doc = "RGMII Interrupt flag register"]
pub mod rgmii_if;
#[doc = "RGMII_IM (rw) register accessor: RGMII interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgmii_im::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgmii_im::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgmii_im`] module"]
#[doc(alias = "RGMII_IM")]
pub type RgmiiIm = crate::Reg<rgmii_im::RgmiiImSpec>;
#[doc = "RGMII interrupt mask register"]
pub mod rgmii_im;
#[doc = "WRAP_100M (rw) register accessor: RGMII 25MHz clock wrap register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrap_100m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrap_100m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrap_100m`] module"]
#[doc(alias = "WRAP_100M")]
pub type Wrap100m = crate::Reg<wrap_100m::Wrap100mSpec>;
#[doc = "RGMII 25MHz clock wrap register"]
pub mod wrap_100m;
#[doc = "EDGE1_100M (rw) register accessor: RGMII 25 MHz first edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`edge1_100m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge1_100m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edge1_100m`] module"]
#[doc(alias = "EDGE1_100M")]
pub type Edge1_100m = crate::Reg<edge1_100m::Edge1_100mSpec>;
#[doc = "RGMII 25 MHz first edge register"]
pub mod edge1_100m;
#[doc = "EDGE2_100M (rw) register accessor: RGMII 25 MHz second edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`edge2_100m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge2_100m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edge2_100m`] module"]
#[doc(alias = "EDGE2_100M")]
pub type Edge2_100m = crate::Reg<edge2_100m::Edge2_100mSpec>;
#[doc = "RGMII 25 MHz second edge register"]
pub mod edge2_100m;
#[doc = "WRAP_10M (rw) register accessor: RGMII 2.5MHz clock wrap register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrap_10m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrap_10m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wrap_10m`] module"]
#[doc(alias = "WRAP_10M")]
pub type Wrap10m = crate::Reg<wrap_10m::Wrap10mSpec>;
#[doc = "RGMII 2.5MHz clock wrap register"]
pub mod wrap_10m;
#[doc = "EDGE1_10M (rw) register accessor: RGMII 2.5 MHz first edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`edge1_10m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge1_10m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edge1_10m`] module"]
#[doc(alias = "EDGE1_10M")]
pub type Edge1_10m = crate::Reg<edge1_10m::Edge1_10mSpec>;
#[doc = "RGMII 2.5 MHz first edge register"]
pub mod edge1_10m;
#[doc = "EDGE2_10M (rw) register accessor: RGMII 2.5 MHz second edge register\n\nYou can [`read`](crate::Reg::read) this register and get [`edge2_10m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edge2_10m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@edge2_10m`] module"]
#[doc(alias = "EDGE2_10M")]
pub type Edge2_10m = crate::Reg<edge2_10m::Edge2_10mSpec>;
#[doc = "RGMII 2.5 MHz second edge register"]
pub mod edge2_10m;
#[doc = "BIT_PATTERN (rw) register accessor: RGMII bit pattern register\n\nYou can [`read`](crate::Reg::read) this register and get [`bit_pattern::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit_pattern::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bit_pattern`] module"]
#[doc(alias = "BIT_PATTERN")]
pub type BitPattern = crate::Reg<bit_pattern::BitPatternSpec>;
#[doc = "RGMII bit pattern register"]
pub mod bit_pattern;
#[doc = "NEG_DELAY (rw) register accessor: RGMII positive clock data receiver delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`neg_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`neg_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@neg_delay`] module"]
#[doc(alias = "NEG_DELAY")]
pub type NegDelay = crate::Reg<neg_delay::NegDelaySpec>;
#[doc = "RGMII positive clock data receiver delay register"]
pub mod neg_delay;
#[doc = "POS_DELAY (rw) register accessor: RGMII negative clock data receiver delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`pos_delay::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pos_delay::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pos_delay`] module"]
#[doc(alias = "POS_DELAY")]
pub type PosDelay = crate::Reg<pos_delay::PosDelaySpec>;
#[doc = "RGMII negative clock data receiver delay register"]
pub mod pos_delay;
#[doc = "BIT_SWAP (rw) register accessor: RGMII receiver sample order swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`bit_swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bit_swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bit_swap`] module"]
#[doc(alias = "BIT_SWAP")]
pub type BitSwap = crate::Reg<bit_swap::BitSwapSpec>;
#[doc = "RGMII receiver sample order swap register"]
pub mod bit_swap;
#[doc = "DATA_SWAP (rw) register accessor: RGMII receiver data swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`data_swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_swap`] module"]
#[doc(alias = "DATA_SWAP")]
pub type DataSwap = crate::Reg<data_swap::DataSwapSpec>;
#[doc = "RGMII receiver data swap register"]
pub mod data_swap;
#[doc = "CTRL_SWAP (rw) register accessor: RGMII receiver control signal swap register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_swap::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_swap::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_swap`] module"]
#[doc(alias = "CTRL_SWAP")]
pub type CtrlSwap = crate::Reg<ctrl_swap::CtrlSwapSpec>;
#[doc = "RGMII receiver control signal swap register"]
pub mod ctrl_swap;
#[doc = "PHYCFG (rw) register accessor: ETH PHY control register\n\nYou can [`read`](crate::Reg::read) this register and get [`phycfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`phycfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phycfg`] module"]
#[doc(alias = "PHYCFG")]
pub type Phycfg = crate::Reg<phycfg::PhycfgSpec>;
#[doc = "ETH PHY control register"]
pub mod phycfg;
