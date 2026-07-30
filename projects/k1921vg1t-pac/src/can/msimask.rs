#[doc = "Register `MSIMASK` reader"]
pub type R = crate::R<MsimaskSpec>;
#[doc = "Register `MSIMASK` writer"]
pub type W = crate::W<MsimaskSpec>;
#[doc = "Field `IM` reader - Mask for waiting bit messages"]
pub type ImR = crate::FieldReader<u32>;
#[doc = "Field `IM` writer - Mask for waiting bit messages"]
pub type ImW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask for waiting bit messages"]
    #[inline(always)]
    pub fn im(&self) -> ImR {
        ImR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask for waiting bit messages"]
    #[inline(always)]
    pub fn im(&mut self) -> ImW<'_, MsimaskSpec> {
        ImW::new(self, 0)
    }
}
#[doc = "Mask register message index\n\nYou can [`read`](crate::Reg::read) this register and get [`msimask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msimask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsimaskSpec;
impl crate::RegisterSpec for MsimaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msimask::R`](R) reader structure"]
impl crate::Readable for MsimaskSpec {}
#[doc = "`write(|w| ..)` method takes [`msimask::W`](W) writer structure"]
impl crate::Writable for MsimaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSIMASK to value 0"]
impl crate::Resettable for MsimaskSpec {}
