#[doc = "Register `LOCKCLR` writer"]
pub type W = crate::W<LockclrSpec>;
#[doc = "Field `VAL` writer - Lock configuration disable"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Lock configuration disable"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, LockclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Lock pins configuration disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockclrSpec;
impl crate::RegisterSpec for LockclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lockclr::W`](W) writer structure"]
impl crate::Writable for LockclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCKCLR to value 0"]
impl crate::Resettable for LockclrSpec {}
