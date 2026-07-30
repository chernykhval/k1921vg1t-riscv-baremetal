#[doc = "Register `ADCSOCCLR` writer"]
pub type W = crate::W<AdcsocclrSpec>;
#[doc = "Field `VAL` writer - ADC SOC disable"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - ADC SOC disable"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, AdcsocclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "ADC Start Of Conversion disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsocclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdcsocclrSpec;
impl crate::RegisterSpec for AdcsocclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`adcsocclr::W`](W) writer structure"]
impl crate::Writable for AdcsocclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCSOCCLR to value 0"]
impl crate::Resettable for AdcsocclrSpec {}
