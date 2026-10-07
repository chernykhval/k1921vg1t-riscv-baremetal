#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    control: Control,
    status: Status,
    if_: If,
    ie: Ie,
    moben: Moben,
    mobie: Mobie,
    mobif: Mobif,
    bittiming: Bittiming,
    timerctrl: Timerctrl,
    timerval: Timerval,
    ttcval: Ttcval,
    trec: Trec,
    hprimob: Hprimob,
    _reserved13: [u8; 0x04],
    fd_ctrl: FdCtrl,
    mobfden: Mobfden,
    fdbittiming: Fdbittiming,
    _reserved16: [u8; 0x24],
    mobpage: Mobpage,
    timestamp: Timestamp,
    mobstatus: Mobstatus,
    mobctrl: Mobctrl,
    idtag: Idtag,
    idmask: Idmask,
    message: [Message; 16],
}
impl RegisterBlock {
    #[doc = "0x40 - CAN General Control Register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x44 - CAN General Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x48 - CAN General Interrupt Register"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x4c - CAN General Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x50 - CAN Enable MOb Register"]
    #[inline(always)]
    pub const fn moben(&self) -> &Moben {
        &self.moben
    }
    #[doc = "0x54 - CAN Enable Interrupt MOb Register"]
    #[inline(always)]
    pub const fn mobie(&self) -> &Mobie {
        &self.mobie
    }
    #[doc = "0x58 - CAN Status Interrupt MOb Register"]
    #[inline(always)]
    pub const fn mobif(&self) -> &Mobif {
        &self.mobif
    }
    #[doc = "0x5c - CAN Bit Timing Register"]
    #[inline(always)]
    pub const fn bittiming(&self) -> &Bittiming {
        &self.bittiming
    }
    #[doc = "0x60 - CAN Timer Control Register"]
    #[inline(always)]
    pub const fn timerctrl(&self) -> &Timerctrl {
        &self.timerctrl
    }
    #[doc = "0x64 - CAN Timer Register"]
    #[inline(always)]
    pub const fn timerval(&self) -> &Timerval {
        &self.timerval
    }
    #[doc = "0x68 - CAN TTC Timer Register"]
    #[inline(always)]
    pub const fn ttcval(&self) -> &Ttcval {
        &self.ttcval
    }
    #[doc = "0x6c - CAN Transmit Error Counter Register"]
    #[inline(always)]
    pub const fn trec(&self) -> &Trec {
        &self.trec
    }
    #[doc = "0x70 - CAN Highest Priority MOb Register"]
    #[inline(always)]
    pub const fn hprimob(&self) -> &Hprimob {
        &self.hprimob
    }
    #[doc = "0x78 - CAN FD Mode Register"]
    #[inline(always)]
    pub const fn fd_ctrl(&self) -> &FdCtrl {
        &self.fd_ctrl
    }
    #[doc = "0x7c - CAN MOb FD Mode Enable Register"]
    #[inline(always)]
    pub const fn mobfden(&self) -> &Mobfden {
        &self.mobfden
    }
    #[doc = "0x80 - CAN FD Bit Timing Register"]
    #[inline(always)]
    pub const fn fdbittiming(&self) -> &Fdbittiming {
        &self.fdbittiming
    }
    #[doc = "0xa8 - CAN Page MOb Register"]
    #[inline(always)]
    pub const fn mobpage(&self) -> &Mobpage {
        &self.mobpage
    }
    #[doc = "0xac - CAN Timestamp Register"]
    #[inline(always)]
    pub const fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
    #[doc = "0xb0 - CAN MOb Status Register"]
    #[inline(always)]
    pub const fn mobstatus(&self) -> &Mobstatus {
        &self.mobstatus
    }
    #[doc = "0xb4 - CAN MOb Control and DLC Register"]
    #[inline(always)]
    pub const fn mobctrl(&self) -> &Mobctrl {
        &self.mobctrl
    }
    #[doc = "0xb8 - CAN Identifier Tag Register"]
    #[inline(always)]
    pub const fn idtag(&self) -> &Idtag {
        &self.idtag
    }
    #[doc = "0xbc - CAN Identifier Mask Register"]
    #[inline(always)]
    pub const fn idmask(&self) -> &Idmask {
        &self.idmask
    }
    #[doc = "0xc0..0x100 - MESSAGE"]
    #[inline(always)]
    pub const fn message(&self, n: usize) -> &Message {
        &self.message[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xc0..0x100 - MESSAGE"]
    #[inline(always)]
    pub fn message_iter(&self) -> impl Iterator<Item = &Message> {
        self.message.iter()
    }
}
#[doc = "CONTROL (rw) register accessor: CAN General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "CAN General Control Register"]
pub mod control;
#[doc = "STATUS (r) register accessor: CAN General Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "CAN General Status Register"]
pub mod status;
#[doc = "IF (rw) register accessor: CAN General Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "CAN General Interrupt Register"]
pub mod if_;
#[doc = "IE (rw) register accessor: CAN General Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`] module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "CAN General Interrupt Enable Register"]
pub mod ie;
#[doc = "MOBEN (r) register accessor: CAN Enable MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`moben::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moben`] module"]
#[doc(alias = "MOBEN")]
pub type Moben = crate::Reg<moben::MobenSpec>;
#[doc = "CAN Enable MOb Register"]
pub mod moben;
#[doc = "MOBIE (rw) register accessor: CAN Enable Interrupt MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mobie`] module"]
#[doc(alias = "MOBIE")]
pub type Mobie = crate::Reg<mobie::MobieSpec>;
#[doc = "CAN Enable Interrupt MOb Register"]
pub mod mobie;
#[doc = "MOBIF (r) register accessor: CAN Status Interrupt MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobif::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mobif`] module"]
#[doc(alias = "MOBIF")]
pub type Mobif = crate::Reg<mobif::MobifSpec>;
#[doc = "CAN Status Interrupt MOb Register"]
pub mod mobif;
#[doc = "BITTIMING (rw) register accessor: CAN Bit Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bittiming::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bittiming::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bittiming`] module"]
#[doc(alias = "BITTIMING")]
pub type Bittiming = crate::Reg<bittiming::BittimingSpec>;
#[doc = "CAN Bit Timing Register"]
pub mod bittiming;
#[doc = "TIMERCTRL (rw) register accessor: CAN Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timerctrl`] module"]
#[doc(alias = "TIMERCTRL")]
pub type Timerctrl = crate::Reg<timerctrl::TimerctrlSpec>;
#[doc = "CAN Timer Control Register"]
pub mod timerctrl;
#[doc = "TIMERVAL (r) register accessor: CAN Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerval::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timerval`] module"]
#[doc(alias = "TIMERVAL")]
pub type Timerval = crate::Reg<timerval::TimervalSpec>;
#[doc = "CAN Timer Register"]
pub mod timerval;
#[doc = "TTCVAL (r) register accessor: CAN TTC Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ttcval::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttcval`] module"]
#[doc(alias = "TTCVAL")]
pub type Ttcval = crate::Reg<ttcval::TtcvalSpec>;
#[doc = "CAN TTC Timer Register"]
pub mod ttcval;
#[doc = "TREC (r) register accessor: CAN Transmit Error Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`trec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trec`] module"]
#[doc(alias = "TREC")]
pub type Trec = crate::Reg<trec::TrecSpec>;
#[doc = "CAN Transmit Error Counter Register"]
pub mod trec;
#[doc = "HPRIMOB (rw) register accessor: CAN Highest Priority MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hprimob::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprimob::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprimob`] module"]
#[doc(alias = "HPRIMOB")]
pub type Hprimob = crate::Reg<hprimob::HprimobSpec>;
#[doc = "CAN Highest Priority MOb Register"]
pub mod hprimob;
#[doc = "FD_CTRL (rw) register accessor: CAN FD Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fd_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fd_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fd_ctrl`] module"]
#[doc(alias = "FD_CTRL")]
pub type FdCtrl = crate::Reg<fd_ctrl::FdCtrlSpec>;
#[doc = "CAN FD Mode Register"]
pub mod fd_ctrl;
#[doc = "MOBFDEN (rw) register accessor: CAN MOb FD Mode Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobfden::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobfden::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mobfden`] module"]
#[doc(alias = "MOBFDEN")]
pub type Mobfden = crate::Reg<mobfden::MobfdenSpec>;
#[doc = "CAN MOb FD Mode Enable Register"]
pub mod mobfden;
#[doc = "FDBITTIMING (rw) register accessor: CAN FD Bit Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdbittiming::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdbittiming::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdbittiming`] module"]
#[doc(alias = "FDBITTIMING")]
pub type Fdbittiming = crate::Reg<fdbittiming::FdbittimingSpec>;
#[doc = "CAN FD Bit Timing Register"]
pub mod fdbittiming;
#[doc = "MOBPAGE (rw) register accessor: CAN Page MOb Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobpage::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobpage::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mobpage`] module"]
#[doc(alias = "MOBPAGE")]
pub type Mobpage = crate::Reg<mobpage::MobpageSpec>;
#[doc = "CAN Page MOb Register"]
pub mod mobpage;
#[doc = "TIMESTAMP (r) register accessor: CAN Timestamp Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timestamp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp`] module"]
#[doc(alias = "TIMESTAMP")]
pub type Timestamp = crate::Reg<timestamp::TimestampSpec>;
#[doc = "CAN Timestamp Register"]
pub mod timestamp;
#[doc = "MOBSTATUS (rw) register accessor: CAN MOb Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mobstatus`] module"]
#[doc(alias = "MOBSTATUS")]
pub type Mobstatus = crate::Reg<mobstatus::MobstatusSpec>;
#[doc = "CAN MOb Status Register"]
pub mod mobstatus;
#[doc = "MOBCTRL (rw) register accessor: CAN MOb Control and DLC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mobctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mobctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mobctrl`] module"]
#[doc(alias = "MOBCTRL")]
pub type Mobctrl = crate::Reg<mobctrl::MobctrlSpec>;
#[doc = "CAN MOb Control and DLC Register"]
pub mod mobctrl;
#[doc = "IDTAG (rw) register accessor: CAN Identifier Tag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idtag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idtag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idtag`] module"]
#[doc(alias = "IDTAG")]
pub type Idtag = crate::Reg<idtag::IdtagSpec>;
#[doc = "CAN Identifier Tag Register"]
pub mod idtag;
#[doc = "IDMASK (rw) register accessor: CAN Identifier Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idmask`] module"]
#[doc(alias = "IDMASK")]
pub type Idmask = crate::Reg<idmask::IdmaskSpec>;
#[doc = "CAN Identifier Mask Register"]
pub mod idmask;
#[doc = "MESSAGE"]
pub use self::message::Message;
#[doc = r"Cluster"]
#[doc = "MESSAGE"]
pub mod message;
