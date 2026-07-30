#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data12: Data12,
    data8: Data8,
    data_dual: DataDual,
    control: Control,
    bgctrl: Bgctrl,
    dmactrl: Dmactrl,
    swtrig: Swtrig,
    hwtrig: Hwtrig,
    patdorl: Patdorl,
    patdorh: Patdorh,
    test: Test,
    _reserved11: [u8; 0x14],
    reserved: Reserved,
}
impl RegisterBlock {
    #[doc = "0x00 - Output data register (12-bit)"]
    #[inline(always)]
    pub const fn data12(&self) -> &Data12 {
        &self.data12
    }
    #[doc = "0x04 - Output data register (8-bit)"]
    #[inline(always)]
    pub const fn data8(&self) -> &Data8 {
        &self.data8
    }
    #[doc = "0x08 - Output data register (Dual mode and DAC0 only)"]
    #[inline(always)]
    pub const fn data_dual(&self) -> &DataDual {
        &self.data_dual
    }
    #[doc = "0x0c - Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x10 - Internal reference-voltage source (adjustable BandGap) register"]
    #[inline(always)]
    pub const fn bgctrl(&self) -> &Bgctrl {
        &self.bgctrl
    }
    #[doc = "0x14 - DAC DMA control register"]
    #[inline(always)]
    pub const fn dmactrl(&self) -> &Dmactrl {
        &self.dmactrl
    }
    #[doc = "0x18 - Software trigger control register"]
    #[inline(always)]
    pub const fn swtrig(&self) -> &Swtrig {
        &self.swtrig
    }
    #[doc = "0x1c - Internal Hardware trigger generator control register"]
    #[inline(always)]
    pub const fn hwtrig(&self) -> &Hwtrig {
        &self.hwtrig
    }
    #[doc = "0x20 - Pattern data output register (low part)"]
    #[inline(always)]
    pub const fn patdorl(&self) -> &Patdorl {
        &self.patdorl
    }
    #[doc = "0x24 - Pattern data output register (high part)"]
    #[inline(always)]
    pub const fn patdorh(&self) -> &Patdorh {
        &self.patdorh
    }
    #[doc = "0x28 - DAC test register"]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0x40 - MCP4921 control register"]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
}
#[doc = "DATA12 (rw) register accessor: Output data register (12-bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`data12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data12`] module"]
#[doc(alias = "DATA12")]
pub type Data12 = crate::Reg<data12::Data12Spec>;
#[doc = "Output data register (12-bit)"]
pub mod data12;
#[doc = "DATA8 (rw) register accessor: Output data register (8-bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`data8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data8`] module"]
#[doc(alias = "DATA8")]
pub type Data8 = crate::Reg<data8::Data8Spec>;
#[doc = "Output data register (8-bit)"]
pub mod data8;
#[doc = "DATA_DUAL (rw) register accessor: Output data register (Dual mode and DAC0 only)\n\nYou can [`read`](crate::Reg::read) this register and get [`data_dual::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_dual::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_dual`] module"]
#[doc(alias = "DATA_DUAL")]
pub type DataDual = crate::Reg<data_dual::DataDualSpec>;
#[doc = "Output data register (Dual mode and DAC0 only)"]
pub mod data_dual;
#[doc = "CONTROL (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Control register"]
pub mod control;
#[doc = "BGCTRL (rw) register accessor: Internal reference-voltage source (adjustable BandGap) register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgctrl`] module"]
#[doc(alias = "BGCTRL")]
pub type Bgctrl = crate::Reg<bgctrl::BgctrlSpec>;
#[doc = "Internal reference-voltage source (adjustable BandGap) register"]
pub mod bgctrl;
#[doc = "DMACTRL (rw) register accessor: DAC DMA control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactrl`] module"]
#[doc(alias = "DMACTRL")]
pub type Dmactrl = crate::Reg<dmactrl::DmactrlSpec>;
#[doc = "DAC DMA control register"]
pub mod dmactrl;
#[doc = "SWTRIG (w) register accessor: Software trigger control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrig::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrig`] module"]
#[doc(alias = "SWTRIG")]
pub type Swtrig = crate::Reg<swtrig::SwtrigSpec>;
#[doc = "Software trigger control register"]
pub mod swtrig;
#[doc = "HWTRIG (rw) register accessor: Internal Hardware trigger generator control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hwtrig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwtrig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwtrig`] module"]
#[doc(alias = "HWTRIG")]
pub type Hwtrig = crate::Reg<hwtrig::HwtrigSpec>;
#[doc = "Internal Hardware trigger generator control register"]
pub mod hwtrig;
#[doc = "PATDORL (rw) register accessor: Pattern data output register (low part)\n\nYou can [`read`](crate::Reg::read) this register and get [`patdorl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patdorl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patdorl`] module"]
#[doc(alias = "PATDORL")]
pub type Patdorl = crate::Reg<patdorl::PatdorlSpec>;
#[doc = "Pattern data output register (low part)"]
pub mod patdorl;
#[doc = "PATDORH (rw) register accessor: Pattern data output register (high part)\n\nYou can [`read`](crate::Reg::read) this register and get [`patdorh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`patdorh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@patdorh`] module"]
#[doc(alias = "PATDORH")]
pub type Patdorh = crate::Reg<patdorh::PatdorhSpec>;
#[doc = "Pattern data output register (high part)"]
pub mod patdorh;
#[doc = "TEST (rw) register accessor: DAC test register\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`] module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "DAC test register"]
pub mod test;
#[doc = "Reserved (rw) register accessor: MCP4921 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`] module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = "MCP4921 control register"]
pub mod reserved;
