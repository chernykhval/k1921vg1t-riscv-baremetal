#[doc = "Register `DATAOUTTGL` writer"]
pub type W = crate::W<DataouttglSpec>;
#[doc = "Field `VAL` writer - Data output toggle"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Data output toggle"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, DataouttglSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Data output toggle bits register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataouttgl::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataouttglSpec;
impl crate::RegisterSpec for DataouttglSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dataouttgl::W`](W) writer structure"]
impl crate::Writable for DataouttglSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAOUTTGL to value 0"]
impl crate::Resettable for DataouttglSpec {}
