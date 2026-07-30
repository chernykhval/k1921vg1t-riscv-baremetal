#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iv: [Iv; 4],
    text_in: [TextIn; 4],
    key: [Key; 8],
    irq_enable: IrqEnable,
    control: Control,
    base_descriptor: BaseDescriptor,
    dma_control: DmaControl,
    terminate: Terminate,
    _reserved8: [u8; 0x0c],
    irq: Irq,
    _reserved9: [u8; 0x0c],
    text_out: [TextOut; 4],
    status: Status,
    current_descriptor: CurrentDescriptor,
    next_descriptor: NextDescriptor,
}
impl RegisterBlock {
    #[doc = "0x00..0x10 - IV"]
    #[inline(always)]
    pub const fn iv(&self, n: usize) -> &Iv {
        &self.iv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x10 - IV"]
    #[inline(always)]
    pub fn iv_iter(&self) -> impl Iterator<Item = &Iv> {
        self.iv.iter()
    }
    #[doc = "0x10..0x20 - TEXT_IN"]
    #[inline(always)]
    pub const fn text_in(&self, n: usize) -> &TextIn {
        &self.text_in[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - TEXT_IN"]
    #[inline(always)]
    pub fn text_in_iter(&self) -> impl Iterator<Item = &TextIn> {
        self.text_in.iter()
    }
    #[doc = "0x20..0x40 - KEY"]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &Key {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x40 - KEY"]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &Key> {
        self.key.iter()
    }
    #[doc = "0x40 - Interrupt enable register"]
    #[inline(always)]
    pub const fn irq_enable(&self) -> &IrqEnable {
        &self.irq_enable
    }
    #[doc = "0x44 - Operation control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x48 - Register of address of base descriptor"]
    #[inline(always)]
    pub const fn base_descriptor(&self) -> &BaseDescriptor {
        &self.base_descriptor
    }
    #[doc = "0x4c - DMA operations control register"]
    #[inline(always)]
    pub const fn dma_control(&self) -> &DmaControl {
        &self.dma_control
    }
    #[doc = "0x50 - Operation emergency terminate register (write of 0xD0 terminate current oeparation)"]
    #[inline(always)]
    pub const fn terminate(&self) -> &Terminate {
        &self.terminate
    }
    #[doc = "0x60 - Interrupts pending register"]
    #[inline(always)]
    pub const fn irq(&self) -> &Irq {
        &self.irq
    }
    #[doc = "0x70..0x80 - TEXT_OUT"]
    #[inline(always)]
    pub const fn text_out(&self, n: usize) -> &TextOut {
        &self.text_out[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - TEXT_OUT"]
    #[inline(always)]
    pub fn text_out_iter(&self) -> impl Iterator<Item = &TextOut> {
        self.text_out.iter()
    }
    #[doc = "0x80 - Operation status register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x84 - Register of address of current handling descriptor"]
    #[inline(always)]
    pub const fn current_descriptor(&self) -> &CurrentDescriptor {
        &self.current_descriptor
    }
    #[doc = "0x88 - Register of address of next descriptor"]
    #[inline(always)]
    pub const fn next_descriptor(&self) -> &NextDescriptor {
        &self.next_descriptor
    }
}
#[doc = "IV"]
pub use self::iv::Iv;
#[doc = r"Cluster"]
#[doc = "IV"]
pub mod iv;
#[doc = "TEXT_IN"]
pub use self::text_in::TextIn;
#[doc = r"Cluster"]
#[doc = "TEXT_IN"]
pub mod text_in;
#[doc = "KEY"]
pub use self::key::Key;
#[doc = r"Cluster"]
#[doc = "KEY"]
pub mod key;
#[doc = "IRQ_ENABLE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_enable`] module"]
#[doc(alias = "IRQ_ENABLE")]
pub type IrqEnable = crate::Reg<irq_enable::IrqEnableSpec>;
#[doc = "Interrupt enable register"]
pub mod irq_enable;
#[doc = "CONTROL (rw) register accessor: Operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Operation control register"]
pub mod control;
#[doc = "BASE_DESCRIPTOR (rw) register accessor: Register of address of base descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`base_descriptor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base_descriptor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base_descriptor`] module"]
#[doc(alias = "BASE_DESCRIPTOR")]
pub type BaseDescriptor = crate::Reg<base_descriptor::BaseDescriptorSpec>;
#[doc = "Register of address of base descriptor"]
pub mod base_descriptor;
#[doc = "DMA_CONTROL (rw) register accessor: DMA operations control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma_control`] module"]
#[doc(alias = "DMA_CONTROL")]
pub type DmaControl = crate::Reg<dma_control::DmaControlSpec>;
#[doc = "DMA operations control register"]
pub mod dma_control;
#[doc = "TERMINATE (w) register accessor: Operation emergency terminate register (write of 0xD0 terminate current oeparation)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`terminate::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@terminate`] module"]
#[doc(alias = "TERMINATE")]
pub type Terminate = crate::Reg<terminate::TerminateSpec>;
#[doc = "Operation emergency terminate register (write of 0xD0 terminate current oeparation)"]
pub mod terminate;
#[doc = "IRQ (rw) register accessor: Interrupts pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq`] module"]
#[doc(alias = "IRQ")]
pub type Irq = crate::Reg<irq::IrqSpec>;
#[doc = "Interrupts pending register"]
pub mod irq;
#[doc = "TEXT_OUT"]
pub use self::text_out::TextOut;
#[doc = r"Cluster"]
#[doc = "TEXT_OUT"]
pub mod text_out;
#[doc = "STATUS (r) register accessor: Operation status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Operation status register"]
pub mod status;
#[doc = "CURRENT_DESCRIPTOR (r) register accessor: Register of address of current handling descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`current_descriptor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@current_descriptor`] module"]
#[doc(alias = "CURRENT_DESCRIPTOR")]
pub type CurrentDescriptor = crate::Reg<current_descriptor::CurrentDescriptorSpec>;
#[doc = "Register of address of current handling descriptor"]
pub mod current_descriptor;
#[doc = "NEXT_DESCRIPTOR (r) register accessor: Register of address of next descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`next_descriptor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@next_descriptor`] module"]
#[doc(alias = "NEXT_DESCRIPTOR")]
pub type NextDescriptor = crate::Reg<next_descriptor::NextDescriptorSpec>;
#[doc = "Register of address of next descriptor"]
pub mod next_descriptor;
