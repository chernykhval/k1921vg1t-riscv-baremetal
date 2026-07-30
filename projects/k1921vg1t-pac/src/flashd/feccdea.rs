#[doc = "Register `FECCDEA` reader"]
pub type R = crate::R<FeccdeaSpec>;
#[doc = "Register `FECCDEA` writer"]
pub type W = crate::W<FeccdeaSpec>;
#[doc = "Field `VAL` reader - Last latched address when double error occurred"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Last latched address when double error occurred"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last latched address when double error occurred"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Last latched address when double error occurred"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FeccdeaSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Flash ECC Double Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccdea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccdea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeccdeaSpec;
impl crate::RegisterSpec for FeccdeaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feccdea::R`](R) reader structure"]
impl crate::Readable for FeccdeaSpec {}
#[doc = "`write(|w| ..)` method takes [`feccdea::W`](W) writer structure"]
impl crate::Writable for FeccdeaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FECCDEA to value 0"]
impl crate::Resettable for FeccdeaSpec {}
