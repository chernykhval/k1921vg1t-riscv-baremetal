#[doc = "Register `TESTDATA` writer"]
pub type W = crate::W<TestdataSpec>;
#[doc = "Field `VAL` writer - Test data"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Test data"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, TestdataSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Test data register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestdataSpec;
impl crate::RegisterSpec for TestdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`testdata::W`](W) writer structure"]
impl crate::Writable for TestdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TESTDATA to value 0"]
impl crate::Resettable for TestdataSpec {}
