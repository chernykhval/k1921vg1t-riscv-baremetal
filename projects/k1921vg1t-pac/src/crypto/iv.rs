#[repr(C)]
#[doc = "IV"]
#[doc(alias = "IV")]
pub struct Iv {
    value: Value,
}
impl Iv {
    #[doc = "0x00 - Initialization vector register 0"]
    #[inline(always)]
    pub const fn value(&self) -> &Value {
        &self.value
    }
}
#[doc = "VALUE (rw) register accessor: Initialization vector register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@value`] module"]
#[doc(alias = "VALUE")]
pub type Value = crate::Reg<value::ValueSpec>;
#[doc = "Initialization vector register 0"]
pub mod value;
