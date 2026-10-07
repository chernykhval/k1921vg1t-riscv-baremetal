#[doc = "Register `CPSR` reader"]
pub type R = crate::R<CpsrSpec>;
#[doc = "Register `CPSR` writer"]
pub type W = crate::W<CpsrSpec>;
#[doc = "Field `CPSDVSR` reader - Clock division factor. Bit0 always 0"]
pub type CpsdvsrR = crate::FieldReader;
#[doc = "Field `CPSDVSR` writer - Clock division factor. Bit0 always 0"]
pub type CpsdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock division factor. Bit0 always 0"]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CpsdvsrR {
        CpsdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock division factor. Bit0 always 0"]
    #[inline(always)]
    pub fn cpsdvsr(&mut self) -> CpsdvsrW<'_, CpsrSpec> {
        CpsdvsrW::new(self, 0)
    }
}
#[doc = "Clock division factor register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsrSpec;
impl crate::RegisterSpec for CpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr::R`](R) reader structure"]
impl crate::Readable for CpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsr::W`](W) writer structure"]
impl crate::Writable for CpsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CpsrSpec {}
