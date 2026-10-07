#[repr(C)]
#[doc = "MSID"]
#[doc(alias = "MSID")]
pub struct Msid {
    value: Value,
}
impl Msid {
    #[doc = "0x00 - Register messages index0"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}
#[doc = "VALUE (r) register accessor: Register messages index0\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Register messages index0"]
pub mod value;
