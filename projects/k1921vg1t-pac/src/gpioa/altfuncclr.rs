#[doc = "Register `ALTFUNCCLR` writer"]
pub type W = crate::W<AltfuncclrSpec>;
#[doc = "Field `VAL` writer - Alternative function disable"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Alternative function disable"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, AltfuncclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Alternative function disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`altfuncclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltfuncclrSpec;
impl crate::RegisterSpec for AltfuncclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`altfuncclr::W`](W) writer structure"]
impl crate::Writable for AltfuncclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ALTFUNCCLR to value 0"]
impl crate::Resettable for AltfuncclrSpec {}
