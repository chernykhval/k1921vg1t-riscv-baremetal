#[doc = "Register `TERMINATE` writer"]
pub type W = crate::W<TerminateSpec>;
#[doc = "Field `CODE` writer - "]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn code(&mut self) -> CodeW<'_, TerminateSpec> {
        CodeW::new(self, 0)
    }
}
#[doc = "Operation emergency terminate register (write of 0xD0 terminate current oeparation)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`terminate::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TerminateSpec;
impl crate::RegisterSpec for TerminateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`terminate::W`](W) writer structure"]
impl crate::Writable for TerminateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TERMINATE to value 0"]
impl crate::Resettable for TerminateSpec {}
