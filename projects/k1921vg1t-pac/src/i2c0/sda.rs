#[doc = "Register `SDA` reader"]
pub type R = crate::R<SdaSpec>;
#[doc = "Register `SDA` writer"]
pub type W = crate::W<SdaSpec>;
#[doc = "Field `DATA` reader - Data field"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Data field"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data field"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data field"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, SdaSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`sda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdaSpec;
impl crate::RegisterSpec for SdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda::R`](R) reader structure"]
impl crate::Readable for SdaSpec {}
#[doc = "`write(|w| ..)` method takes [`sda::W`](W) writer structure"]
impl crate::Writable for SdaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDA to value 0"]
impl crate::Resettable for SdaSpec {}
