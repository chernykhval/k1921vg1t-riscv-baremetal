#[doc = "Register `DATAOUTCLR` writer"]
pub type W = crate::W<DataoutclrSpec>;
#[doc = "Field `VAL` writer - Data output clear"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data output clear"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, DataoutclrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Data output clear bits register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataoutclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoutclrSpec;
impl crate::RegisterSpec for DataoutclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dataoutclr::W`](W) writer structure"]
impl crate::Writable for DataoutclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAOUTCLR to value 0"]
impl crate::Resettable for DataoutclrSpec {}
