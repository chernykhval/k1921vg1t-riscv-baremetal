#[doc = "Register `ERRMASK` reader"]
pub type R = crate::R<ErrmaskSpec>;
#[doc = "Register `ERRMASK` writer"]
pub type W = crate::W<ErrmaskSpec>;
#[doc = "Field `VAL` reader - Corresponding bit of 32-bit write data bus is inverted"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Corresponding bit of 32-bit write data bus is inverted"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Corresponding bit of 32-bit write data bus is inverted"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Corresponding bit of 32-bit write data bus is inverted"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, ErrmaskSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "SRAM Data Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`errmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`errmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrmaskSpec;
impl crate::RegisterSpec for ErrmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`errmask::R`](R) reader structure"]
impl crate::Readable for ErrmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`errmask::W`](W) writer structure"]
impl crate::Writable for ErrmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERRMASK to value 0"]
impl crate::Resettable for ErrmaskSpec {}
