#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ch: [Ch; 32],
    ch_intstat: ChIntstat,
    control: Control,
    rd_priority: RdPriority,
    wr_priority: WrPriority,
    ch_start: ChStart,
    ch_enable: ChEnable,
    idle: Idle,
    _reserved8: [u8; 0x04],
    per_rx0_ctrl: PerRx0Ctrl,
    per_tx0_ctrl: PerTx0Ctrl,
    per_rx1_ctrl: PerRx1Ctrl,
    per_tx1_ctrl: PerTx1Ctrl,
    per_rx2_ctrl: PerRx2Ctrl,
    per_tx2_ctrl: PerTx2Ctrl,
    _reserved14: [u8; 0x04],
    reserved: Reserved,
}
impl RegisterBlock {
    #[doc = "0x00..0x1000 - CH"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1000 - CH"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        self.ch.iter()
    }
    #[doc = "0x1000 - DMA Core Interrupt status register"]
    #[inline(always)]
    pub const fn ch_intstat(&self) -> &ChIntstat {
        &self.ch_intstat
    }
    #[doc = "0x1004 - DMA Core Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x1008 - DMA Core read priority channels"]
    #[inline(always)]
    pub const fn rd_priority(&self) -> &RdPriority {
        &self.rd_priority
    }
    #[doc = "0x100c - DMA Core write priority channels"]
    #[inline(always)]
    pub const fn wr_priority(&self) -> &WrPriority {
        &self.wr_priority
    }
    #[doc = "0x1010 - DMA channel start register"]
    #[inline(always)]
    pub const fn ch_start(&self) -> &ChStart {
        &self.ch_start
    }
    #[doc = "0x1014 - DMA channel enable register"]
    #[inline(always)]
    pub const fn ch_enable(&self) -> &ChEnable {
        &self.ch_enable
    }
    #[doc = "0x1018 - Idle indication register"]
    #[inline(always)]
    pub const fn idle(&self) -> &Idle {
        &self.idle
    }
    #[doc = "0x1020 - Direct control of peripheral RX request (register 0)"]
    #[inline(always)]
    pub const fn per_rx0_ctrl(&self) -> &PerRx0Ctrl {
        &self.per_rx0_ctrl
    }
    #[doc = "0x1024 - Direct control of peripheral TX request (register 0)"]
    #[inline(always)]
    pub const fn per_tx0_ctrl(&self) -> &PerTx0Ctrl {
        &self.per_tx0_ctrl
    }
    #[doc = "0x1028 - Direct control of peripheral RX request (register 1)"]
    #[inline(always)]
    pub const fn per_rx1_ctrl(&self) -> &PerRx1Ctrl {
        &self.per_rx1_ctrl
    }
    #[doc = "0x102c - Direct control of peripheral TX request (register 1)"]
    #[inline(always)]
    pub const fn per_tx1_ctrl(&self) -> &PerTx1Ctrl {
        &self.per_tx1_ctrl
    }
    #[doc = "0x1030 - Direct control of peripheral RX request (register 2)"]
    #[inline(always)]
    pub const fn per_rx2_ctrl(&self) -> &PerRx2Ctrl {
        &self.per_rx2_ctrl
    }
    #[doc = "0x1034 - Direct control of peripheral TX request (register 2)"]
    #[inline(always)]
    pub const fn per_tx2_ctrl(&self) -> &PerTx2Ctrl {
        &self.per_tx2_ctrl
    }
    #[doc = "0x103c - "]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
}
#[doc = "CH"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "CH"]
pub mod ch;
#[doc = "CH_INTSTAT (r) register accessor: DMA Core Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_intstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_intstat`] module"]
#[doc(alias = "CH_INTSTAT")]
pub type ChIntstat = crate::Reg<ch_intstat::ChIntstatSpec>;
#[doc = "DMA Core Interrupt status register"]
pub mod ch_intstat;
#[doc = "CONTROL (rw) register accessor: DMA Core Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "DMA Core Control register"]
pub mod control;
#[doc = "RD_PRIORITY (rw) register accessor: DMA Core read priority channels\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_priority`] module"]
#[doc(alias = "RD_PRIORITY")]
pub type RdPriority = crate::Reg<rd_priority::RdPrioritySpec>;
#[doc = "DMA Core read priority channels"]
pub mod rd_priority;
#[doc = "WR_PRIORITY (rw) register accessor: DMA Core write priority channels\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_priority`] module"]
#[doc(alias = "WR_PRIORITY")]
pub type WrPriority = crate::Reg<wr_priority::WrPrioritySpec>;
#[doc = "DMA Core write priority channels"]
pub mod wr_priority;
#[doc = "CH_START (w) register accessor: DMA channel start register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_start`] module"]
#[doc(alias = "CH_START")]
pub type ChStart = crate::Reg<ch_start::ChStartSpec>;
#[doc = "DMA channel start register"]
pub mod ch_start;
#[doc = "CH_ENABLE (rw) register accessor: DMA channel enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_enable`] module"]
#[doc(alias = "CH_ENABLE")]
pub type ChEnable = crate::Reg<ch_enable::ChEnableSpec>;
#[doc = "DMA channel enable register"]
pub mod ch_enable;
#[doc = "IDLE (r) register accessor: Idle indication register\n\nYou can [`read`](crate::Reg::read) this register and get [`idle::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idle`] module"]
#[doc(alias = "IDLE")]
pub type Idle = crate::Reg<idle::IdleSpec>;
#[doc = "Idle indication register"]
pub mod idle;
#[doc = "PER_RX0_CTRL (rw) register accessor: Direct control of peripheral RX request (register 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_rx0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_rx0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_rx0_ctrl`] module"]
#[doc(alias = "PER_RX0_CTRL")]
pub type PerRx0Ctrl = crate::Reg<per_rx0_ctrl::PerRx0CtrlSpec>;
#[doc = "Direct control of peripheral RX request (register 0)"]
pub mod per_rx0_ctrl;
#[doc = "PER_TX0_CTRL (rw) register accessor: Direct control of peripheral TX request (register 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_tx0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_tx0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_tx0_ctrl`] module"]
#[doc(alias = "PER_TX0_CTRL")]
pub type PerTx0Ctrl = crate::Reg<per_tx0_ctrl::PerTx0CtrlSpec>;
#[doc = "Direct control of peripheral TX request (register 0)"]
pub mod per_tx0_ctrl;
#[doc = "PER_RX1_CTRL (rw) register accessor: Direct control of peripheral RX request (register 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_rx1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_rx1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_rx1_ctrl`] module"]
#[doc(alias = "PER_RX1_CTRL")]
pub type PerRx1Ctrl = crate::Reg<per_rx1_ctrl::PerRx1CtrlSpec>;
#[doc = "Direct control of peripheral RX request (register 1)"]
pub mod per_rx1_ctrl;
#[doc = "PER_TX1_CTRL (rw) register accessor: Direct control of peripheral TX request (register 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_tx1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_tx1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_tx1_ctrl`] module"]
#[doc(alias = "PER_TX1_CTRL")]
pub type PerTx1Ctrl = crate::Reg<per_tx1_ctrl::PerTx1CtrlSpec>;
#[doc = "Direct control of peripheral TX request (register 1)"]
pub mod per_tx1_ctrl;
#[doc = "PER_RX2_CTRL (rw) register accessor: Direct control of peripheral RX request (register 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_rx2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_rx2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_rx2_ctrl`] module"]
#[doc(alias = "PER_RX2_CTRL")]
pub type PerRx2Ctrl = crate::Reg<per_rx2_ctrl::PerRx2CtrlSpec>;
#[doc = "Direct control of peripheral RX request (register 2)"]
pub mod per_rx2_ctrl;
#[doc = "PER_TX2_CTRL (rw) register accessor: Direct control of peripheral TX request (register 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`per_tx2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`per_tx2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_tx2_ctrl`] module"]
#[doc(alias = "PER_TX2_CTRL")]
pub type PerTx2Ctrl = crate::Reg<per_tx2_ctrl::PerTx2CtrlSpec>;
#[doc = "Direct control of peripheral TX request (register 2)"]
pub mod per_tx2_ctrl;
#[doc = "Reserved (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`] module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = ""]
pub mod reserved;
