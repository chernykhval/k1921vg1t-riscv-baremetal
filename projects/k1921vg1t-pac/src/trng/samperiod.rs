#[doc = "Register `SAMPERIOD` reader"]
pub type R = crate::R<SamperiodSpec>;
#[doc = "Register `SAMPERIOD` writer"]
pub type W = crate::W<SamperiodSpec>;
#[doc = "Field `VAL` reader - Number of clock cycles to wait between sampling moments"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Number of clock cycles to wait between sampling moments"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of clock cycles to wait between sampling moments"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of clock cycles to wait between sampling moments"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, SamperiodSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Sampling Period register\n\nYou can [`read`](crate::Reg::read) this register and get [`samperiod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samperiod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SamperiodSpec;
impl crate::RegisterSpec for SamperiodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samperiod::R`](R) reader structure"]
impl crate::Readable for SamperiodSpec {}
#[doc = "`write(|w| ..)` method takes [`samperiod::W`](W) writer structure"]
impl crate::Writable for SamperiodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMPERIOD to value 0x0fff"]
impl crate::Resettable for SamperiodSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
