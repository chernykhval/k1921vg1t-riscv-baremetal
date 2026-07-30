#[doc = "Register `DATAIN` reader"]
pub type R = crate::R<DatainSpec>;
#[doc = "Register `DATAIN` writer"]
pub type W = crate::W<DatainSpec>;
#[doc = "Field `VAL` reader - Data input"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Data input"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data input"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data input"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, DatainSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`datain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatainSpec;
impl crate::RegisterSpec for DatainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datain::R`](R) reader structure"]
impl crate::Readable for DatainSpec {}
#[doc = "`write(|w| ..)` method takes [`datain::W`](W) writer structure"]
impl crate::Writable for DatainSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAIN to value 0"]
impl crate::Resettable for DatainSpec {}
