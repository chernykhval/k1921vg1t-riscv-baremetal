#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Enable write access to all other registers by writing LOCK = 0x1ACCE551. Disable write to any othr registers of Watchdog by writing any other value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Code {
    #[doc = "449635665: Value that clear LOCKSTAT.REGWRDIS bit"]
    Unlock = 449635665,
    #[doc = "0: Example value that set LOCKSTAT.REGWRDIS (write to all registers except LOCK is disabled)"]
    Lock0 = 0,
    #[doc = "1: Example value that set LOCKSTAT.REGWRDIS (write to all registers except LOCK is disabled)"]
    Lock1 = 1,
}
impl From<Code> for u32 {
    #[inline(always)]
    fn from(variant: Code) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Code {
    type Ux = u32;
}
impl crate::IsEnum for Code {}
#[doc = "Field `CODE` writer - Enable write access to all other registers by writing LOCK = 0x1ACCE551. Disable write to any othr registers of Watchdog by writing any other value"]
pub type CodeW<'a, REG> = crate::FieldWriter<'a, REG, 32, Code>;
impl<'a, REG> CodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Value that clear LOCKSTAT.REGWRDIS bit"]
    #[inline(always)]
    pub fn unlock(self) -> &'a mut crate::W<REG> {
        self.variant(Code::Unlock)
    }
    #[doc = "Example value that set LOCKSTAT.REGWRDIS (write to all registers except LOCK is disabled)"]
    #[inline(always)]
    pub fn lock_0(self) -> &'a mut crate::W<REG> {
        self.variant(Code::Lock0)
    }
    #[doc = "Example value that set LOCKSTAT.REGWRDIS (write to all registers except LOCK is disabled)"]
    #[inline(always)]
    pub fn lock_1(self) -> &'a mut crate::W<REG> {
        self.variant(Code::Lock1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable write access to all other registers by writing LOCK = 0x1ACCE551. Disable write to any othr registers of Watchdog by writing any other value"]
    #[inline(always)]
    pub fn code(&mut self) -> CodeW<'_, LockSpec> {
        CodeW::new(self, 0)
    }
}
#[doc = "Watchdog Lock Register (write)\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}
