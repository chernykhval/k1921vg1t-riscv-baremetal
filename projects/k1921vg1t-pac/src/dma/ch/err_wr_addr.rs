#[doc = "Register `ERR_WR_ADDR` reader"]
pub type R = crate::R<ErrWrAddrSpec>;
#[doc = "Field `ERR_ADDR` reader - First encountered erroneous address in write transaction"]
pub type ErrAddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - First encountered erroneous address in write transaction"]
    #[inline(always)]
    pub fn err_addr(&self) -> ErrAddrR {
        ErrAddrR::new(self.bits)
    }
}
#[doc = "Erroneous write transaction address register\n\nYou can [`read`](crate::Reg::read) this register and get [`err_wr_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrWrAddrSpec;
impl crate::RegisterSpec for ErrWrAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`err_wr_addr::R`](R) reader structure"]
impl crate::Readable for ErrWrAddrSpec {}
#[doc = "`reset()` method sets ERR_WR_ADDR to value 0"]
impl crate::Resettable for ErrWrAddrSpec {}
