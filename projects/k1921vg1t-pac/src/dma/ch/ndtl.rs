#[doc = "Register `NDTL` reader"]
pub type R = crate::R<NdtlSpec>;
#[doc = "Register `NDTL` writer"]
pub type W = crate::W<NdtlSpec>;
#[doc = "Field `BUFFER_SIZE` reader - Size of buffer to transfer"]
pub type BufferSizeR = crate::FieldReader<u16>;
#[doc = "Field `BUFFER_SIZE` writer - Size of buffer to transfer"]
pub type BufferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Size of buffer to transfer"]
    #[inline(always)]
    pub fn buffer_size(&self) -> BufferSizeR {
        BufferSizeR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Size of buffer to transfer"]
    #[inline(always)]
    pub fn buffer_size(&mut self) -> BufferSizeW<'_, NdtlSpec> {
        BufferSizeW::new(self, 0)
    }
}
#[doc = "Command register 2 (number of data to transfer, linear mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ndtl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndtl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NdtlSpec;
impl crate::RegisterSpec for NdtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ndtl::R`](R) reader structure"]
impl crate::Readable for NdtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ndtl::W`](W) writer structure"]
impl crate::Writable for NdtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NDTL to value 0"]
impl crate::Resettable for NdtlSpec {}
