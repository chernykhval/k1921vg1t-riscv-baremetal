#[doc = "Register `NDTB` reader"]
pub type R = crate::R<NdtbSpec>;
#[doc = "Field `X_SIZE` reader - Number of block lines - Not Implemented"]
pub type XSizeR = crate::FieldReader;
#[doc = "Field `Y_SIZE` reader - Number of block lines - Not Implemented"]
pub type YSizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Number of block lines - Not Implemented"]
    #[inline(always)]
    pub fn x_size(&self) -> XSizeR {
        XSizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Number of block lines - Not Implemented"]
    #[inline(always)]
    pub fn y_size(&self) -> YSizeR {
        YSizeR::new(((self.bits >> 8) & 3) as u8)
    }
}
#[doc = "Command register 2 (number of data to transfer, block mode) - Not Implemented\n\nYou can [`read`](crate::Reg::read) this register and get [`ndtb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NdtbSpec;
impl crate::RegisterSpec for NdtbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndtb::R`](R) reader structure"]
impl crate::Readable for NdtbSpec {}
#[doc = "`reset()` method sets NDTB to value 0"]
impl crate::Resettable for NdtbSpec {}
