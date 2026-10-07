#[doc = "Register `DATAOUT` reader"]
pub type R = crate::R<DataoutSpec>;
#[doc = "Register `DATAOUT` writer"]
pub type W = crate::W<DataoutSpec>;
#[doc = "Field `VAL` reader - Data output"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Data output"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data output"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Data output"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, DataoutSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dataout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dataout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataoutSpec;
impl crate::RegisterSpec for DataoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dataout::R`](R) reader structure"]
impl crate::Readable for DataoutSpec {}
#[doc = "`write(|w| ..)` method takes [`dataout::W`](W) writer structure"]
impl crate::Writable for DataoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATAOUT to value 0"]
impl crate::Resettable for DataoutSpec {}
