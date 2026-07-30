#[doc = "Register `TIMERVAL` reader"]
pub type R = crate::R<TimervalSpec>;
#[doc = "Field `CANTIM` reader - CAN Timer Value"]
pub type CantimR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - CAN Timer Value"]
    #[inline(always)]
    pub fn cantim(&self) -> CantimR {
        CantimR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "CAN Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerval::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimervalSpec;
impl crate::RegisterSpec for TimervalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerval::R`](R) reader structure"]
impl crate::Readable for TimervalSpec {}
#[doc = "`reset()` method sets TIMERVAL to value 0"]
impl crate::Resettable for TimervalSpec {}
