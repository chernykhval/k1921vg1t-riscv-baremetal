#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cgcfgahb: Cgcfgahb,
    cgcfgapb0: Cgcfgapb0,
    cgcfgapb1: Cgcfgapb1,
    cgcfgapb2: Cgcfgapb2,
    rstdisahb: Rstdisahb,
    rstdisapb0: Rstdisapb0,
    rstdisapb1: Rstdisapb1,
    rstdisapb2: Rstdisapb2,
    sysclkcfg: Sysclkcfg,
    seccnt0: Seccnt0,
    seccnt1: Seccnt1,
    clkstat: Clkstat,
    rstsys: Rstsys,
    rststat: Rststat,
    borcfg: Borcfg,
    lock: Lock,
    inten: Inten,
    intstat: Intstat,
    hsecfg: Hsecfg,
    _reserved19: [u8; 0x04],
    uartcfg: [Uartcfg; 8],
    spicfg: [Spicfg; 8],
    adccfg: Adccfg,
    wdtcfg: Wdtcfg,
    clkoutcfg: Clkoutcfg,
    sdramcfg: Sdramcfg,
    i2scfg: [I2scfg; 2],
    _reserved26: [u8; 0x08],
    usbcfg: [Usbcfg; 2],
    ethcfg: Ethcfg,
    canfdcfg: Canfdcfg,
    flashcfg: Flashcfg,
    rtccfg: Rtccfg,
    _reserved31: [u8; 0x38],
    pll: [Pll; 3],
    pllstat: Pllstat,
}
impl RegisterBlock {
    #[doc = "0x00 - Clockgate Config AHB register"]
    #[inline(always)]
    pub const fn cgcfgahb(&self) -> &Cgcfgahb {
        &self.cgcfgahb
    }
    #[doc = "0x04 - Clockgate Config APB0 register"]
    #[inline(always)]
    pub const fn cgcfgapb0(&self) -> &Cgcfgapb0 {
        &self.cgcfgapb0
    }
    #[doc = "0x08 - Clockgate Config APB1 register"]
    #[inline(always)]
    pub const fn cgcfgapb1(&self) -> &Cgcfgapb1 {
        &self.cgcfgapb1
    }
    #[doc = "0x0c - Clockgate Config APB2 register"]
    #[inline(always)]
    pub const fn cgcfgapb2(&self) -> &Cgcfgapb2 {
        &self.cgcfgapb2
    }
    #[doc = "0x10 - Reset Disable AHB register"]
    #[inline(always)]
    pub const fn rstdisahb(&self) -> &Rstdisahb {
        &self.rstdisahb
    }
    #[doc = "0x14 - Reset Disable APB0 register"]
    #[inline(always)]
    pub const fn rstdisapb0(&self) -> &Rstdisapb0 {
        &self.rstdisapb0
    }
    #[doc = "0x18 - Reset Disable APB1 register"]
    #[inline(always)]
    pub const fn rstdisapb1(&self) -> &Rstdisapb1 {
        &self.rstdisapb1
    }
    #[doc = "0x1c - Reset Disable APB2 register"]
    #[inline(always)]
    pub const fn rstdisapb2(&self) -> &Rstdisapb2 {
        &self.rstdisapb2
    }
    #[doc = "0x20 - System clock configuration register"]
    #[inline(always)]
    pub const fn sysclkcfg(&self) -> &Sysclkcfg {
        &self.sysclkcfg
    }
    #[doc = "0x24 - Security counter configuration 0 register"]
    #[inline(always)]
    pub const fn seccnt0(&self) -> &Seccnt0 {
        &self.seccnt0
    }
    #[doc = "0x28 - Security counter configuration 1 register"]
    #[inline(always)]
    pub const fn seccnt1(&self) -> &Seccnt1 {
        &self.seccnt1
    }
    #[doc = "0x2c - System clock status register"]
    #[inline(always)]
    pub const fn clkstat(&self) -> &Clkstat {
        &self.clkstat
    }
    #[doc = "0x30 - System Reset Register"]
    #[inline(always)]
    pub const fn rstsys(&self) -> &Rstsys {
        &self.rstsys
    }
    #[doc = "0x34 - Reset Status Register"]
    #[inline(always)]
    pub const fn rststat(&self) -> &Rststat {
        &self.rststat
    }
    #[doc = "0x38 - Broun-Out Reset Configuration register"]
    #[inline(always)]
    pub const fn borcfg(&self) -> &Borcfg {
        &self.borcfg
    }
    #[doc = "0x3c - Lock configuration register"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x40 - Interrupt enable reigster"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x44 - Interrupt status register"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x48 - HSE Configuration Register"]
    #[inline(always)]
    pub const fn hsecfg(&self) -> &Hsecfg {
        &self.hsecfg
    }
    #[doc = "0x50..0x70 - UARTCFG"]
    #[inline(always)]
    pub const fn uartcfg(&self, n: usize) -> &Uartcfg {
        &self.uartcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x70 - UARTCFG"]
    #[inline(always)]
    pub fn uartcfg_iter(&self) -> impl Iterator<Item = &Uartcfg> {
        self.uartcfg.iter()
    }
    #[doc = "0x70..0x90 - SPICFG"]
    #[inline(always)]
    pub const fn spicfg(&self, n: usize) -> &Spicfg {
        &self.spicfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x90 - SPICFG"]
    #[inline(always)]
    pub fn spicfg_iter(&self) -> impl Iterator<Item = &Spicfg> {
        self.spicfg.iter()
    }
    #[doc = "0x90 - ADC Clock and Reset Configuration Register"]
    #[inline(always)]
    pub const fn adccfg(&self) -> &Adccfg {
        &self.adccfg
    }
    #[doc = "0x94 - WatchDog Configuration Register"]
    #[inline(always)]
    pub const fn wdtcfg(&self) -> &Wdtcfg {
        &self.wdtcfg
    }
    #[doc = "0x98 - Clockout Configuration Register"]
    #[inline(always)]
    pub const fn clkoutcfg(&self) -> &Clkoutcfg {
        &self.clkoutcfg
    }
    #[doc = "0x9c - SDRAM Clock and Reset Configuration Register"]
    #[inline(always)]
    pub const fn sdramcfg(&self) -> &Sdramcfg {
        &self.sdramcfg
    }
    #[doc = "0xa0..0xa8 - I2SCFG"]
    #[inline(always)]
    pub const fn i2scfg(&self, n: usize) -> &I2scfg {
        &self.i2scfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xa8 - I2SCFG"]
    #[inline(always)]
    pub fn i2scfg_iter(&self) -> impl Iterator<Item = &I2scfg> {
        self.i2scfg.iter()
    }
    #[doc = "0xb0..0xb8 - USBCFG"]
    #[inline(always)]
    pub const fn usbcfg(&self, n: usize) -> &Usbcfg {
        &self.usbcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb0..0xb8 - USBCFG"]
    #[inline(always)]
    pub fn usbcfg_iter(&self) -> impl Iterator<Item = &Usbcfg> {
        self.usbcfg.iter()
    }
    #[doc = "0xb8 - ETH Clock and Reset Configuration Register"]
    #[inline(always)]
    pub const fn ethcfg(&self) -> &Ethcfg {
        &self.ethcfg
    }
    #[doc = "0xbc - CANFD Clock and Reset Configuration Register"]
    #[inline(always)]
    pub const fn canfdcfg(&self) -> &Canfdcfg {
        &self.canfdcfg
    }
    #[doc = "0xc0 - Flash Clock Configuration register"]
    #[inline(always)]
    pub const fn flashcfg(&self) -> &Flashcfg {
        &self.flashcfg
    }
    #[doc = "0xc4 - RTC APB clock divider"]
    #[inline(always)]
    pub const fn rtccfg(&self) -> &Rtccfg {
        &self.rtccfg
    }
    #[doc = "0x100..0x130 - PLL"]
    #[inline(always)]
    pub const fn pll(&self, n: usize) -> &Pll {
        &self.pll[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x130 - PLL"]
    #[inline(always)]
    pub fn pll_iter(&self) -> impl Iterator<Item = &Pll> {
        self.pll.iter()
    }
    #[doc = "0x130 - PLLs status regisoter"]
    #[inline(always)]
    pub const fn pllstat(&self) -> &Pllstat {
        &self.pllstat
    }
}
#[doc = "CGCFGAHB (rw) register accessor: Clockgate Config AHB register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcfgahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcfgahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgcfgahb`] module"]
#[doc(alias = "CGCFGAHB")]
pub type Cgcfgahb = crate::Reg<cgcfgahb::CgcfgahbSpec>;
#[doc = "Clockgate Config AHB register"]
pub mod cgcfgahb;
#[doc = "CGCFGAPB0 (rw) register accessor: Clockgate Config APB0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcfgapb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcfgapb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgcfgapb0`] module"]
#[doc(alias = "CGCFGAPB0")]
pub type Cgcfgapb0 = crate::Reg<cgcfgapb0::Cgcfgapb0Spec>;
#[doc = "Clockgate Config APB0 register"]
pub mod cgcfgapb0;
#[doc = "CGCFGAPB1 (rw) register accessor: Clockgate Config APB1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcfgapb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcfgapb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgcfgapb1`] module"]
#[doc(alias = "CGCFGAPB1")]
pub type Cgcfgapb1 = crate::Reg<cgcfgapb1::Cgcfgapb1Spec>;
#[doc = "Clockgate Config APB1 register"]
pub mod cgcfgapb1;
#[doc = "CGCFGAPB2 (rw) register accessor: Clockgate Config APB2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgcfgapb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcfgapb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cgcfgapb2`] module"]
#[doc(alias = "CGCFGAPB2")]
pub type Cgcfgapb2 = crate::Reg<cgcfgapb2::Cgcfgapb2Spec>;
#[doc = "Clockgate Config APB2 register"]
pub mod cgcfgapb2;
#[doc = "RSTDISAHB (rw) register accessor: Reset Disable AHB register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstdisahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstdisahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstdisahb`] module"]
#[doc(alias = "RSTDISAHB")]
pub type Rstdisahb = crate::Reg<rstdisahb::RstdisahbSpec>;
#[doc = "Reset Disable AHB register"]
pub mod rstdisahb;
#[doc = "RSTDISAPB0 (rw) register accessor: Reset Disable APB0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstdisapb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstdisapb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstdisapb0`] module"]
#[doc(alias = "RSTDISAPB0")]
pub type Rstdisapb0 = crate::Reg<rstdisapb0::Rstdisapb0Spec>;
#[doc = "Reset Disable APB0 register"]
pub mod rstdisapb0;
#[doc = "RSTDISAPB1 (rw) register accessor: Reset Disable APB1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstdisapb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstdisapb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstdisapb1`] module"]
#[doc(alias = "RSTDISAPB1")]
pub type Rstdisapb1 = crate::Reg<rstdisapb1::Rstdisapb1Spec>;
#[doc = "Reset Disable APB1 register"]
pub mod rstdisapb1;
#[doc = "RSTDISAPB2 (rw) register accessor: Reset Disable APB2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`rstdisapb2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstdisapb2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstdisapb2`] module"]
#[doc(alias = "RSTDISAPB2")]
pub type Rstdisapb2 = crate::Reg<rstdisapb2::Rstdisapb2Spec>;
#[doc = "Reset Disable APB2 register"]
pub mod rstdisapb2;
#[doc = "SYSCLKCFG (rw) register accessor: System clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclkcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclkcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclkcfg`] module"]
#[doc(alias = "SYSCLKCFG")]
pub type Sysclkcfg = crate::Reg<sysclkcfg::SysclkcfgSpec>;
#[doc = "System clock configuration register"]
pub mod sysclkcfg;
#[doc = "SECCNT0 (rw) register accessor: Security counter configuration 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seccnt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccnt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccnt0`] module"]
#[doc(alias = "SECCNT0")]
pub type Seccnt0 = crate::Reg<seccnt0::Seccnt0Spec>;
#[doc = "Security counter configuration 0 register"]
pub mod seccnt0;
#[doc = "SECCNT1 (rw) register accessor: Security counter configuration 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`seccnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccnt1`] module"]
#[doc(alias = "SECCNT1")]
pub type Seccnt1 = crate::Reg<seccnt1::Seccnt1Spec>;
#[doc = "Security counter configuration 1 register"]
pub mod seccnt1;
#[doc = "CLKSTAT (r) register accessor: System clock status register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkstat`] module"]
#[doc(alias = "CLKSTAT")]
pub type Clkstat = crate::Reg<clkstat::ClkstatSpec>;
#[doc = "System clock status register"]
pub mod clkstat;
#[doc = "RSTSYS (w) register accessor: System Reset Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsys::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstsys`] module"]
#[doc(alias = "RSTSYS")]
pub type Rstsys = crate::Reg<rstsys::RstsysSpec>;
#[doc = "System Reset Register"]
pub mod rstsys;
#[doc = "RSTSTAT (rw) register accessor: Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rststat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rststat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rststat`] module"]
#[doc(alias = "RSTSTAT")]
pub type Rststat = crate::Reg<rststat::RststatSpec>;
#[doc = "Reset Status Register"]
pub mod rststat;
#[doc = "BORCFG (rw) register accessor: Broun-Out Reset Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`borcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`borcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@borcfg`] module"]
#[doc(alias = "BORCFG")]
pub type Borcfg = crate::Reg<borcfg::BorcfgSpec>;
#[doc = "Broun-Out Reset Configuration register"]
pub mod borcfg;
#[doc = "LOCK (rw) register accessor: Lock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Lock configuration register"]
pub mod lock;
#[doc = "INTEN (rw) register accessor: Interrupt enable reigster\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Interrupt enable reigster"]
pub mod inten;
#[doc = "INTSTAT (w) register accessor: Interrupt status register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intstat::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`] module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Interrupt status register"]
pub mod intstat;
#[doc = "HSECFG (rw) register accessor: HSE Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsecfg`] module"]
#[doc(alias = "HSECFG")]
pub type Hsecfg = crate::Reg<hsecfg::HsecfgSpec>;
#[doc = "HSE Configuration Register"]
pub mod hsecfg;
#[doc = "UARTCFG"]
pub use self::uartcfg::Uartcfg;
#[doc = r"Cluster"]
#[doc = "UARTCFG"]
pub mod uartcfg;
#[doc = "SPICFG"]
pub use self::spicfg::Spicfg;
#[doc = r"Cluster"]
#[doc = "SPICFG"]
pub mod spicfg;
#[doc = "ADCCFG (rw) register accessor: ADC Clock and Reset Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adccfg`] module"]
#[doc(alias = "ADCCFG")]
pub type Adccfg = crate::Reg<adccfg::AdccfgSpec>;
#[doc = "ADC Clock and Reset Configuration Register"]
pub mod adccfg;
#[doc = "WDTCFG (rw) register accessor: WatchDog Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wdtcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdtcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcfg`] module"]
#[doc(alias = "WDTCFG")]
pub type Wdtcfg = crate::Reg<wdtcfg::WdtcfgSpec>;
#[doc = "WatchDog Configuration Register"]
pub mod wdtcfg;
#[doc = "CLKOUTCFG (rw) register accessor: Clockout Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutcfg`] module"]
#[doc(alias = "CLKOUTCFG")]
pub type Clkoutcfg = crate::Reg<clkoutcfg::ClkoutcfgSpec>;
#[doc = "Clockout Configuration Register"]
pub mod clkoutcfg;
#[doc = "SDRAMCFG (rw) register accessor: SDRAM Clock and Reset Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdramcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdramcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdramcfg`] module"]
#[doc(alias = "SDRAMCFG")]
pub type Sdramcfg = crate::Reg<sdramcfg::SdramcfgSpec>;
#[doc = "SDRAM Clock and Reset Configuration Register"]
pub mod sdramcfg;
#[doc = "I2SCFG"]
pub use self::i2scfg::I2scfg;
#[doc = r"Cluster"]
#[doc = "I2SCFG"]
pub mod i2scfg;
#[doc = "USBCFG"]
pub use self::usbcfg::Usbcfg;
#[doc = r"Cluster"]
#[doc = "USBCFG"]
pub mod usbcfg;
#[doc = "ETHCFG (rw) register accessor: ETH Clock and Reset Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ethcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ethcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ethcfg`] module"]
#[doc(alias = "ETHCFG")]
pub type Ethcfg = crate::Reg<ethcfg::EthcfgSpec>;
#[doc = "ETH Clock and Reset Configuration Register"]
pub mod ethcfg;
#[doc = "CANFDCFG (rw) register accessor: CANFD Clock and Reset Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`canfdcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canfdcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canfdcfg`] module"]
#[doc(alias = "CANFDCFG")]
pub type Canfdcfg = crate::Reg<canfdcfg::CanfdcfgSpec>;
#[doc = "CANFD Clock and Reset Configuration Register"]
pub mod canfdcfg;
#[doc = "FLASHCFG (rw) register accessor: Flash Clock Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcfg`] module"]
#[doc(alias = "FLASHCFG")]
pub type Flashcfg = crate::Reg<flashcfg::FlashcfgSpec>;
#[doc = "Flash Clock Configuration register"]
pub mod flashcfg;
#[doc = "RTCCFG (rw) register accessor: RTC APB clock divider\n\nYou can [`read`](crate::Reg::read) this register and get [`rtccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccfg`] module"]
#[doc(alias = "RTCCFG")]
pub type Rtccfg = crate::Reg<rtccfg::RtccfgSpec>;
#[doc = "RTC APB clock divider"]
pub mod rtccfg;
#[doc = "PLL"]
pub use self::pll::Pll;
#[doc = r"Cluster"]
#[doc = "PLL"]
pub mod pll;
#[doc = "PLLSTAT (r) register accessor: PLLs status regisoter\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstat`] module"]
#[doc(alias = "PLLSTAT")]
pub type Pllstat = crate::Reg<pllstat::PllstatSpec>;
#[doc = "PLLs status regisoter"]
pub mod pllstat;
