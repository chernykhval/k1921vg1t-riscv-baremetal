#[doc = "Register `TIMERCTRL` reader"]
pub type R = crate::R<TimerctrlSpec>;
#[doc = "Register `TIMERCTRL` writer"]
pub type W = crate::W<TimerctrlSpec>;
#[doc = "Field `TPRSC` reader - Timer Prescale"]
pub type TprscR = crate::FieldReader;
#[doc = "Field `TPRSC` writer - Timer Prescale"]
pub type TprscW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Timer Prescale"]
    #[inline(always)]
    pub fn tprsc(&self) -> TprscR {
        TprscR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Prescale"]
    #[inline(always)]
    pub fn tprsc(&mut self) -> TprscW<'_, TimerctrlSpec> {
        TprscW::new(self, 0)
    }
}
#[doc = "CAN Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimerctrlSpec;
impl crate::RegisterSpec for TimerctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerctrl::R`](R) reader structure"]
impl crate::Readable for TimerctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`timerctrl::W`](W) writer structure"]
impl crate::Writable for TimerctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERCTRL to value 0"]
impl crate::Resettable for TimerctrlSpec {}
