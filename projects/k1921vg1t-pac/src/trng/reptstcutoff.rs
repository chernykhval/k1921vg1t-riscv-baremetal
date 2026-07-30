#[doc = "Register `REPTSTCUTOFF` reader"]
pub type R = crate::R<ReptstcutoffSpec>;
#[doc = "Register `REPTSTCUTOFF` writer"]
pub type W = crate::W<ReptstcutoffSpec>;
#[doc = "Field `VAL` reader - Cut-off value for the repetition tests"]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Cut-off value for the repetition tests"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Cut-off value for the repetition tests"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Cut-off value for the repetition tests"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, ReptstcutoffSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Cut-off value for the repetition tests register\n\nYou can [`read`](crate::Reg::read) this register and get [`reptstcutoff::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reptstcutoff::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReptstcutoffSpec;
impl crate::RegisterSpec for ReptstcutoffSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reptstcutoff::R`](R) reader structure"]
impl crate::Readable for ReptstcutoffSpec {}
#[doc = "`write(|w| ..)` method takes [`reptstcutoff::W`](W) writer structure"]
impl crate::Writable for ReptstcutoffSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REPTSTCUTOFF to value 0x06"]
impl crate::Resettable for ReptstcutoffSpec {
    const RESET_VALUE: u32 = 0x06;
}
