#[doc = "Register `DEU_CNTR` reader"]
pub type R = crate::R<DeuCntrSpec>;
#[doc = "Register `DEU_CNTR` writer"]
pub type W = crate::W<DeuCntrSpec>;
#[doc = "Field `VAL` reader - Double error times occurred"]
pub type ValR = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - Double error times occurred"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Double error times occurred"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double error times occurred"]
    #[inline(always)]
    pub fn val(&mut self) -> ValW<'_, DeuCntrSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "SRAM Double Error Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`deu_cntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deu_cntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DeuCntrSpec;
impl crate::RegisterSpec for DeuCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`deu_cntr::R`](R) reader structure"]
impl crate::Readable for DeuCntrSpec {}
#[doc = "`write(|w| ..)` method takes [`deu_cntr::W`](W) writer structure"]
impl crate::Writable for DeuCntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEU_CNTR to value 0"]
impl crate::Resettable for DeuCntrSpec {}
