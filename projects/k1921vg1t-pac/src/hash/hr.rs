#[repr(C)]
#[doc = "HR"]
#[doc(alias = "HR")]
pub struct Hr {
    value: Value,
}
impl Hr {
    #[doc = "0x00 - Hash digest register"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}
#[doc = "VALUE (r) register accessor: Hash digest register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Hash digest register"]
pub mod value;
