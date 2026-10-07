#[doc = "Register `MITR` writer"]
pub type W = crate::W<MitrSpec>;
#[doc = "Field `IT` writer - Field generate interrupt"]
pub type ItW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Field generate interrupt"]
    #[inline(always)]
    pub fn it(&mut self) -> ItW<'_, MitrSpec> {
        ItW::new(self, 0)
    }
}
#[doc = "Interrupt register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mitr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MitrSpec;
impl crate::RegisterSpec for MitrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mitr::W`](W) writer structure"]
impl crate::Writable for MitrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MITR to value 0"]
impl crate::Resettable for MitrSpec {}
