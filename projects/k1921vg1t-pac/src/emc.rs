#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wincfg: [Wincfg; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - WINCFG"]
    #[inline(always)]
    pub const fn wincfg(&self, n: usize) -> &Wincfg {
        &self.wincfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - WINCFG"]
    #[inline(always)]
    pub fn wincfg_iter(&self) -> impl Iterator<Item = &Wincfg> {
        self.wincfg.iter()
    }
}
#[doc = "WINCFG"]
pub use self::wincfg::Wincfg;
#[doc = r"Cluster"]
#[doc = "WINCFG"]
pub mod wincfg;
