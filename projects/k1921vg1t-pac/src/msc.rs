#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fdr: Fdr,
    io_ctrl: IoCtrl,
    u_ctrl: UCtrl,
    d_ctrl: DCtrl,
    d_data: DData,
    d_cmd: DCmd,
    flag_ctrl: FlagCtrl,
    ui_clr: UiClr,
    irq_en: IrqEn,
    irq_stat: IrqStat,
    d_stat: DStat,
    ui_stat: UiStat,
    u_data: UData,
    _reserved13: [u8; 0x0c],
    u_item: [UItem; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Fractional divider register"]
    #[inline(always)]
    pub const fn fdr(&self) -> &Fdr {
        &self.fdr
    }
    #[doc = "0x04 - Input/output Control Register"]
    #[inline(always)]
    pub const fn io_ctrl(&self) -> &IoCtrl {
        &self.io_ctrl
    }
    #[doc = "0x08 - Upstream Control/Status Register"]
    #[inline(always)]
    pub const fn u_ctrl(&self) -> &UCtrl {
        &self.u_ctrl
    }
    #[doc = "0x0c - Downstream Control Register"]
    #[inline(always)]
    pub const fn d_ctrl(&self) -> &DCtrl {
        &self.d_ctrl
    }
    #[doc = "0x10 - Downstream Data bits"]
    #[inline(always)]
    pub const fn d_data(&self) -> &DData {
        &self.d_data
    }
    #[doc = "0x14 - Downstream Command bits"]
    #[inline(always)]
    pub const fn d_cmd(&self) -> &DCmd {
        &self.d_cmd
    }
    #[doc = "0x18 - Flags control register (set/clear)"]
    #[inline(always)]
    pub const fn flag_ctrl(&self) -> &FlagCtrl {
        &self.flag_ctrl
    }
    #[doc = "0x1c - Upstream Items valid clear"]
    #[inline(always)]
    pub const fn ui_clr(&self) -> &UiClr {
        &self.ui_clr
    }
    #[doc = "0x20 - Interrupt requests enable register"]
    #[inline(always)]
    pub const fn irq_en(&self) -> &IrqEn {
        &self.irq_en
    }
    #[doc = "0x24 - Interrupt requests status register"]
    #[inline(always)]
    pub const fn irq_stat(&self) -> &IrqStat {
        &self.irq_stat
    }
    #[doc = "0x28 - Downstream status register"]
    #[inline(always)]
    pub const fn d_stat(&self) -> &DStat {
        &self.d_stat
    }
    #[doc = "0x2c - Upstream items status register"]
    #[inline(always)]
    pub const fn ui_stat(&self) -> &UiStat {
        &self.ui_stat
    }
    #[doc = "0x30 - Aggregated upstream items data"]
    #[inline(always)]
    pub const fn u_data(&self) -> &UData {
        &self.u_data
    }
    #[doc = "0x40..0x50 - U_ITEM"]
    #[inline(always)]
    pub const fn u_item(&self, n: usize) -> &UItem {
        &self.u_item[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - U_ITEM"]
    #[inline(always)]
    pub fn u_item_iter(&self) -> impl Iterator<Item = &UItem> {
        self.u_item.iter()
    }
}
#[doc = "FDR (rw) register accessor: Fractional divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`] module"]
#[doc(alias = "FDR")]
pub type Fdr = crate::Reg<fdr::FdrSpec>;
#[doc = "Fractional divider register"]
pub mod fdr;
#[doc = "IO_CTRL (rw) register accessor: Input/output Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@io_ctrl`] module"]
#[doc(alias = "IO_CTRL")]
pub type IoCtrl = crate::Reg<io_ctrl::IoCtrlSpec>;
#[doc = "Input/output Control Register"]
pub mod io_ctrl;
#[doc = "U_CTRL (rw) register accessor: Upstream Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`u_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_ctrl`] module"]
#[doc(alias = "U_CTRL")]
pub type UCtrl = crate::Reg<u_ctrl::UCtrlSpec>;
#[doc = "Upstream Control/Status Register"]
pub mod u_ctrl;
#[doc = "D_CTRL (rw) register accessor: Downstream Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_ctrl`] module"]
#[doc(alias = "D_CTRL")]
pub type DCtrl = crate::Reg<d_ctrl::DCtrlSpec>;
#[doc = "Downstream Control Register"]
pub mod d_ctrl;
#[doc = "D_DATA (rw) register accessor: Downstream Data bits\n\nYou can [`read`](crate::Reg::read) this register and get [`d_data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_data`] module"]
#[doc(alias = "D_DATA")]
pub type DData = crate::Reg<d_data::DDataSpec>;
#[doc = "Downstream Data bits"]
pub mod d_data;
#[doc = "D_CMD (rw) register accessor: Downstream Command bits\n\nYou can [`read`](crate::Reg::read) this register and get [`d_cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d_cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_cmd`] module"]
#[doc(alias = "D_CMD")]
pub type DCmd = crate::Reg<d_cmd::DCmdSpec>;
#[doc = "Downstream Command bits"]
pub mod d_cmd;
#[doc = "FLAG_CTRL (w) register accessor: Flags control register (set/clear)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flag_ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flag_ctrl`] module"]
#[doc(alias = "FLAG_CTRL")]
pub type FlagCtrl = crate::Reg<flag_ctrl::FlagCtrlSpec>;
#[doc = "Flags control register (set/clear)"]
pub mod flag_ctrl;
#[doc = "UI_CLR (w) register accessor: Upstream Items valid clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ui_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ui_clr`] module"]
#[doc(alias = "UI_CLR")]
pub type UiClr = crate::Reg<ui_clr::UiClrSpec>;
#[doc = "Upstream Items valid clear"]
pub mod ui_clr;
#[doc = "IRQ_EN (rw) register accessor: Interrupt requests enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_en`] module"]
#[doc(alias = "IRQ_EN")]
pub type IrqEn = crate::Reg<irq_en::IrqEnSpec>;
#[doc = "Interrupt requests enable register"]
pub mod irq_en;
#[doc = "IRQ_STAT (r) register accessor: Interrupt requests status register\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_stat`] module"]
#[doc(alias = "IRQ_STAT")]
pub type IrqStat = crate::Reg<irq_stat::IrqStatSpec>;
#[doc = "Interrupt requests status register"]
pub mod irq_stat;
#[doc = "D_STAT (r) register accessor: Downstream status register\n\nYou can [`read`](crate::Reg::read) this register and get [`d_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d_stat`] module"]
#[doc(alias = "D_STAT")]
pub type DStat = crate::Reg<d_stat::DStatSpec>;
#[doc = "Downstream status register"]
pub mod d_stat;
#[doc = "UI_STAT (r) register accessor: Upstream items status register\n\nYou can [`read`](crate::Reg::read) this register and get [`ui_stat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ui_stat`] module"]
#[doc(alias = "UI_STAT")]
pub type UiStat = crate::Reg<ui_stat::UiStatSpec>;
#[doc = "Upstream items status register"]
pub mod ui_stat;
#[doc = "U_DATA (r) register accessor: Aggregated upstream items data\n\nYou can [`read`](crate::Reg::read) this register and get [`u_data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@u_data`] module"]
#[doc(alias = "U_DATA")]
pub type UData = crate::Reg<u_data::UDataSpec>;
#[doc = "Aggregated upstream items data"]
pub mod u_data;
#[doc = "U_ITEM"]
pub use self::u_item::UItem;
#[doc = r"Cluster"]
#[doc = "U_ITEM"]
pub mod u_item;
