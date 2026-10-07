#[doc = "Register `DISOSC` reader"]
pub type R = crate::R<DisoscSpec>;
#[doc = "Register `DISOSC` writer"]
pub type W = crate::W<DisoscSpec>;
#[doc = "Field `VAL` reader - Disable oscillator rings 0 to size-6"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Disable oscillator rings 0 to size-6"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Disable oscillator rings 0 to size-6"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Disable oscillator rings 0 to size-6"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, DisoscSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Disable Oscillator registers\n\nYou can [`read`](crate::Reg::read) this register and get [`disosc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disosc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisoscSpec;
impl crate::RegisterSpec for DisoscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disosc::R`](R) reader structure"]
impl crate::Readable for DisoscSpec {}
#[doc = "`write(|w| ..)` method takes [`disosc::W`](W) writer structure"]
impl crate::Writable for DisoscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISOSC to value 0"]
impl crate::Resettable for DisoscSpec {}
