#[doc = "Register `CSRX` reader"]
pub type R = crate::R<CsrxSpec>;
#[doc = "Field `CSUMRX` reader - Value of the accepted checksum"]
pub type CsumrxR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Value of the accepted checksum"]
    #[inline(always)]
    pub fn csumrx(&self) -> CsumrxR {
        CsumrxR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Register of the received checksum\n\nYou can [`read`](crate::Reg::read) this register and get [`csrx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrxSpec;
impl crate::RegisterSpec for CsrxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csrx::R`](R) reader structure"]
impl crate::Readable for CsrxSpec {}
#[doc = "`reset()` method sets CSRX to value 0"]
impl crate::Resettable for CsrxSpec {}
