#[repr(C)]
#[doc = "UARTCFG"]
#[doc(alias = "UARTCFG")]
pub struct Uartcfg {
    value: Value,
}
impl Uartcfg {
    #[doc = "0x00 - UART Clock and Reset Configuration Register"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}
#[doc = "VALUE (rw) register accessor: UART Clock and Reset Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "UART Clock and Reset Configuration Register"]
pub mod value;
