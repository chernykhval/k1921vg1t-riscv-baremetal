#[doc = "Register `VALUE` reader"]
pub type R = crate::R<ValueSpec>;
#[doc = "Register `VALUE` writer"]
pub type W = crate::W<ValueSpec>;
#[doc = "Field `OFFTRIM` reader - ADC channel offset trimm value"]
pub type OfftrimR = crate::FieldReader<u16>;
#[doc = "Field `OFFTRIM` writer - ADC channel offset trimm value"]
pub type OfftrimW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `GAINTRIM` reader - ADC channel gain trimm value"]
pub type GaintrimR = crate::FieldReader<u16>;
#[doc = "Field `GAINTRIM` writer - ADC channel gain trimm value"]
pub type GaintrimW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `PRIORITY` reader - ADC channel priority level"]
pub type PriorityR = crate::BitReader;
#[doc = "Field `PRIORITY` writer - ADC channel priority level"]
pub type PriorityW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - ADC channel offset trimm value"]
    #[inline(always)]
    pub fn offtrim(&self) -> OfftrimR {
        OfftrimR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - ADC channel gain trimm value"]
    #[inline(always)]
    pub fn gaintrim(&self) -> GaintrimR {
        GaintrimR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 28 - ADC channel priority level"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - ADC channel offset trimm value"]
    #[inline(always)]
    pub fn offtrim(&mut self) -> OfftrimW<'_, ValueSpec> {
        OfftrimW::new(self, 0)
    }
    #[doc = "Bits 16:24 - ADC channel gain trimm value"]
    #[inline(always)]
    pub fn gaintrim(&mut self) -> GaintrimW<'_, ValueSpec> {
        GaintrimW::new(self, 16)
    }
    #[doc = "Bit 28 - ADC channel priority level"]
    #[inline(always)]
    pub fn priority(&mut self) -> PriorityW<'_, ValueSpec> {
        PriorityW::new(self, 28)
    }
}
#[doc = "ADC channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValueSpec;
impl crate::RegisterSpec for ValueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for ValueSpec {}
#[doc = "`write(|w| ..)` method takes [`value::W`](W) writer structure"]
impl crate::Writable for ValueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for ValueSpec {}
