#[doc = "Register `COUNT` reader"]
pub type R = crate::R<CountSpec>;
#[doc = "Register `COUNT` writer"]
pub type W = crate::W<CountSpec>;
#[doc = "Field `VAL` reader - Current timer value"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Current timer value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current timer value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current timer value"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, CountSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Current value timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`count::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`count::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CountSpec;
impl crate::RegisterSpec for CountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`count::R`](R) reader structure"]
impl crate::Readable for CountSpec {}
#[doc = "`write(|w| ..)` method takes [`count::W`](W) writer structure"]
impl crate::Writable for CountSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COUNT to value 0"]
impl crate::Resettable for CountSpec {}
