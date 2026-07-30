#[doc = "Register `NODE4ECNT` reader"]
pub type R = crate::R<Node4ecntSpec>;
#[doc = "Field `VALUE` reader - Number of errors in current node"]
pub type ValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Number of errors in current node"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Node 4 error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`node4ecnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Node4ecntSpec;
impl crate::RegisterSpec for Node4ecntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`node4ecnt::R`](R) reader structure"]
impl crate::Readable for Node4ecntSpec {}
#[doc = "`reset()` method sets NODE4ECNT to value 0"]
impl crate::Resettable for Node4ecntSpec {}
