#[repr(C)]
#[doc = "CH"]
#[doc(alias = "CH")]
pub struct Ch {
    src_ptr: SrcPtr,
    dst_ptr: DstPtr,
    _reserved_2_ndtb: [u8; 0x04],
    config: Config,
    static0: Static0,
    static1: Static1,
    static2: Static2,
    static3: Static3,
    static4: Static4,
    restrict: Restrict,
    rdoffset: Rdoffset,
    wroffset: Wroffset,
    fifostat: Fifostat,
    cmd_outs: CmdOuts,
    _reserved14: [u8; 0x08],
    ch_active: ChActive,
    ch_start: ChStart,
    ch_status: ChStatus,
    count: Count,
    int_rawstat: IntRawstat,
    int_clear: IntClear,
    int_enable: IntEnable,
    int_status: IntStatus,
    err_stop: ErrStop,
    err_rd_addr: ErrRdAddr,
    err_wr_addr: ErrWrAddr,
    _reserved25: [u8; 0x10],
    reserved: Reserved,
}
impl Ch {
    #[doc = "0x00 - Command register 0 (source pointer)"]
    #[inline(always)]
    pub const fn src_ptr(&self) -> &SrcPtr {
        &self.src_ptr
    }
    #[doc = "0x04 - Command register 1 (destination pointer)"]
    #[inline(always)]
    pub const fn dst_ptr(&self) -> &DstPtr {
        &self.dst_ptr
    }
    #[doc = "0x08 - Command register 2 (number of data to transfer, block mode) - Not Implemented"]
    #[inline(always)]
    pub const fn ndtb(&self) -> &Ndtb {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Command register 2 (number of data to transfer, linear mode)"]
    #[inline(always)]
    pub const fn ndtl(&self) -> &Ndtl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Command register 3 (config)"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x10 - Static register 0"]
    #[inline(always)]
    pub const fn static0(&self) -> &Static0 {
        &self.static0
    }
    #[doc = "0x14 - Static register 1"]
    #[inline(always)]
    pub const fn static1(&self) -> &Static1 {
        &self.static1
    }
    #[doc = "0x18 - Static register 2"]
    #[inline(always)]
    pub const fn static2(&self) -> &Static2 {
        &self.static2
    }
    #[doc = "0x1c - Static register 3"]
    #[inline(always)]
    pub const fn static3(&self) -> &Static3 {
        &self.static3
    }
    #[doc = "0x20 - Static register 4"]
    #[inline(always)]
    pub const fn static4(&self) -> &Static4 {
        &self.static4
    }
    #[doc = "0x24 - Channels restrictions status register"]
    #[inline(always)]
    pub const fn restrict(&self) -> &Restrict {
        &self.restrict
    }
    #[doc = "0x28 - Channels read offset status register"]
    #[inline(always)]
    pub const fn rdoffset(&self) -> &Rdoffset {
        &self.rdoffset
    }
    #[doc = "0x2c - Channels write offset status register"]
    #[inline(always)]
    pub const fn wroffset(&self) -> &Wroffset {
        &self.wroffset
    }
    #[doc = "0x30 - FIFO fullness status register"]
    #[inline(always)]
    pub const fn fifostat(&self) -> &Fifostat {
        &self.fifostat
    }
    #[doc = "0x34 - Outstanding commands status register"]
    #[inline(always)]
    pub const fn cmd_outs(&self) -> &CmdOuts {
        &self.cmd_outs
    }
    #[doc = "0x40 - Channel active register"]
    #[inline(always)]
    pub const fn ch_active(&self) -> &ChActive {
        &self.ch_active
    }
    #[doc = "0x44 - Channel start register"]
    #[inline(always)]
    pub const fn ch_start(&self) -> &ChStart {
        &self.ch_start
    }
    #[doc = "0x48 - Channel status register"]
    #[inline(always)]
    pub const fn ch_status(&self) -> &ChStatus {
        &self.ch_status
    }
    #[doc = "0x4c - Buffer counter status register"]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x50 - Interrupt raw status register"]
    #[inline(always)]
    pub const fn int_rawstat(&self) -> &IntRawstat {
        &self.int_rawstat
    }
    #[doc = "0x54 - Interrupt clear register"]
    #[inline(always)]
    pub const fn int_clear(&self) -> &IntClear {
        &self.int_clear
    }
    #[doc = "0x58 - Interrupt enable register"]
    #[inline(always)]
    pub const fn int_enable(&self) -> &IntEnable {
        &self.int_enable
    }
    #[doc = "0x5c - Interrupt status"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x60 - Channel error stop condition register"]
    #[inline(always)]
    pub const fn err_stop(&self) -> &ErrStop {
        &self.err_stop
    }
    #[doc = "0x64 - Erroneous read transaction address register"]
    #[inline(always)]
    pub const fn err_rd_addr(&self) -> &ErrRdAddr {
        &self.err_rd_addr
    }
    #[doc = "0x68 - Erroneous write transaction address register"]
    #[inline(always)]
    pub const fn err_wr_addr(&self) -> &ErrWrAddr {
        &self.err_wr_addr
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn reserved(&self) -> &Reserved {
        &self.reserved
    }
}
#[doc = "SRC_PTR (rw) register accessor: Command register 0 (source pointer)\n\nYou can [`read`](crate::Reg::read) this register and get [`src_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`src_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_ptr`] module"]
#[doc(alias = "SRC_PTR")]
pub type SrcPtr = crate::Reg<src_ptr::SrcPtrSpec>;
#[doc = "Command register 0 (source pointer)"]
pub mod src_ptr;
#[doc = "DST_PTR (rw) register accessor: Command register 1 (destination pointer)\n\nYou can [`read`](crate::Reg::read) this register and get [`dst_ptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dst_ptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_ptr`] module"]
#[doc(alias = "DST_PTR")]
pub type DstPtr = crate::Reg<dst_ptr::DstPtrSpec>;
#[doc = "Command register 1 (destination pointer)"]
pub mod dst_ptr;
#[doc = "NDTL (rw) register accessor: Command register 2 (number of data to transfer, linear mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ndtl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndtl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndtl`] module"]
#[doc(alias = "NDTL")]
pub type Ndtl = crate::Reg<ndtl::NdtlSpec>;
#[doc = "Command register 2 (number of data to transfer, linear mode)"]
pub mod ndtl;
#[doc = "NDTB (r) register accessor: Command register 2 (number of data to transfer, block mode) - Not Implemented\n\nYou can [`read`](crate::Reg::read) this register and get [`ndtb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndtb`] module"]
#[doc(alias = "NDTB")]
pub type Ndtb = crate::Reg<ndtb::NdtbSpec>;
#[doc = "Command register 2 (number of data to transfer, block mode) - Not Implemented"]
pub mod ndtb;
#[doc = "CONFIG (rw) register accessor: Command register 3 (config)\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Command register 3 (config)"]
pub mod config;
#[doc = "STATIC0 (rw) register accessor: Static register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`static0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@static0`] module"]
#[doc(alias = "STATIC0")]
pub type Static0 = crate::Reg<static0::Static0Spec>;
#[doc = "Static register 0"]
pub mod static0;
#[doc = "STATIC1 (rw) register accessor: Static register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`static1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@static1`] module"]
#[doc(alias = "STATIC1")]
pub type Static1 = crate::Reg<static1::Static1Spec>;
#[doc = "Static register 1"]
pub mod static1;
#[doc = "STATIC2 (rw) register accessor: Static register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`static2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@static2`] module"]
#[doc(alias = "STATIC2")]
pub type Static2 = crate::Reg<static2::Static2Spec>;
#[doc = "Static register 2"]
pub mod static2;
#[doc = "STATIC3 (rw) register accessor: Static register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`static3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@static3`] module"]
#[doc(alias = "STATIC3")]
pub type Static3 = crate::Reg<static3::Static3Spec>;
#[doc = "Static register 3"]
pub mod static3;
#[doc = "STATIC4 (rw) register accessor: Static register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`static4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@static4`] module"]
#[doc(alias = "STATIC4")]
pub type Static4 = crate::Reg<static4::Static4Spec>;
#[doc = "Static register 4"]
pub mod static4;
#[doc = "RESTRICT (r) register accessor: Channels restrictions status register\n\nYou can [`read`](crate::Reg::read) this register and get [`restrict::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@restrict`] module"]
#[doc(alias = "RESTRICT")]
pub type Restrict = crate::Reg<restrict::RestrictSpec>;
#[doc = "Channels restrictions status register"]
pub mod restrict;
#[doc = "RDOFFSET (r) register accessor: Channels read offset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdoffset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdoffset`] module"]
#[doc(alias = "RDOFFSET")]
pub type Rdoffset = crate::Reg<rdoffset::RdoffsetSpec>;
#[doc = "Channels read offset status register"]
pub mod rdoffset;
#[doc = "WROFFSET (r) register accessor: Channels write offset status register\n\nYou can [`read`](crate::Reg::read) this register and get [`wroffset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wroffset`] module"]
#[doc(alias = "WROFFSET")]
pub type Wroffset = crate::Reg<wroffset::WroffsetSpec>;
#[doc = "Channels write offset status register"]
pub mod wroffset;
#[doc = "FIFOSTAT (r) register accessor: FIFO fullness status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifostat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifostat`] module"]
#[doc(alias = "FIFOSTAT")]
pub type Fifostat = crate::Reg<fifostat::FifostatSpec>;
#[doc = "FIFO fullness status register"]
pub mod fifostat;
#[doc = "CMD_OUTS (r) register accessor: Outstanding commands status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_outs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_outs`] module"]
#[doc(alias = "CMD_OUTS")]
pub type CmdOuts = crate::Reg<cmd_outs::CmdOutsSpec>;
#[doc = "Outstanding commands status register"]
pub mod cmd_outs;
#[doc = "CH_ACTIVE (rw) register accessor: Channel active register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_active::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_active::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_active`] module"]
#[doc(alias = "CH_ACTIVE")]
pub type ChActive = crate::Reg<ch_active::ChActiveSpec>;
#[doc = "Channel active register"]
pub mod ch_active;
#[doc = "CH_START (w) register accessor: Channel start register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_start::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_start`] module"]
#[doc(alias = "CH_START")]
pub type ChStart = crate::Reg<ch_start::ChStartSpec>;
#[doc = "Channel start register"]
pub mod ch_start;
#[doc = "CH_STATUS (r) register accessor: Channel status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch_status`] module"]
#[doc(alias = "CH_STATUS")]
pub type ChStatus = crate::Reg<ch_status::ChStatusSpec>;
#[doc = "Channel status register"]
pub mod ch_status;
#[doc = "COUNT (r) register accessor: Buffer counter status register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`] module"]
#[doc(alias = "COUNT")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "Buffer counter status register"]
pub mod count;
#[doc = "INT_RAWSTAT (rw) register accessor: Interrupt raw status register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_rawstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_rawstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_rawstat`] module"]
#[doc(alias = "INT_RAWSTAT")]
pub type IntRawstat = crate::Reg<int_rawstat::IntRawstatSpec>;
#[doc = "Interrupt raw status register"]
pub mod int_rawstat;
#[doc = "INT_CLEAR (w) register accessor: Interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clear`] module"]
#[doc(alias = "INT_CLEAR")]
pub type IntClear = crate::Reg<int_clear::IntClearSpec>;
#[doc = "Interrupt clear register"]
pub mod int_clear;
#[doc = "INT_ENABLE (rw) register accessor: Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_enable`] module"]
#[doc(alias = "INT_ENABLE")]
pub type IntEnable = crate::Reg<int_enable::IntEnableSpec>;
#[doc = "Interrupt enable register"]
pub mod int_enable;
#[doc = "INT_STATUS (r) register accessor: Interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`] module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt status"]
pub mod int_status;
#[doc = "ERR_STOP (rw) register accessor: Channel error stop condition register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`err_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_stop`] module"]
#[doc(alias = "ERR_STOP")]
pub type ErrStop = crate::Reg<err_stop::ErrStopSpec>;
#[doc = "Channel error stop condition register"]
pub mod err_stop;
#[doc = "ERR_RD_ADDR (r) register accessor: Erroneous read transaction address register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_rd_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_rd_addr`] module"]
#[doc(alias = "ERR_RD_ADDR")]
pub type ErrRdAddr = crate::Reg<err_rd_addr::ErrRdAddrSpec>;
#[doc = "Erroneous read transaction address register"]
pub mod err_rd_addr;
#[doc = "ERR_WR_ADDR (r) register accessor: Erroneous write transaction address register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_wr_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@err_wr_addr`] module"]
#[doc(alias = "ERR_WR_ADDR")]
pub type ErrWrAddr = crate::Reg<err_wr_addr::ErrWrAddrSpec>;
#[doc = "Erroneous write transaction address register"]
pub mod err_wr_addr;
#[doc = "Reserved (rw) register accessor: \n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved`] module"]
pub type Reserved = crate::Reg<reserved::ReservedSpec>;
#[doc = ""]
pub mod reserved;
