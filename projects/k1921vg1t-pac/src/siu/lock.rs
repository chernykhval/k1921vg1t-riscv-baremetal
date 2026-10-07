#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Field `LOCK` reader - Write with specific code to lock / unlock SIU registers configuration"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Write with specific code to lock / unlock SIU registers configuration"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CODE` writer - Write key 0xACCE55 to change LOCK bit value"]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - Write with specific code to lock / unlock SIU registers configuration"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write with specific code to lock / unlock SIU registers configuration"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, LockSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write key 0xACCE55 to change LOCK bit value"]
    #[inline(always)]
    pub fn code(&mut self) -> CodeW<'_, LockSpec> {
        CodeW::new(self, 8)
    }
}
#[doc = "Lock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}
