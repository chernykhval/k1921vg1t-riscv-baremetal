#[doc = "Register `SEU_CNTR` reader"]
pub type R = crate::R<SeuCntrSpec>;
#[doc = "Register `SEU_CNTR` writer"]
pub type W = crate::W<SeuCntrSpec>;
#[doc = "Field `VAL` reader - Single error times occurred"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Single error times occurred"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Single error times occurred"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Single error times occurred"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, SeuCntrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "SRAM Single Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`seu_cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seu_cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeuCntrSpec;
impl crate::RegisterSpec for SeuCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seu_cntr::R`](R) reader structure"]
impl crate::Readable for SeuCntrSpec {}
#[doc = "`write(|w| ..)` method takes [`seu_cntr::W`](W) writer structure"]
impl crate::Writable for SeuCntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEU_CNTR to value 0"]
impl crate::Resettable for SeuCntrSpec {}
