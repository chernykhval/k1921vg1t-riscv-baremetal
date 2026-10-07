#[repr(C)]
#[doc = "Msg"]
pub struct Msg {
    mofcr: Mofcr,
    mofgpr: Mofgpr,
    moipr: Moipr,
    moamr: Moamr,
    modatal: Modatal,
    modatah: Modatah,
    moar: Moar,
    _reserved_7_moctr: [u8; 0x04],
}
impl Msg {
    #[doc = "0x00 - Register control the operation of the message object 0"]
    #[inline(always)]
    pub const fn mofcr(&self) -> &Mofcr {
        &self.mofcr
    }
    #[doc = "0x04 - Pointer register FIFO / gateway message object 0"]
    #[inline(always)]
    pub const fn mofgpr(&self) -> &Mofgpr {
        &self.mofgpr
    }
    #[doc = "0x08 - Pointer register interrupt message object 0"]
    #[inline(always)]
    pub const fn moipr(&self) -> &Moipr {
        &self.moipr
    }
    #[doc = "0x0c - Mask register message object 0"]
    #[inline(always)]
    pub const fn moamr(&self) -> &Moamr {
        &self.moamr
    }
    #[doc = "0x10 - Low data registers of the message object 0"]
    #[inline(always)]
    pub const fn modatal(&self) -> &Modatal {
        &self.modatal
    }
    #[doc = "0x14 - High data registers of the message object 0"]
    #[inline(always)]
    pub const fn modatah(&self) -> &Modatah {
        &self.modatah
    }
    #[doc = "0x18 - Register arbitration message object 0"]
    #[inline(always)]
    pub const fn moar(&self) -> &Moar {
        &self.moar
    }
    #[doc = "0x1c - Status register of the message object 0"]
    #[inline(always)]
    pub const fn mostat(&self) -> &Mostat {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
    #[doc = "0x1c - Control register Message object 0"]
    #[inline(always)]
    pub const fn moctr(&self) -> &Moctr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(28).cast() }
    }
}
#[doc = "MOFCR (rw) register accessor: Register control the operation of the message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mofcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mofcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mofcr`] module"]
#[doc(alias = "MOFCR")]
pub type Mofcr = crate::Reg<mofcr::MofcrSpec>;
#[doc = "Register control the operation of the message object 0"]
pub mod mofcr;
#[doc = "MOFGPR (rw) register accessor: Pointer register FIFO / gateway message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mofgpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mofgpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mofgpr`] module"]
#[doc(alias = "MOFGPR")]
pub type Mofgpr = crate::Reg<mofgpr::MofgprSpec>;
#[doc = "Pointer register FIFO / gateway message object 0"]
pub mod mofgpr;
#[doc = "MOIPR (rw) register accessor: Pointer register interrupt message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`moipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moipr`] module"]
#[doc(alias = "MOIPR")]
pub type Moipr = crate::Reg<moipr::MoiprSpec>;
#[doc = "Pointer register interrupt message object 0"]
pub mod moipr;
#[doc = "MOAMR (rw) register accessor: Mask register message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`moamr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moamr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moamr`] module"]
#[doc(alias = "MOAMR")]
pub type Moamr = crate::Reg<moamr::MoamrSpec>;
#[doc = "Mask register message object 0"]
pub mod moamr;
#[doc = "MODATAL (rw) register accessor: Low data registers of the message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`modatal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modatal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modatal`] module"]
#[doc(alias = "MODATAL")]
pub type Modatal = crate::Reg<modatal::ModatalSpec>;
#[doc = "Low data registers of the message object 0"]
pub mod modatal;
#[doc = "MODATAH (rw) register accessor: High data registers of the message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`modatah::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modatah::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modatah`] module"]
#[doc(alias = "MODATAH")]
pub type Modatah = crate::Reg<modatah::ModatahSpec>;
#[doc = "High data registers of the message object 0"]
pub mod modatah;
#[doc = "MOAR (rw) register accessor: Register arbitration message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`moar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moar`] module"]
#[doc(alias = "MOAR")]
pub type Moar = crate::Reg<moar::MoarSpec>;
#[doc = "Register arbitration message object 0"]
pub mod moar;
#[doc = "MOCTR (w) register accessor: Control register Message object 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moctr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moctr`] module"]
#[doc(alias = "MOCTR")]
pub type Moctr = crate::Reg<moctr::MoctrSpec>;
#[doc = "Control register Message object 0"]
pub mod moctr;
#[doc = "MOSTAT (r) register accessor: Status register of the message object 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mostat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mostat`] module"]
#[doc(alias = "MOSTAT")]
pub type Mostat = crate::Reg<mostat::MostatSpec>;
#[doc = "Status register of the message object 0"]
pub mod mostat;
