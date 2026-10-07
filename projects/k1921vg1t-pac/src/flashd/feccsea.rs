#[doc = "Register `FECCSEA` reader"]
pub type R = crate::R<FeccseaSpec>;
#[doc = "Register `FECCSEA` writer"]
pub type W = crate::W<FeccseaSpec>;
#[doc = "Field `VAL` reader - Last latched address when single error occurred"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Last latched address when single error occurred"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Last latched address when single error occurred"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Last latched address when single error occurred"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, FeccseaSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Flash ECC Single Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`feccsea::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feccsea::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FeccseaSpec;
impl crate::RegisterSpec for FeccseaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`feccsea::R`](R) reader structure"]
impl crate::Readable for FeccseaSpec {}
#[doc = "`write(|w| ..)` method takes [`feccsea::W`](W) writer structure"]
impl crate::Writable for FeccseaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FECCSEA to value 0"]
impl crate::Resettable for FeccseaSpec {}
