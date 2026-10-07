#[doc = "Register `LOCKSTAT` reader"]
pub type R = crate::R<LockstatSpec>;
#[doc = "Field `REGWRDIS` reader - Write to all registers of Watchdog is disabled"]
pub type RegwrdisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write to all registers of Watchdog is disabled"]
    #[inline(always)]
    pub fn regwrdis(&self) -> RegwrdisR {
        RegwrdisR::new((self.bits & 1) != 0)
    }
}
#[doc = "Watchdog Lock Register (read)\n\nYou can [`read`](crate::Reg::read) this register and get [`lockstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockstatSpec;
impl crate::RegisterSpec for LockstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockstat::R`](R) reader structure"]
impl crate::Readable for LockstatSpec {}
#[doc = "`reset()` method sets LOCKSTAT to value 0"]
impl crate::Resettable for LockstatSpec {}
