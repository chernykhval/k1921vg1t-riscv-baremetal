#[doc = "Register `CURRENT_DESCRIPTOR` reader"]
pub type R = crate::R<CurrentDescriptorSpec>;
#[doc = "Field `ZEROES` reader - "]
pub type ZeroesR = crate::FieldReader;
#[doc = "Field `ADDRESS` reader - "]
pub type AddressR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn zeroes(&self) -> ZeroesR {
        ZeroesR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
#[doc = "Register of address of current handling descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`current_descriptor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentDescriptorSpec;
impl crate::RegisterSpec for CurrentDescriptorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`current_descriptor::R`](R) reader structure"]
impl crate::Readable for CurrentDescriptorSpec {}
#[doc = "`reset()` method sets CURRENT_DESCRIPTOR to value 0"]
impl crate::Resettable for CurrentDescriptorSpec {}
