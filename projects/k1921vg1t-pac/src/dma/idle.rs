#[doc = "Register `IDLE` reader"]
pub type R = crate::R<IdleSpec>;
#[doc = "Field `IDLE` reader - IDLE Status"]
pub type IdleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IDLE Status"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new((self.bits & 1) != 0)
    }
}
#[doc = "Idle indication register\n\nYou can [`read`](crate::Reg::read) this register and get [`idle::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdleSpec;
impl crate::RegisterSpec for IdleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idle::R`](R) reader structure"]
impl crate::Readable for IdleSpec {}
#[doc = "`reset()` method sets IDLE to value 0x01"]
impl crate::Resettable for IdleSpec {
    const RESET_VALUE: u32 = 0x01;
}
