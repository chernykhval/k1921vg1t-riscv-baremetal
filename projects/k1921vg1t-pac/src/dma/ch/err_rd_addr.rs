#[doc = "Register `ERR_RD_ADDR` reader"]
pub type R = crate::R<ErrRdAddrSpec>;
#[doc = "Field `ERR_ADDR` reader - First encountered erroneous address in read transaction"]
pub type ErrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - First encountered erroneous address in read transaction"]
    #[inline(always)]
    pub fn err_addr(&self) -> ErrAddrR {
        ErrAddrR::new(self.bits)
    }
}
#[doc = "Erroneous read transaction address register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_rd_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrRdAddrSpec;
impl crate::RegisterSpec for ErrRdAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_rd_addr::R`](R) reader structure"]
impl crate::Readable for ErrRdAddrSpec {}
#[doc = "`reset()` method sets ERR_RD_ADDR to value 0"]
impl crate::Resettable for ErrRdAddrSpec {}
