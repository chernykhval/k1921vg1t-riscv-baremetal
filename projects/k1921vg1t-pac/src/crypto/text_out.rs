#[repr(C)]
#[doc = "TEXT_OUT"]
#[doc(alias = "TEXT_OUT")]
pub struct TextOut {
    value: Value,
}
impl TextOut {
    #[doc = "0x00 - Output text regiter 0 (cipher text / plain text)"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}
#[doc = "VALUE (r) register accessor: Output text regiter 0 (cipher text / plain text)\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Output text regiter 0 (cipher text / plain text)"]
pub mod value;
