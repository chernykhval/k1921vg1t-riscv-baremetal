#[doc = "Register `NODE2ECNT` reader"]
pub type R = crate::R<Node2ecntSpec>;
#[doc = "Field `VALUE` reader - Number of errors in current node"]
pub type ValueR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Number of errors in current node"]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Node 2 error counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`node2ecnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Node2ecntSpec;
impl crate::RegisterSpec for Node2ecntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`node2ecnt::R`](R) reader structure"]
impl crate::Readable for Node2ecntSpec {}
#[doc = "`reset()` method sets NODE2ECNT to value 0"]
impl crate::Resettable for Node2ecntSpec {}
