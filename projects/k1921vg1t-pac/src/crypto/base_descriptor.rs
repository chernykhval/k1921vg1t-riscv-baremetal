#[doc = "Register `BASE_DESCRIPTOR` reader"]
pub type R = crate::R<BaseDescriptorSpec>;
#[doc = "Register `BASE_DESCRIPTOR` writer"]
pub type W = crate::W<BaseDescriptorSpec>;
#[doc = "Field `ZEROES` reader - "]
pub type ZeroesR = crate::FieldReader;
#[doc = "Field `ADDRESS` reader - "]
pub type AddressR = crate::FieldReader<u32>;
#[doc = "Field `ADDRESS` writer - "]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
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
impl W {
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, BaseDescriptorSpec> {
        AddressW::new(self, 4)
    }
}
#[doc = "Register of address of base descriptor\n\nYou can [`read`](crate::Reg::read) this register and get [`base_descriptor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`base_descriptor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseDescriptorSpec;
impl crate::RegisterSpec for BaseDescriptorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base_descriptor::R`](R) reader structure"]
impl crate::Readable for BaseDescriptorSpec {}
#[doc = "`write(|w| ..)` method takes [`base_descriptor::W`](W) writer structure"]
impl crate::Writable for BaseDescriptorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BASE_DESCRIPTOR to value 0"]
impl crate::Resettable for BaseDescriptorSpec {}
