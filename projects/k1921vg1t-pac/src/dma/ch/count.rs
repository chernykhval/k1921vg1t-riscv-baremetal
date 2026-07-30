#[doc = "Register `COUNT` reader"]
pub type R = crate::R<CountSpec>;
#[doc = "Field `BUFF_COUNT` reader - Number of buffers transferred by channel since started"]
pub type BuffCountR = crate::FieldReader<u16>;
#[doc = "Field `INT_COUNT` reader - Number of unserviced end interrupts"]
pub type IntCountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Number of buffers transferred by channel since started"]
    #[inline(always)]
    pub fn buff_count(&self) -> BuffCountR {
        BuffCountR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:20 - Number of unserviced end interrupts"]
    #[inline(always)]
    pub fn int_count(&self) -> IntCountR {
        IntCountR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Buffer counter status register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountSpec;
impl crate::RegisterSpec for CountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count::R`](R) reader structure"]
impl crate::Readable for CountSpec {}
#[doc = "`reset()` method sets COUNT to value 0"]
impl crate::Resettable for CountSpec {}
