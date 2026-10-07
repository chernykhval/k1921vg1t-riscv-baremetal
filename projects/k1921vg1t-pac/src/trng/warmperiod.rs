#[doc = "Register `WARMPERIOD` reader"]
pub type R = crate::R<WarmperiodSpec>;
#[doc = "Register `WARMPERIOD` writer"]
pub type W = crate::W<WarmperiodSpec>;
#[doc = "Field `VAL` reader - Number of clock cycles to wait during warm-up sequence"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Number of clock cycles to wait during warm-up sequence"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of clock cycles to wait during warm-up sequence"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of clock cycles to wait during warm-up sequence"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, WarmperiodSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Warmup Period register\n\nYou can [`read`](crate::Reg::read) this register and get [`warmperiod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`warmperiod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WarmperiodSpec;
impl crate::RegisterSpec for WarmperiodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`warmperiod::R`](R) reader structure"]
impl crate::Readable for WarmperiodSpec {}
#[doc = "`write(|w| ..)` method takes [`warmperiod::W`](W) writer structure"]
impl crate::Writable for WarmperiodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WARMPERIOD to value 0x0200"]
impl crate::Resettable for WarmperiodSpec {
    const RESET_VALUE: u32 = 0x0200;
}
