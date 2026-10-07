#[repr(C)]
#[doc = "FIFO"]
#[doc(alias = "FIFO")]
pub struct Fifo {
    value: Value,
}
impl Fifo {
    #[doc = "0x00 - FIFO register"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}
#[doc = "VALUE (r) register accessor: FIFO register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "FIFO register"]
pub mod value;
