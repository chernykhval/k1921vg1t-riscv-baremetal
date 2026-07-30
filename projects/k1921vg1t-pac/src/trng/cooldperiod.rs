#[doc = "Register `COOLDPERIOD` reader"]
pub type R = crate::R<CooldperiodSpec>;
#[doc = "Register `COOLDPERIOD` writer"]
pub type W = crate::W<CooldperiodSpec>;
#[doc = "Field `VAL` reader - Number of clock cycles to wait during cool-down sequence"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Number of clock cycles to wait during cool-down sequence"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Number of clock cycles to wait during cool-down sequence"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Number of clock cycles to wait during cool-down sequence"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, CooldperiodSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Cooldown Period register\n\nYou can [`read`](crate::Reg::read) this register and get [`cooldperiod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cooldperiod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CooldperiodSpec;
impl crate::RegisterSpec for CooldperiodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cooldperiod::R`](R) reader structure"]
impl crate::Readable for CooldperiodSpec {}
#[doc = "`write(|w| ..)` method takes [`cooldperiod::W`](W) writer structure"]
impl crate::Writable for CooldperiodSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COOLDPERIOD to value 0"]
impl crate::Resettable for CooldperiodSpec {}
